use bitfield::bitfield;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Register {
    LeftInputVol = 0x00,
    RightInputVol = 0x01,
    Lout1Vol = 0x02,
    Rout1Vol = 0x03,
    Clocking = 0x04,
    Ctr1 = 0x05,
    Ctr2 = 0x06,
    AudioIface = 0x07,
    Clocking2 = 0x08,
    AudioIface2 = 0x09,
    LdacVol = 0x0A,
    RdacVol = 0x0B,
    Reset = 0x0F,
    Ctr3D = 0x10,
    Alc1 = 0x11,
    Alc2 = 0x12,
    Alc3 = 0x13,
    NoiseGate = 0x14,
    LadcVol = 0x15,
    RadcVol = 0x16,
    Addctr1 = 0x17,
    Addctr2 = 0x18,
    PwrMgmt1 = 0x19,
    PwrMgmt2 = 0x1A,
    Addctr3 = 0x1B,
    AntiPop1 = 0x1C,
    AntiPop2 = 0x1D,
    LadcSignalPath = 0x20,
    RadcSignalPath = 0x21,
    LoutMix1 = 0x22,
    RoutMix1 = 0x25,
    MonoOutMix1 = 0x26,
    MonoOutMix2 = 0x27,
    Lout2Vol = 0x28,
    Rout2Vol = 0x29,
    MonoOutVol = 0x2A,
    InputBoostMixer1 = 0x2B,
    InputBoostMixer2 = 0x2C,
    Bypass1 = 0x2D,
    Bypass2 = 0x2E,
    PwrMgmt3 = 0x2F,
    Addctr4 = 0x30,
    ClassdCtr1 = 0x31,
    ClassdCtr3 = 0x33,
    PllN = 0x34,
    PllK1 = 0x35,
    PllK2 = 0x36,
    PllK3 = 0x37,
}

