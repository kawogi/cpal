use crate::{
    buffers::{ChannelIndex, FrameIndex, SampleAddress},
    sample_buffer,
};

use super::RawSample;
use dasp_sample::{Sample, U24};

pub type Primitive = U24;
pub const DEFAULT: Primitive = Primitive::EQUILIBRIUM;
// TODO ask author of `dasp_sample` why this couldn't be `u32`
type Repr = i32;

/// Bit memory layout: [0..7, 8..15, 16..23]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct LE3B([u8; 3]);

impl Default for LE3B {
    fn default() -> Self {
        Self::from(DEFAULT)
    }
}

impl From<Primitive> for LE3B {
    fn from(v: Primitive) -> Self {
        let repr_bytes = v.inner().to_le_bytes();
        // `Repr` bit memory layout: [0..7, 8..15, 16..23, _]
        // `Self` bit memory layout: [0..7, 8..15, 16..23]
        Self([repr_bytes[0], repr_bytes[1], repr_bytes[2]])
    }
}

impl From<LE3B> for Primitive {
    fn from(v: LE3B) -> Self {
        // `Self` bit memory layout: [0..7, 8..15, 16..23]
        // `Repr` bit memory layout: [0..7, 8..15, 16..23, _]
        Self::new_unchecked(Repr::from_le_bytes([v.0[0], v.0[1], v.0[2], 0]))
    }
}

impl RawSample for LE3B {
    type Primitive = Primitive;
}

/// Bit memory layout: [16..23, 8..15, 0..7]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct BE3B([u8; 3]);

impl Default for BE3B {
    fn default() -> Self {
        Self::from(DEFAULT)
    }
}

impl From<Primitive> for BE3B {
    fn from(v: Primitive) -> Self {
        let repr_bytes = v.inner().to_be_bytes();
        // `Repr` bit memory layout: [_, 16..23, 8..15, 0..7]
        // `Self` bit memory layout: [16..23, 8..15, 0..7]
        Self([repr_bytes[1], repr_bytes[2], repr_bytes[3]])
    }
}

impl From<BE3B> for Primitive {
    fn from(v: BE3B) -> Self {
        // `Self` bit memory layout: [16..23, 8..15, 0..7]
        // `Repr` bit memory layout: [_, 16..23, 8..15, 0..7]
        Self::new_unchecked(Repr::from_be_bytes([0, v.0[0], v.0[1], v.0[2]]))
    }
}

impl RawSample for BE3B {
    type Primitive = Primitive;
}

/// Bit memory layout: [0..7, 8..15, 16..23, _]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct LE4B([u8; 4]);

impl Default for LE4B {
    fn default() -> Self {
        Self::from(DEFAULT)
    }
}

impl From<Primitive> for LE4B {
    fn from(v: Primitive) -> Self {
        // `Repr` bit memory layout: [0..7, 8..15, 16..23, _]
        // `Self` bit memory layout: [0..7, 8..15, 16..23, _]
        Self(v.inner().to_le_bytes())
    }
}

impl From<LE4B> for Primitive {
    fn from(v: LE4B) -> Self {
        // `Self` bit memory layout: [0..7, 8..15, 16..23, _]
        // `Repr` bit memory layout: [0..7, 8..15, 16..23, _]
        // load bytes into upper bits and shift right to sign-extend the result
        Self::new_unchecked(Repr::from_le_bytes([v.0[0], v.0[1], v.0[2], 0]))
    }
}

impl PartialEq for LE4B {
    fn eq(&self, other: &Self) -> bool {
        self.0[0..3] == other.0[0..3]
    }
}

impl Eq for LE4B {}

impl RawSample for LE4B {
    type Primitive = Primitive;
}

/// Bit memory layout: [_, 16..23, 8..15, 0..7]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct BE4B([u8; 4]);

impl Default for BE4B {
    fn default() -> Self {
        Self::from(DEFAULT)
    }
}

impl From<Primitive> for BE4B {
    fn from(v: Primitive) -> Self {
        // `Repr` bit memory layout: [_, 16..23, 8..15, 0..7]
        // `Self` bit memory layout: [_, 16..23, 8..15, 0..7]
        Self(v.inner().to_be_bytes())
    }
}

impl From<BE4B> for Primitive {
    fn from(v: BE4B) -> Self {
        // `Self` bit memory layout: [_, 16..23, 8..15, 0..7]
        // `Repr` bit memory layout: [_, 16..23, 8..15, 0..7]
        // load bytes into upper bits and shift right to sign-extend the result
        Self::new_unchecked(Repr::from_be_bytes([0, v.0[1], v.0[2], v.0[3]]))
    }
}

impl RawSample for BE4B {
    type Primitive = Primitive;
}

impl PartialEq for BE4B {
    fn eq(&self, other: &Self) -> bool {
        self.0[1..4] == other.0[1..4]
    }
}

impl Eq for BE4B {}

