#![no_main]
#![no_std]

extern crate stm32f4xx_hal as hal;

#[allow(unused_imports)]
use panic_semihosting;

use crate::hal::dma::config::{DmaConfig, TransferSize};
use crate::hal::dma::{Channel0, DmaStream, MemoryToMemory, Stream0};
use crate::hal::prelude::*;
use crate::hal::serial::{config::Config, Serial};
use crate::hal::stm32::{self, interrupt, Interrupt, NVIC};
use core::cell::RefCell;
use core::fmt::Write;
use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};

static DMA_DONE: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

static mut DMA_SRC_BUFFER: [u16; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
static mut DMA_DST_BUFFER: [u16; 8] = [0; 8];

#[entry]
fn main() -> ! {
    let mut dp = stm32::Peripherals::take().expect("Failed to take stm32::Peripherals");
    let cp =
        cortex_m::peripheral::Peripherals::take().expect("Failed to take cortex_m::Peripherals");

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

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

    let (mut tx, _rx) = serial.split();

    writeln!(tx, "DMA mem2mem example").unwrap();

    let num_transfers = unsafe {
        assert_eq!(DMA_SRC_BUFFER.len(), DMA_DST_BUFFER.len());
        assert_ne!(DMA_SRC_BUFFER, DMA_DST_BUFFER);
        DMA_SRC_BUFFER.len() as u16
    };

    writeln!(tx, "Enable DMA2_STREAM0 interrupt").unwrap();

    let interrupt = Interrupt::DMA2_STREAM0;
    NVIC::unpend(interrupt);
    unsafe {
        NVIC::unmask(interrupt);
    }

    let dma_config = DmaConfig::default()
        .memory_size(TransferSize::HalfWord)
        .peripheral_size(TransferSize::HalfWord)
        .number_of_transfers(num_transfers)
        .memory_increment(true)
        .peripheral_increment(true)
        .fifo_enable(true)
        .circular(false)
        .transfer_complete_interrupt(true)
        .double_buffer(false);

    writeln!(tx, "DMA config\n{:#?}", dma_config).unwrap();

    let _dma_stream = unsafe {
        DmaStream::<stm32::DMA2, Stream0<stm32::DMA2>, Channel0, MemoryToMemory, MemoryToMemory>::init(
            &mut dp.DMA2,
            &MemoryToMemory,
            &DMA_DST_BUFFER as &[u16],
            // Double buffer is the source in mem2mem mode
            Some(&DMA_SRC_BUFFER as &[u16]),
            dma_config,
        )
    };

    loop {
        delay.delay_ms(1_u32);

        let dma_done = cortex_m::interrupt::free(|cs| *DMA_DONE.borrow(cs).borrow());
        if dma_done {
            writeln!(tx, "All done").unwrap();

            unsafe {
                writeln!(tx, "DMA_SRC_BUFFER: {:?}", DMA_SRC_BUFFER).unwrap();
                writeln!(tx, "DMA_DST_BUFFER: {:?}", DMA_DST_BUFFER).unwrap();
                assert_eq!(DMA_SRC_BUFFER, DMA_DST_BUFFER);
            }

            loop {
                asm::wfi();
            }
        }
    }
}

#[interrupt]
fn DMA2_STREAM0() {
    unsafe {
        Stream0::<stm32::DMA2>::clear_interrupts_unsafe();
    }

    cortex_m::interrupt::free(|cs| {
        let mut dma_done = DMA_DONE.borrow(cs).borrow_mut();
        *dma_done = true;
    });
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
