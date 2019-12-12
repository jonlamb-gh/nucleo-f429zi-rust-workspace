#![no_main]
#![no_std]
#![deny(unsafe_code)]

extern crate stm32f4xx_hal as hal;

#[allow(unused_imports)]
use panic_halt;

use crate::hal::{prelude::*, serial::config::Config, serial::Serial, stm32};
use core::fmt::Write;
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("Failed to take stm32::Peripherals");
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

    loop {
        writeln!(tx, "Hello world").unwrap();
        delay.delay_ms(1000_u32);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
