pub mod f32;
pub mod f64;
pub mod i16;
pub mod i24;
pub mod i32;
pub mod i64;
pub mod i8;
pub mod u16;
pub mod u24;
pub mod u32;
pub mod u64;
pub mod u8;

/// A single sample in its raw memory representation (`[u8; N]`).
pub trait RawSample: std::fmt::Debug + Copy + From<Self::Primitive> + 'static
where
    Self::Primitive: From<Self>,
{
    /// The _public facing_ type to use when converting from/to the raw byte representation. (e.g. `i16`, `I24`, `f32`)
    type Primitive: Copy;
}

pub trait Encoding: Copy + Sized {
    /// number of bytes the sample format occupies in a slice
    #[must_use]
    fn sample_size(self) -> usize;

    /// Returns whether the sample format is little endian
    #[must_use]
    fn is_le(self) -> bool;

    /// Returns whether the sample format is big endian
    #[must_use]
    fn is_be(self) -> bool;

    /// Returns whether the sample format is in native endianness
    #[inline]
    #[must_use]
    fn is_ne(self) -> bool {
        #[cfg(target_endian = "little")]
        return self.is_le();
        #[cfg(target_endian = "big")]
        return self.is_be();
    }
}

/// Simple raw format descriptor where only the variants _little endian_ and _big_endian_ exist and both types share
/// the same size.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SimpleLittleBigEncoding<const SIZE: usize> {
    LE,
    BE,
}

impl<const SIZE: usize> Encoding for SimpleLittleBigEncoding<SIZE> {
    #[inline]
    #[must_use]
    fn sample_size(self) -> usize {
        SIZE
    }

    #[inline]
    #[must_use]
    fn is_le(self) -> bool {
        matches!(self, Self::LE)
    }

    #[inline]
    #[must_use]
    fn is_be(self) -> bool {
        matches!(self, Self::BE)
    }
}

impl<const SIZE: usize> std::fmt::Display for SimpleLittleBigEncoding<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::LE => "le",
            Self::BE => "be",
        }
        .fmt(f)
    }
}

/// Simple raw format descriptor where only only one variant (native endianess) exists.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SimpleEncoding<const SIZE: usize> {
    NE,
}

impl<const SIZE: usize> Encoding for SimpleEncoding<SIZE> {
    #[inline]
    #[must_use]
    fn sample_size(self) -> usize {
        SIZE
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

impl<const SIZE: usize> std::fmt::Display for SimpleEncoding<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NE => "ne",
        }
        .fmt(f)
    }
}
