// Copied from:
// https://github.com/stm32-rs/stm32-eth/blob/master/examples/pktgen.rs

#![no_main]
#![no_std]

extern crate stm32f4xx_hal as hal;

#[allow(unused_imports)]
use panic_semihosting;

use crate::hal::{prelude::*, serial::config::Config, serial::Serial, stm32, stm32::interrupt};
use core::cell::RefCell;
use core::fmt::Write;
use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32_eth::{Eth, RingEntry};

const SRC_MAC: [u8; 6] = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
const DST_MAC: [u8; 6] = [0x00, 0x00, 0xBE, 0xEF, 0xDE, 0xAD];
const ETH_TYPE: [u8; 2] = [0x80, 0x00];

static TIME: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
static ETH_PENDING: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("Failed to take stm32::Peripherals");
    let mut cp =
        cortex_m::peripheral::Peripherals::take().expect("Failed to take cortex_m::Peripherals");

    setup_systick(&mut cp.SYST);
    stm32_eth::setup(&dp.RCC, &dp.SYSCFG);

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    // Setup USART3
    let gpiod = dp.GPIOD.split();
    let pin_tx = gpiod.pd8.into_alternate_af7();
    let pin_rx = gpiod.pd9.into_alternate_af7();

    let serial = Serial::usart3(
        dp.USART3,
        (pin_tx, pin_rx),
        Config {
            baudrate: 115_200.bps(),
            ..Default::default()
        },
        clocks,
    )
    .unwrap();

    let (mut stdout, _rx) = serial.split();

    writeln!(stdout, "Initializing").unwrap();

    if !stm32::SYST::is_precise() {
        writeln!(
            stdout,
            "Warning: SYSTICK with source {:?} is not precise",
            cp.SYST.get_clock_source()
        )
        .unwrap();
    }

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiog = dp.GPIOG.split();
    stm32_eth::setup_pins(
        gpioa.pa1, gpioa.pa2, gpioa.pa7, gpiob.pb13, gpioc.pc1, gpioc.pc4, gpioc.pc5, gpiog.pg11,
        gpiog.pg13,
    );

    let mut rx_ring: [RingEntry<_>; 16] = Default::default();
    let mut tx_ring: [RingEntry<_>; 8] = Default::default();
    let mut eth = Eth::new(
        dp.ETHERNET_MAC,
        dp.ETHERNET_DMA,
        SRC_MAC,
        &mut rx_ring[..],
        &mut tx_ring[..],
    );
    eth.enable_interrupt(&mut cp.NVIC);

    // Main loop
    let mut last_stats_time = 0usize;
    let mut rx_bytes = 0usize;
    let mut rx_pkts = 0usize;
    let mut tx_bytes = 0usize;
    let mut tx_pkts = 0usize;
    let mut last_status = None;

    writeln!(stdout, "Starting").unwrap();

    loop {
        let time: usize = cortex_m::interrupt::free(|cs| *TIME.borrow(cs).borrow());

        // Print stats every 30 seconds
        if time >= last_stats_time + 30 {
            let t = time - last_stats_time;
            writeln!(
                stdout,
                "T={}\tRx:\t{} KB/s\t{} pps\tTx:\t{} KB/s\t{} pps",
                time,
                rx_bytes / 1024 / t,
                rx_pkts / t,
                tx_bytes / 1024 / t,
                tx_pkts / t
            )
            .unwrap();

            // Reset
            rx_bytes = 0;
            rx_pkts = 0;
            tx_bytes = 0;
            tx_pkts = 0;
            last_stats_time = time;
        }

        // Link change detection
        let status = eth.status();
        if last_status
            .map(|last_status| last_status != status)
            .unwrap_or(true)
        {
            if !status.link_detected() {
                writeln!(stdout, "Ethernet: no link detected").unwrap();
            } else {
                writeln!(
                    stdout,
                    "Ethernet: link detected with {} Mbps/{}",
                    status.speed(),
                    match status.is_full_duplex() {
                        Some(true) => "FD",
                        Some(false) => "HD",
                        None => "?",
                    }
                )
                .unwrap();
            }

            last_status = Some(status);
        }

        cortex_m::interrupt::free(|cs| {
            let mut eth_pending = ETH_PENDING.borrow(cs).borrow_mut();
            *eth_pending = false;
        });

        // Handle rx packet
        {
            let mut recvd = 0usize;
            while let Ok(pkt) = eth.recv_next() {
                rx_bytes += pkt.len();
                rx_pkts += 1;
                pkt.free();

                recvd += 1;
                if recvd > 16 {
                    // Break arbitrarily to process tx eventually
                    break;
                }
            }
        }
        if !eth.rx_is_running() {
            writeln!(stdout, "RX stopped").unwrap();
        }

        // Fill tx queue
        const SIZE: usize = 1500;
        if status.link_detected() {
            let r = eth.send(SIZE, |buf| {
                buf[0..6].copy_from_slice(&DST_MAC);
                buf[6..12].copy_from_slice(&SRC_MAC);
                buf[12..14].copy_from_slice(&ETH_TYPE);
            });

            match r {
                Ok(()) => {
                    tx_bytes += SIZE;
                    tx_pkts += 1;
                }
                _ => (),
            }
        }

        cortex_m::interrupt::free(|cs| {
            let eth_pending = ETH_PENDING.borrow(cs).borrow_mut();
            if !*eth_pending {
                asm::wfi();
            }
        });
    }
}

fn setup_systick(syst: &mut stm32::SYST) {
    syst.set_reload(100 * stm32::SYST::get_ticks_per_10ms());
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    //if !stm32::SYST::is_precise() {
    //    panic!(
    //        "Warning: SYSTICK with source {:?} is not precise",
    //        syst.get_clock_source()
    //    )
    //}
}

#[exception]
fn SysTick() {
    cortex_m::interrupt::free(|cs| {
        let mut time = TIME.borrow(cs).borrow_mut();
        *time += 1;
    })
}

#[interrupt]
fn ETH() {
    cortex_m::interrupt::free(|cs| {
        let mut eth_pending = ETH_PENDING.borrow(cs).borrow_mut();
        *eth_pending = true;
    });

    // Clear interrupt flags
    let p = unsafe { stm32::Peripherals::steal() };
    stm32_eth::eth_interrupt_handler(&p.ETHERNET_DMA);
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
