#![no_std]
#![deny(unsafe_code)]

extern crate stm32f4xx_hal as hal;

use crate::hal::{i2c, i2s};

const WM8960_ADDRESS: u8 = 0x1A;

#[derive(Debug)]
pub enum Error {
    I2c(i2c::Error),
    I2s(i2s::Error),
}

pub struct Wm8960<I2C, I2S> {
    i2c: I2C,
    i2s: I2S,
}

impl<I2C, I2S> Wm8960<I2C, I2S>
where
    I2C: embedded_hal::blocking::i2c::Write<Error = i2c::Error>,
    I2S: i2s::Write<u16, Error = i2s::Error>,
{
    pub fn new(i2c: I2C, i2s: I2S) -> Result<Self, Error> {
        let mut wm = Wm8960 { i2c, i2s };

        // Reset
        wm.write_reg(0x0F, 0x0000)?;

        // Set power source
        wm.write_reg(0x19, 1 << 8 | 1 << 7 | 1 << 6)?;
        wm.write_reg(0x1A, 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3)?;
        wm.write_reg(0x2F, 1 << 3 | 1 << 2)?;

        // Configure clock
        // MCLK->div1->SYSCLK->DAC/ADC sample Freq
        // = 25MHz(MCLK)/2*256 = 48.8kHz
        wm.write_reg(0x04, 0x0000)?;

        // Configure ADC/DAC
        wm.write_reg(0x05, 0x0000)?;

        // Configure audio interface
        // I2S format 16 bits word length
        wm.write_reg(0x07, 0x0002)?;

        // Configure HP_L and HP_R OUTPUTS
        wm.write_reg(0x02, 0x006F | 0x0100)?; // LOUT1 Volume Set
        wm.write_reg(0x03, 0x006F | 0x0100)?; // ROUT1 Volume Set

        // Configure SPK_RP and SPK_RN
        wm.write_reg(0x28, 0x007F | 0x0100)?; // Left Speaker Volume
        wm.write_reg(0x29, 0x007F | 0x0100)?; // Right Speaker Volume

        // Enable the OUTPUTS
        wm.write_reg(0x31, 0x00F7)?; // Enable Class D Speaker Outputs

        // Configure DAC volume
        wm.write_reg(0x0a, 0x00FF | 0x0100)?;
        wm.write_reg(0x0b, 0x00FF | 0x0100)?;

        // 3D
        // wm.write_reg(0x10, 0x001F);

        // Configure MIXER
        wm.write_reg(0x22, 1 << 8 | 1 << 7)?;
        wm.write_reg(0x25, 1 << 8 | 1 << 7)?;

        // Jack Detect
        wm.write_reg(0x18, 1 << 6 | 0 << 5)?;
        wm.write_reg(0x17, 0x01C3)?;
        wm.write_reg(0x30, 0x0009)?;

        Ok(wm)
    }

    pub fn play_audio(&mut self, data: &[u16]) -> Result<(), Error> {
        self.i2s.write(data)?;
        Ok(())
    }

    // 9-bit registers
    fn write_reg(&mut self, reg: u8, data: u16) -> Result<(), Error> {
        let data = [(reg << 1) | (data >> 8) as u8 & 0x1, (data & 0xFF) as u8];
        self.i2c.write(WM8960_ADDRESS, &data)?;
        Ok(())
    }
}

impl From<i2c::Error> for Error {
    fn from(e: i2c::Error) -> Self {
        Error::I2c(e)
    }
}

impl From<i2s::Error> for Error {
    fn from(e: i2s::Error) -> Self {
        Error::I2s(e)
    }
}
