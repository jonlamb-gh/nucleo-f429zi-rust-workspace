#![no_main]
#![no_std]

extern crate stm32f4xx_hal as hal;

#[allow(unused_imports)]
use panic_semihosting;

use crate::hal::{
    i2c::I2c,
    i2s::{I2s, I2sStandard},
    prelude::*,
    serial::config::Config,
    serial::Serial,
    stm32,
};
use crate::wave_data::WAVE_DATA;
use core::fmt::Write;
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use wm8960::wave_header::parse_header;
use wm8960::Wm8960;

mod wave_data;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("Failed to take stm32::Peripherals");
    let cp =
        cortex_m::peripheral::Peripherals::take().expect("Failed to take cortex_m::Peripherals");

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();

    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();

    let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
    let sda = gpiob.pb9.into_alternate_af4().set_open_drain();

    let i2s_ck = gpiob.pb13.into_alternate_af5();
    let i2s_ws = gpiob.pb12.into_alternate_af5();
    let i2s_sd = gpiob.pb15.into_alternate_af5();
    let i2s_mck = gpioc.pc6.into_alternate_af5();

    let serial_tx = gpiod.pd8.into_alternate_af7();
    let serial_rx = gpiod.pd9.into_alternate_af7();

    let serial = Serial::usart3(
        dp.USART3,
        (serial_tx, serial_rx),
        Config {
            baudrate: 115_200.bps(),
            ..Default::default()
        },
        clocks,
    )
    .unwrap();
    let (mut stdout, _rx) = serial.split();

    writeln!(stdout, "Clocks",).unwrap();
    writeln!(stdout, "  hclk  : {}", clocks.hclk().0).unwrap();
    writeln!(stdout, "  pclk1 : {}", clocks.pclk1().0).unwrap();
    writeln!(stdout, "  pclk2 : {}", clocks.pclk2().0).unwrap();
    writeln!(stdout, "  ppre1 : {}", clocks.ppre1()).unwrap();
    writeln!(stdout, "  ppre2 : {}", clocks.ppre2()).unwrap();
    writeln!(stdout, "  sysclk: {}", clocks.sysclk().0).unwrap();

    writeln!(stdout, "Init I2C").unwrap();

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks);

    writeln!(stdout, "Init I2S").unwrap();

    let i2s = I2s::i2s2(dp.SPI2, (i2s_sd, i2s_ck, i2s_ws, i2s_mck), clocks)
        .into_master_output::<u16>(I2sStandard::Philips);

    writeln!(stdout, "Init Wm8960").unwrap();

    let mut wm8960 = Wm8960::new(i2c, i2s).unwrap();

    writeln!(stdout, "Init Wm8960").unwrap();

    writeln!(stdout, "WAVE_DATA ([u16]) len: {}", WAVE_DATA.len()).unwrap();

    let (head, input, tail) = unsafe { WAVE_DATA.align_to::<u8>() };

    assert!(head.is_empty());
    assert!(tail.is_empty());
    assert_eq!(input.len(), WAVE_DATA.len() * 2);

    writeln!(stdout, "WAVE_DATA ([u8]) len: {}", input.len()).unwrap();

    let (_input, header) = parse_header(input).map_err(|_| unimplemented!()).unwrap();
    writeln!(stdout, "{:#?}", header).unwrap();

    assert_eq!(header.riff.chunk_size as usize, input.len() - 8);
    let data_offset = header.data_offset();

    writeln!(stdout, "data_offset: {}", data_offset).unwrap();
    assert_eq!(header.data.chunk_size as usize, input.len() - data_offset);

    loop {
        writeln!(stdout, "Playing").unwrap();

        wm8960.play_audio(&WAVE_DATA[data_offset..]).unwrap();

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
