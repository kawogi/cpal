// reason: allows for some convenience aliases
#![allow(clippy::module_name_repetitions)]

use std::mem;

use crate::{sample_buffer, sample_primitive};

use super::{RawSample, SimpleLittleBigEncoding};
use dasp_sample::Sample;

pub type Primitive = i16;
pub const DEFAULT: Primitive = Primitive::EQUILIBRIUM;
type Encoding = SimpleLittleBigEncoding<{ mem::size_of::<Primitive>() }>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct LE([u8; 2]);

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct BE([u8; 2]);

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

sample_primitive!(I16: LE, BE);
sample_buffer!(I16: LE, BE);
pub type I16SampleBuffer<'buffer> = SampleBuffer<'buffer>;
pub type I16SampleBufferMut<'buffer> = SampleBufferMut<'buffer>;
