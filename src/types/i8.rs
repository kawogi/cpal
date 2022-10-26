use std::{fmt::Display, mem};

use crate::{sample_buffer, sized_sample};

use super::RawSample;
use dasp_sample::Sample;

pub type Primitive = i8;
pub const DEFAULT: Primitive = Primitive::EQUILIBRIUM;
//pub const FORMAT: SampleFormat = SampleFormat::I8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RawFormat {
    NE,
}

impl super::RawFormat for RawFormat {
    #[inline]
    #[must_use]
    fn sample_size(self) -> usize {
        match self {
            Self::NE => mem::size_of::<NE>(),
        }
    }

    #[inline]
    #[must_use]
    fn is_le(self) -> bool {
        matches!(self, Self::NE)
    }

    #[inline]
    #[must_use]
    fn is_be(self) -> bool {
        matches!(self, Self::NE)
    }
}

impl Display for RawFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RawFormat::NE => "ne",
        }
        .fmt(f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct NE([u8; 1]);

impl Default for NE {
    fn default() -> Self {
        Self::from(DEFAULT)
    }
}

impl From<Primitive> for NE {
    fn from(v: Primitive) -> Self {
        Self(v.to_le_bytes())
    }
}

impl From<NE> for Primitive {
    fn from(v: NE) -> Self {
        Self::from_le_bytes(v.0)
    }
}

impl RawSample for NE {
    type Primitive = Primitive;
}

sized_sample!(I8: NE);
sample_buffer!(NE);
pub type I8SampleBuffer<'buffer> = SampleBuffer<'buffer>;
pub type I8SampleBufferMut<'buffer> = SampleBufferMut<'buffer>;
