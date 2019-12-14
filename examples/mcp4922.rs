#![no_main]
#![no_std]
#![deny(unsafe_code)]

extern crate stm32f4xx_hal as hal;

#[allow(unused_imports)]
use panic_semihosting;

use crate::hal::{prelude::*, spi::Spi, stm32};
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_hal::digital::v1_compat::OldOutputPin;
use mcp49xx::{Command, Mcp49xx, MODE0};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("Failed to take stm32::Peripherals");
    let _cp =
        cortex_m::peripheral::Peripherals::take().expect("Failed to take cortex_m::Peripherals");

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    // Set up SPI
    let gpioa = dp.GPIOA.split();
    let gpiod = dp.GPIOD.split();
    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();
    let mut cs = gpiod.pd14.into_push_pull_output();

    // Deselect
    cs.set_high().unwrap();

    let gpioc = dp.GPIOC.split();
    let btn = gpioc.pc13.into_pull_down_input();

    let spi = Spi::spi1(dp.SPI1, (sck, miso, mosi), MODE0, 1.mhz().into(), clocks);

    let mut mcp4922 = Mcp49xx::new_mcp4922(spi, OldOutputPin::from(cs));

    // Set up state for the loop
    let mut was_pressed = btn.is_low().unwrap();

    let cmd = Command::default();
    let cmd = cmd.double_gain().value(50);

    // This runs continuously, as fast as possible
    loop {
        // Check if the button has just been pressed.
        // Remember, active low.
        let is_pressed = btn.is_low().unwrap();
        if !was_pressed && is_pressed {
            // Enable double gain and set value
            mcp4922.send(cmd).unwrap();

            // Keeps double gain enabled but changes value
            mcp4922.send(cmd.value(100)).unwrap();

            was_pressed = true;
        } else if !is_pressed {
            was_pressed = false;
        }
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
