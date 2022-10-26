use std::{fmt::Display, mem};

use crate::{sample_buffer, sized_sample};

use super::RawSample;
use dasp_sample::Sample;

pub type Primitive = u64;
pub const DEFAULT: Primitive = Primitive::EQUILIBRIUM;
//pub const FORMAT: SampleFormat = SampleFormat::U64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RawFormat {
    LE,
    BE,
}

impl RawFormat {
    #[inline]
    #[must_use]
    pub fn sample_size(self) -> usize {
        match self {
            Self::LE => mem::size_of::<LE>(),
            Self::BE => mem::size_of::<BE>(),
        }
    }
}

impl Display for RawFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RawFormat::LE => "le",
            RawFormat::BE => "be",
        }
        .fmt(f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct LE([u8; 8]);

impl Default for LE {
    fn default() -> Self {
        Self::from(DEFAULT)
    }
}

impl From<Primitive> for LE {
    fn from(v: Primitive) -> Self {
        Self(v.to_le_bytes())
    }
}

impl From<LE> for Primitive {
    fn from(v: LE) -> Self {
        Self::from_le_bytes(v.0)
    }
}

impl RawSample for LE {
    type Primitive = Primitive;
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BE([u8; 8]);

impl Default for BE {
    fn default() -> Self {
        Self::from(DEFAULT)
    }
}

impl From<Primitive> for BE {
    fn from(v: Primitive) -> Self {
        Self(v.to_be_bytes())
    }
}

impl From<BE> for Primitive {
    fn from(v: BE) -> Self {
        Self::from_be_bytes(v.0)
    }
}

impl RawSample for BE {
    type Primitive = Primitive;
}

sized_sample!(U64: LE, BE);
sample_buffer!(LE, BE);
pub type F64SampleBuffer<'buffer> = SampleBuffer<'buffer>;
pub type F64SampleBufferMut<'buffer> = SampleBufferMut<'buffer>;