sample_buffer!(LE3B, BE3B, LE4B, BE4B);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_le3b() {
        {
            // default
            let primitive = Primitive::EQUILIBRIUM;
            let raw = LE3B::default();
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE3B::from(primitive), raw);
        }

        {
            // min/zero
            let primitive = Primitive::new(0).expect("out of valid range");
            let raw = LE3B([0, 0, 0]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE3B::from(primitive), raw);
        }

        {
            // max
            let primitive = Primitive::new(0xff_ff_ff).expect("out of valid range");
            let raw = LE3B([0xff, 0xff, 0xff]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE3B::from(primitive), raw);
        }

        {
            // least significant byte
            let primitive = Primitive::new(0x00_00_01).expect("out of valid range");
            let raw = LE3B([0x01, 0x00, 0x00]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE3B::from(primitive), raw);
        }

        {
            // middle byte
            let primitive = Primitive::new(0x00_01_00).expect("out of valid range");
            let raw = LE3B([0x00, 0x01, 0x00]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE3B::from(primitive), raw);
        }

        {
            // most significant byte
            let primitive = Primitive::new(0x01_00_00).expect("out of valid range");
            let raw = LE3B([0x00, 0x00, 0x01]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE3B::from(primitive), raw);
        }

        {
            // most significant bit
            let primitive = Primitive::new(0x80_00_00).expect("out of valid range");
            let raw = LE3B([0x00, 0x00, 0x80]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE3B::from(primitive), raw);
        }
    }

    #[test]
    fn test_be3b() {
        {
            // default
            let primitive = Primitive::EQUILIBRIUM;
            let raw = BE3B::default();
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE3B::from(primitive), raw);
        }

        {
            // min/zero
            let primitive = Primitive::new(0).expect("out of valid range");
            let raw = BE3B([0, 0, 0]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE3B::from(primitive), raw);
        }

        {
            // max
            let primitive = Primitive::new(0xff_ff_ff).expect("out of valid range");
            let raw = BE3B([0xff, 0xff, 0xff]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE3B::from(primitive), raw);
        }

        {
            // least significant byte
            let primitive = Primitive::new(0x00_00_01).expect("out of valid range");
            let raw = BE3B([0x00, 0x00, 0x01]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE3B::from(primitive), raw);
        }

        {
            // middle byte
            let primitive = Primitive::new(0x00_01_00).expect("out of valid range");
            let raw = BE3B([0x00, 0x01, 0x00]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE3B::from(primitive), raw);
        }

        {
            // most significant byte
            let primitive = Primitive::new(0x01_00_00).expect("out of valid range");
            let raw = BE3B([0x01, 0x00, 0x00]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE3B::from(primitive), raw);
        }

        {
            // most significant bit
            let primitive = Primitive::new(0x80_00_00).expect("out of valid range");
            let raw = BE3B([0x80, 0x00, 0x00]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE3B::from(primitive), raw);
        }
    }

    #[test]
    fn test_le4b() {
        let undefined = 123;
        {
            // default
            let primitive = Primitive::EQUILIBRIUM;
            let raw = LE4B::default();
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE4B::from(primitive), raw);
        }

        {
            // min/zero
            let primitive = Primitive::new(0).expect("out of valid range");
            let raw = LE4B([0, 0, 0, undefined]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE4B::from(primitive), raw);
        }

        {
            // max
            let primitive = Primitive::new(0xff_ff_ff).expect("out of valid range");
            let raw = LE4B([0xff, 0xff, 0xff, undefined]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE4B::from(primitive), raw);
        }

        {
            // least significant byte
            let primitive = Primitive::new(0x00_00_01).expect("out of valid range");
            let raw = LE4B([0x01, 0x00, 0x00, undefined]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE4B::from(primitive), raw);
        }

        {
            // middle byte
            let primitive = Primitive::new(0x00_01_00).expect("out of valid range");
            let raw = LE4B([0x00, 0x01, 0x00, undefined]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE4B::from(primitive), raw);
        }

        {
            // most significant byte
            let primitive = Primitive::new(0x01_00_00).expect("out of valid range");
            let raw = LE4B([0x00, 0x00, 0x01, undefined]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE4B::from(primitive), raw);
        }

        {
            // most significant bit
            let primitive = Primitive::new(0x80_00_00).expect("out of valid range");
            let raw = LE4B([0x00, 0x00, 0x80, undefined]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(LE4B::from(primitive), raw);
        }
    }

    #[test]
    fn test_be4b() {
        let undefined = 123;
        {
            // default
            let primitive = Primitive::EQUILIBRIUM;
            let raw = BE4B::default();
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE4B::from(primitive), raw);
        }

        {
            // min/zero
            let primitive = Primitive::new(0).expect("out of valid range");
            let raw = BE4B([undefined, 0, 0, 0]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE4B::from(primitive), raw);
        }

        {
            // max
            let primitive = Primitive::new(0xff_ff_ff).expect("out of valid range");
            let raw = BE4B([undefined, 0xff, 0xff, 0xff]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE4B::from(primitive), raw);
        }

        {
            // least significant byte
            let primitive = Primitive::new(0x00_00_01).expect("out of valid range");
            let raw = BE4B([undefined, 0x00, 0x00, 0x01]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE4B::from(primitive), raw);
        }

        {
            // middle byte
            let primitive = Primitive::new(0x00_01_00).expect("out of valid range");
            let raw = BE4B([undefined, 0x00, 0x01, 0x00]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE4B::from(primitive), raw);
        }

        {
            // most significant byte
            let primitive = Primitive::new(0x01_00_00).expect("out of valid range");
            let raw = BE4B([undefined, 0x01, 0x00, 0x00]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE4B::from(primitive), raw);
        }

        {
            // most significant bit
            let primitive = Primitive::new(0x80_00_00).expect("out of valid range");
            let raw = BE4B([undefined, 0x80, 0x00, 0x00]);
            assert_eq!(Primitive::from(raw), primitive);
            assert_eq!(BE4B::from(primitive), raw);
        }
    }
}