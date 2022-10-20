use crate::{
    buffers::{ChannelIndex, FrameIndex, SampleAddress},
    sample_buffer,
};

use super::RawSample;
use dasp_sample::Sample;

pub type Primitive = u8;
pub const DEFAULT: Primitive = Primitive::EQUILIBRIUM;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct LE([u8; 1]);

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
pub struct BE([u8; 1]);

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

sample_buffer!(LE, BE);
