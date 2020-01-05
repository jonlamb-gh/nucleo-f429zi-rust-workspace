#![no_std]
#![deny(unsafe_code)]

extern crate stm32f4xx_hal as hal;

use crate::hal::{i2c, i2s};
use crate::register::*;

mod register;
pub mod wave_header;

const DEVICE_ADDRESS: u8 = 0x1A;

#[derive(Debug)]
pub enum Error {
    I2c(i2c::Error),
    I2s(i2s::Error),
    InvalidInputData,
}

pub struct Wm8960<I2C, I2S> {
    i2c: I2C,
    i2s: I2S,
}

impl<I2C, I2S> Wm8960<I2C, I2S>
where
    I2C: embedded_hal::blocking::i2c::Write<Error = i2c::Error>,
    I2S: i2s::Write<u16, Error = i2s::Error> + i2s::Read<u16, Error = i2s::Error>,
{
    pub fn new(i2c: I2C, i2s: I2S) -> Result<Self, Error> {
        let mut wm = Wm8960 { i2c, i2s };

        wm.write_control_register(Register::Reset, 0xFFFF)?;
        let reg_vals = [
            (0x19, 0xE8),
            (0x1A, 0x1F8),
            (0x2F, 0x3C),
            (0x4, 0x0),
            (0x7, 0x2),
            (0x0, 0x13F),
            (0x1, 0x13F),
            (0x20, 0x108),
            (0x21, 0x0),
            (0x2B, 0x0),
            (0x2C, 0x0),
            (0x5, 0xC),
            (0x15, 0x1C3),
            (0x16, 0x1C3),
            (0x17, 0x1C4),
            (0x14, 0xF9),
            (0xA, 0x100),
            (0xB, 0x100),
            (0x5, 0x0),
            (0x6, 0x0),
            (0x10, 0x0),
            (0x31, 0xF7),
        ];

        for (reg, val) in &reg_vals {
            wm.write_reg(*reg, *val)?;
        }
        return Ok(wm);

        // Reset
        wm.write_control_register(Register::Reset, 0)?;

        // Set power source
        let mut val = PwrMgmt1(0);
        val.set_vref(true);
        val.set_vmidsel(0b11);
        val.set_adcl(true);
        val.set_ainl(true);
        wm.write_control_register(Register::PwrMgmt1, val.0)?;
        let mut val = PwrMgmt2(0);
        val.set_spkr(true);
        val.set_spkl(true);
        val.set_rout1(true);
        val.set_lout1(true);
        val.set_dacr(true);
        val.set_dacl(true);
        wm.write_control_register(Register::PwrMgmt2, val.0)?;
        let mut val = PwrMgmt3(0);
        val.set_romix(true);
        val.set_lomix(true);
        val.set_rmic(true);
        val.set_lmic(true);
        wm.write_control_register(Register::PwrMgmt3, val.0)?;

        // Configure clock
        // MCLK->div1->SYSCLK->DAC/ADC sample Freq
        // = 25MHz(MCLK)/2*256 = 48.8kHz
        let val = Clocking(0);
        wm.write_control_register(Register::Clocking, val.0)?;

        // Configure ADC/DAC
        let val = Ctr1(0);
        //val.set_deemph(0b11);
        wm.write_control_register(Register::Ctr1, val.0)?;

        // Configure audio interface
        // I2S format 16 bits word length
        let mut val = AudioIface(0);
        val.set_format(0b10);
        wm.write_control_register(Register::AudioIface, val.0)?;

        // Input PGA
        let mut val = LeftInputVol(0);
        val.set_linvol(0x3F);
        val.set_ipvu(true);
        wm.write_control_register(Register::LeftInputVol, val.0)?;
        let mut val = RightInputVol(0);
        val.set_rinvol(0x3F);
        val.set_ipvu(true);
        wm.write_control_register(Register::RightInputVol, val.0)?;

        // Input signal path
        // TODO - board mic vs earphone mic, assume earphone
        let val = LadcSignalPath(0);
        wm.write_control_register(Register::LadcSignalPath, val.0)?;
        let mut val = RadcSignalPath(0);
        val.set_rmic2b(true);
        val.set_rmn1(true);
        wm.write_control_register(Register::RadcSignalPath, val.0)?;

        // TODO
        // Input boost mixer
        // InputBoostMixer1
        // InputBoostMixer2
        //
        // ADC control
        // Ctr1 above

        // ADC digital volume control
        let mut val = LadcVol(0);
        val.set_adcvu(true);
        val.set_ladcvol(0xC3);
        wm.write_control_register(Register::LadcVol, val.0)?;
        let mut val = RadcVol(0);
        val.set_adcvu(true);
        val.set_radcvol(0xC3);
        wm.write_control_register(Register::RadcVol, val.0)?;

        // ALC control
        let mut val = NoiseGate(0);
        val.set_ngat(true);
        val.set_ngth(0x1F);
        wm.write_control_register(Register::NoiseGate, val.0)?;

        // Configure HP_L and HP_R OUTPUTS
        let mut val = Lout1Vol(0);
        val.set_lout1vol(0x6F);
        val.set_out1vu(true);
        wm.write_control_register(Register::Lout1Vol, val.0)?;
        let mut val = Rout1Vol(0);
        val.set_rout1vol(0x6F);
        val.set_out1vu(true);
        wm.write_control_register(Register::Rout1Vol, val.0)?;

        // Configure SPK_RP and SPK_RN
        let mut val = Lout2Vol(0);
        val.set_spklvol(0x7F);
        val.set_spkvu(true);
        wm.write_control_register(Register::Lout2Vol, val.0)?;
        let mut val = Rout2Vol(0);
        val.set_spkrvol(0x7F);
        val.set_spkvu(true);
        wm.write_control_register(Register::Rout2Vol, val.0)?;

        // Enable the OUTPUTS
        let mut val = ClassdCtr1(0);
        val.set_reserved(0b110111);
        val.set_spkopen(0b11);
        wm.write_control_register(Register::ClassdCtr1, val.0)?;

        // Configure DAC volume
        let mut val = LdacVol(0);
        val.set_ldacvol(0xFF);
        val.set_dacvu(true);
        wm.write_control_register(Register::LdacVol, val.0)?;
        let mut val = RdacVol(0);
        val.set_rdacvol(0xFF);
        val.set_dacvu(true);
        wm.write_control_register(Register::RdacVol, val.0)?;

        // 3D
        // wm.write_reg(0x10, 0x001F);

        // Configure MIXER
        let mut val = LoutMix1(0);
        val.set_li2lo(true);
        val.set_ld2lo(true);
        wm.write_control_register(Register::LoutMix1, val.0)?;
        let mut val = RoutMix1(0);
        val.set_ri2ro(true);
        val.set_rd2ro(true);
        wm.write_control_register(Register::RoutMix1, val.0)?;

        // Jack Detect
        let mut val = Addctr2(0);
        val.set_hpswen(true);
        wm.write_control_register(Register::Addctr2, val.0)?;
        let mut val = Addctr1(0);
        val.set_toen(true);
        val.set_toclksel(true);
        val.set_vsel(0b11);
        val.set_tsden(true);
        wm.write_control_register(Register::Addctr1, val.0)?;
        let mut val = Addctr4(0);
        val.set_mbsel(true);
        val.set_hpsel(0b10);
        wm.write_control_register(Register::Addctr4, val.0)?;

        Ok(wm)
    }

    pub fn write_audio(&mut self, data: &[u16]) -> Result<(), Error> {
        self.i2s.write(data)?;
        Ok(())
    }

    // TODO
    pub fn read_audio(&mut self) -> Result<(), Error> {
        let mut data: [u16; 24] = [0; 24];

        loop {
            let data = self.i2s.read(&mut data)?;
        }

        Ok(())
    }

    /// Write a 9-bit control register
    fn write_control_register(&mut self, reg: Register, data: u16) -> Result<(), Error> {
        let bytes = [
            (reg.addr() << 1) | (data >> 8) as u8 & 0x1,
            (data & 0xFF) as u8,
        ];
        self.i2c.write(DEVICE_ADDRESS, &bytes)?;
        Ok(())
    }

    fn write_reg(&mut self, reg: u8, data: u16) -> Result<(), Error> {
        let bytes = [(reg << 1) | (data >> 8) as u8 & 0x1, (data & 0xFF) as u8];
        self.i2c.write(DEVICE_ADDRESS, &bytes)?;
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