impl Register {
    pub(crate) fn addr(&self) -> u8 {
        *self as u8
    }
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct LeftInputVol(u16);
    u16;
    pub linvol, set_linvol : 5, 0;
    pub lizc, set_lizc : 6;
    pub linmute, set_linmute : 7;
    pub ipvu, set_ipvu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct RightInputVol(u16);
    u16;
    pub rinvol, set_rinvol : 5, 0;
    pub rizc, set_rizc : 6;
    pub rinmute, set_rinmute : 7;
    pub ipvu, set_ipvu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Lout1Vol(u16);
    u16;
    pub lout1vol, set_lout1vol : 6, 0;
    pub lo1zc, set_lo1zc : 7;
    pub out1vu, set_out1vu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Rout1Vol(u16);
    u16;
    pub rout1vol, set_rout1vol : 6, 0;
    pub ro1zc, set_ro1zc : 7;
    pub out1vu, set_out1vu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Clocking(u16);
    u16;
    pub clksel, set_clksel : 0;
    pub sysclkdiv, set_sysclkdiv : 2, 1;
    pub dacdiv, set_dacdiv : 5, 3;
    pub adcdiv, set_adcdiv : 8, 6;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Ctr1(u16);
    u16;
    pub deemph, set_deemph : 2, 1;
    pub dacmu, set_dacmu : 3;
    pub adcpol, set_adpol : 6, 5;
    pub dacdiv2, set_dacdiv2 : 7;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Ctr2(u16);
    u16;
    pub dacslope, set_dacslope : 1;
    pub dacmr, set_dacmr : 2;
    pub dacsmm, set_dacsmm : 3;
    pub dacpol, set_dacpol : 6, 5;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct AudioIface(u16);
    u16;
    pub format, set_format : 1, 0;
    pub wl, set_wl : 3, 2;
    pub lrp, set_lrp : 4;
    pub dlrswap, set_dlrswap : 5;
    pub ms, set_ms : 6;
    pub bclkinv, set_bclkinv : 7;
    pub alrswap, set_alrswap : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct LdacVol(u16);
    u16;
    pub ldacvol, set_ldacvol : 7, 0;
    pub dacvu, set_dacvu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct RdacVol(u16);
    u16;
    pub rdacvol, set_rdacvol : 7, 0;
    pub dacvu, set_dacvu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct NoiseGate(u16);
    u16;
    pub ngat, set_ngat : 0;
    pub ngth, set_ngth : 7, 3;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct LadcVol(u16);
    u16;
    pub ladcvol, set_ladcvol : 7, 0;
    pub adcvu, set_adcvu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct RadcVol(u16);
    u16;
    pub radcvol, set_radcvol : 7, 0;
    pub adcvu, set_adcvu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Addctr1(u16);
    u16;
    pub toen, set_toen : 0;
    pub toclksel, set_toclksel : 1;
    pub datsel, set_datsel : 3, 2;
    pub dmonomix, set_dmonomix : 4;
    pub vsel, set_vsel : 7, 6;
    pub tsden, set_tsden : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Addctr2(u16);
    u16;
    pub lrcm, set_lrcm : 2;
    pub tris, set_tris : 3;
    pub hpswpol, set_hpswpol : 5;
    pub hpswen, set_hpswen : 6;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Addctr3(u16);
    u16;
    pub adc_alc_sr, set_adc_alc_sr : 2, 0;
    pub out3cap, set_out3cap : 3;
    pub vroi, set_vroi : 6;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct PwrMgmt1(u16);
    u16;
    pub digenb, set_digenb : 0;
    pub micb, set_micb : 1;
    pub adcr, set_adcr : 2;
    pub adcl, set_adcl : 3;
    pub ainr, set_ainr : 4;
    pub ainl, set_ainl : 5;
    pub vref, set_vref : 6;
    pub vmidsel, set_vmidsel : 8, 7;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct PwrMgmt2(u16);
    u16;
    pub pllen, set_pllen : 0;
    pub out3, set_out3 : 1;
    pub spkr, set_spkr : 3;
    pub spkl, set_spkl : 4;
    pub rout1, set_rout1 : 5;
    pub lout1, set_lout1 : 6;
    pub dacr, set_dacr : 7;
    pub dacl, set_dacl : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct LadcSignalPath(u16);
    u16;
    pub lmic2b, set_lmic2b : 3;
    pub lmicboost, set_lmicboost : 5, 4;
    pub lmp2, set_lmp2 : 6;
    pub lmp3, set_lmp3 : 7;
    pub lmn1, set_lmn1 : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct RadcSignalPath(u16);
    u16;
    pub rmic2b, set_rmic2b : 3;
    pub rmicboost, set_rmicboost : 5, 4;
    pub rmp2, set_rmp2 : 6;
    pub rmp3, set_rmp3 : 7;
    pub rmn1, set_rmn1 : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct LoutMix1(u16);
    u16;
    pub li2lovol, set_li2lovol : 6, 4;
    pub li2lo, set_li2lo : 7;
    pub ld2lo, set_ld2lo : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct RoutMix1(u16);
    u16;
    pub ri2rovol, set_ri2rovol : 6, 4;
    pub ri2ro, set_ri2ro : 7;
    pub rd2ro, set_rd2ro : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Lout2Vol(u16);
    u16;
    pub spklvol, set_spklvol : 6, 0;
    pub spklzc, set_spklzc : 7;
    pub spkvu, set_spkvu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Rout2Vol(u16);
    u16;
    pub spkrvol, set_spkrvol : 6, 0;
    pub spkrzc, set_spkrzc : 7;
    pub spkvu, set_spkvu : 8;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct PwrMgmt3(u16);
    u16;
    pub romix, set_romix : 2;
    pub lomix, set_lomix : 3;
    pub rmic, set_rmic : 4;
    pub lmic, set_lmic : 5;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Addctr4(u16);
    u16;
    pub mbsel, set_mbsel : 0;
    pub tsensen, set_tsensen : 1;
    pub hpsel, set_hpsel : 3, 2;
    pub gpiosel, set_gpiosel : 6, 4;
    pub gpiopol, set_gpiopol : 7;
}

bitfield! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct ClassdCtr1(u16);
    u16;
    /// Should be 0b110111
    pub reserved, set_reserved : 5, 0;
    pub spkopen, set_spkopen : 7, 6;
}
