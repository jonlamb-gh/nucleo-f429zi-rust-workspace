// http://soundfile.sapp.org/doc/WaveFormat/

use core::convert::TryFrom;
use nom::{
    bytes::complete::tag,
    combinator::opt,
    number::complete::{le_u16, le_u32},
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChunkId {
    /// "RIFF" = 0x4646_4952
    RIFF = 0x4646_4952,
    /// "fmt " = 0x2074_6D66
    FMT = 0x2074_6D66,
    /// "fact" = 0x7463_6166
    FACT = 0x7463_6166,
    /// "data" = 0x5453_494C
    DATA = 0x5453_494C,
}

impl ChunkId {
    pub fn as_le_u32(&self) -> u32 {
        *self as _
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Format {
    /// "WAVE"
    WAVE = 0x4556_4157,
}

impl Format {
    pub fn as_le_u32(&self) -> u32 {
        *self as _
    }
}

/// "RIFF" chunk
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChunkRiff {
    pub chunk_id: ChunkId,
    pub chunk_size: u32,
    pub format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AudioFormat {
    PCM = 0x01,
}

impl AudioFormat {
    pub fn as_le_u16(&self) -> u16 {
        *self as _
    }
}

impl TryFrom<u16> for AudioFormat {
    type Error = nom::Err<usize>;

    fn try_from(af: u16) -> Result<Self, Self::Error> {
        if af == AudioFormat::PCM.as_le_u16() {
            Ok(AudioFormat::PCM)
        } else {
            Err(nom::Err::Failure(0))
        }
    }
}

impl Format {
    pub fn as_le_u16(&self) -> u16 {
        *self as _
    }
}

/// "fmt " chunk
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChunkFmt {
    pub chunk_id: ChunkId,
    pub chunk_size: u32,
    pub audio_format: AudioFormat,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

/// "fact" chunk
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChunkFact {
    pub chunk_id: ChunkId,
    pub chunk_size: u32,
    pub fact_size: u32,
}

/// Data chunk
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChunkData {
    pub chunk_id: ChunkId,
    pub chunk_size: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WaveHeader {
    pub riff: ChunkRiff,
    pub fmt: ChunkFmt,
    pub fact: Option<ChunkFact>,
    pub data: ChunkData,
}

impl WaveHeader {
    pub fn data_offset(&self) -> usize {
        // RIFF chunk
        12
        // "fmt " chunk
        + self.fmt.chunk_size as usize + 8
        // Optional "fact" chunk
        + self.fact.map_or(0, |f| f.chunk_size as usize + 8)
        // "data" chunk
        + 8
    }
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], WaveHeader> {
    // "RIFF" chunk
    let (input, _) = tag("RIFF")(input)?;
    let (input, riff_chunk_size) = le_u32(input)?;
    let (input, _) = tag("WAVE")(input)?;
    let riff = ChunkRiff {
        chunk_id: ChunkId::RIFF,
        chunk_size: riff_chunk_size,
        format: Format::WAVE,
    };

    // "fmt " chunk
    let (input, _) = tag("fmt ")(input)?;
    let (input, fmt_chunk_size) = le_u32(input)?;
    let (input, audio_format) = le_u16(input)?;
    let audio_format = AudioFormat::try_from(audio_format)
        .map_err(|_| nom::Err::Failure((input, nom::error::ErrorKind::ParseTo)))?;
    let (input, num_channels) = le_u16(input)?;
    let (input, sample_rate) = le_u32(input)?;
    let (input, byte_rate) = le_u32(input)?;
    let (input, block_align) = le_u16(input)?;
    let (input, bits_per_sample) = le_u16(input)?;

    let input = if fmt_chunk_size == 18 {
        let (input, _) = le_u16(input)?;
        input
    } else {
        input
    };

    let fmt = ChunkFmt {
        chunk_id: ChunkId::FMT,
        chunk_size: fmt_chunk_size,
        audio_format,
        num_channels,
        sample_rate,
        byte_rate,
        block_align,
        bits_per_sample,
    };

    // "fact" chunk
    let (input, maybe_fact) = opt(tag("fact"))(input)?;
    let (input, fact) = if maybe_fact.is_some() {
        let (input, fact_chunk_size) = le_u32(input)?;
        let (input, fact_size) = le_u32(input)?;
        let fact = Some(ChunkFact {
            chunk_id: ChunkId::FACT,
            chunk_size: fact_chunk_size,
            fact_size,
        });
        (input, fact)
    } else {
        (input, None)
    };

    // "data" chunk
    let (input, _) = tag("data")(input)?;
    let (input, data_chunk_size) = le_u32(input)?;
    let data = ChunkData {
        chunk_id: ChunkId::DATA,
        chunk_size: data_chunk_size,
    };

    Ok((
        input,
        WaveHeader {
            riff,
            fmt,
            fact,
            data,
        },
    ))
}
