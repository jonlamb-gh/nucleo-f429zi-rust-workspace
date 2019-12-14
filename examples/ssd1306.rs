#![no_main]
#![no_std]
#![deny(unsafe_code)]

extern crate stm32f4xx_hal as hal;

#[allow(unused_imports)]
use panic_semihosting;

use crate::hal::{i2c::I2c, prelude::*, stm32};
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_graphics::{fonts::Font6x8, prelude::*};
use ssd1306::{prelude::*, Builder as SSD1306Builder};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("Failed to take stm32::Peripherals");
    let _cp =
        cortex_m::peripheral::Peripherals::take().expect("Failed to take cortex_m::Peripherals");

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    // Set up I2C - SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
    // as per the STM32F446xC/E datasheet page 60. Pin assignment as per the Nucleo-F446 board.
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
    let sda = gpiob.pb9.into_alternate_af4().set_open_drain();
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

    // There's a button on PC13. On the Nucleo board, it's pulled up by a 4.7kOhm resistor
    // and therefore is active LOW. There's even a 100nF capacitor for debouncing - nice for us
    // since otherwise we'd have to debounce in software.
    let gpioc = dp.GPIOC.split();
    let btn = gpioc.pc13.into_pull_down_input();

    // Set up the display
    let mut disp: GraphicsMode<_> = SSD1306Builder::new()
        .size(DisplaySize::Display128x32)
        .with_rotation(DisplayRotation::Rotate180)
        .connect_i2c(i2c)
        .into();
    disp.init().unwrap();
    disp.flush().unwrap();

    disp.draw(
        Font6x8::render_str("Waiting")
            .translate(Point::new(0, 16))
            .into_iter(),
    );
    disp.flush().unwrap();

    // Set up state for the loop
    let mut was_pressed = btn.is_low().unwrap();

    // This runs continuously, as fast as possible
    loop {
        // Check if the button has just been pressed.
        // Remember, active low.
        let is_pressed = btn.is_low().unwrap();
        if !was_pressed && is_pressed {
            disp.draw(
                Font6x8::render_str("Pressed")
                    .translate(Point::new(0, 0))
                    .into_iter(),
            );
            disp.flush().unwrap();
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
