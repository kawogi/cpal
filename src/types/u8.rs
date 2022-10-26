use std::mem;

use crate::{sample_buffer, sample_primitive};

use super::{RawSample, SimpleEncoding};

pub type Primitive = u8;
const DEFAULT: Primitive = <Primitive as dasp_sample::Sample>::EQUILIBRIUM;
type Encoding = SimpleEncoding<{ mem::size_of::<Primitive>() }>;

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

sample_primitive!(U8: NE);
sample_buffer!(U8: NE);
pub type U8SampleBuffer<'buffer> = SampleBuffer<'buffer>;
pub type U8SampleBufferMut<'buffer> = SampleBufferMut<'buffer>;
