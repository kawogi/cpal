pub mod f32;
pub mod f64;
pub mod i16;
pub mod i24;
pub mod i32;
pub mod i8;
pub mod u16;
pub mod u24;
pub mod u32;
pub mod u8;

enum SampleBufferType {
    // i8
    InterleavedI8,
    SeparatedI8,

    // i16
    InterleavedI16LE,
    SeparatedI16LE,
    InterleavedI16BE,
    SeparatedI16BE,

    // I18
    InterleavedI18LE3B,
    SeparatedI18LE3B,
    InterleavedI18BE3B,
    SeparatedI18BE3B,
    InterleavedI18LE4B,
    SeparatedI18LE4B,
    InterleavedI18BE4B,
    SeparatedI18BE4B,

    // I20
    InterleavedI20LE3B,
    SeparatedI20LE3B,
    InterleavedI20BE3B,
    SeparatedI20BE3B,
    InterleavedI20LE4B,
    SeparatedI20LE4B,
    InterleavedI20BE4B,
    SeparatedI20BE4B,

    // I24
    InterleavedI24LE3B,
    SeparatedI24LE3B,
    InterleavedI24BE3B,
    SeparatedI24BE3B,
    InterleavedI24LE4B,
    SeparatedI24LE4B,
    InterleavedI24BE4B,
    SeparatedI24BE4B,

    // i32
    InterleavedI32LE,
    SeparatedI32LE,
    InterleavedI32BE,
    SeparatedI32BE,

    // u8
    InterleavedU8,
    SeparatedU8,

    // u16
    InterleavedU16LE,
    SeparatedU16LE,
    InterleavedU16BE,
    SeparatedU16BE,

    // U18
    InterleavedU18LE3B,
    SeparatedU18LE3B,
    InterleavedU18BE3B,
    SeparatedU18BE3B,
    InterleavedU18LE4B,
    SeparatedU18LE4B,
    InterleavedU18BE4B,
    SeparatedU18BE4B,

    // U20
    InterleavedU20LE3B,
    SeparatedU20LE3B,
    InterleavedU20BE3B,
    SeparatedU20BE3B,
    InterleavedU20LE4B,
    SeparatedU20LE4B,
    InterleavedU20BE4B,
    SeparatedU20BE4B,

    // U24
    InterleavedU24LE3B,
    SeparatedU24LE3B,
    InterleavedU24BE3B,
    SeparatedU24BE3B,
    InterleavedU24LE4B,
    SeparatedU24LE4B,
    InterleavedU24BE4B,
    SeparatedU24BE4B,

    // u32
    InterleavedU32LE,
    SeparatedU32LE,
    InterleavedU32BE,
    SeparatedU32BE,

    // f32
    InterleavedF32LE,
    SeparatedF32LE,
    InterleavedF32BE,
    SeparatedF32BE,

    // f64
    InterleavedF64LE,
    SeparatedF64LE,
    InterleavedF64BE,
    SeparatedF64BE,
}

/// A single sample in its raw memory representation (`[u8; N]`).
pub trait RawSample: Copy + From<Self::Primitive> + 'static
where
    Self::Primitive: From<Self>,
{
    /// The _public facing_ type to use when converting from/to the raw byte representation. (e.g. `i16`, `I24`, `f32`)
    type Primitive: Copy;
}

// /// provides write-only access to a single byte-backed sample
// pub struct SampleMut<'buffer, T> {
//     raw_sample: &'buffer mut T,
// }

// impl<'buffer, T> SampleMut<'buffer, T> {
//     #[inline]
//     pub fn new(raw_sample: &'buffer mut T) -> Self {
//         Self { raw_sample }
//     }
// }

// impl<'buffer, T: RawSample> SampleMut<'buffer, T> {
//     // This would allow read access to a write-only buffer
//     // #[inline]
//     // pub fn get(&self) -> T::Primitive {
//     //     T::Primitive::from(*self.raw_sample)
//     // }

//     #[inline]
//     pub fn set(&mut self, sample: T::Primitive) {
//         *self.raw_sample = T::from(sample);
//     }
// }

// This would allow read access to a write-only buffer

// impl<'buffer, T: RawSample> Deref for SampleMut<'buffer, T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         self.raw_sample
//     }
// }

// impl<'buffer, T: RawSample> DerefMut for SampleMut<'buffer, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.raw_sample
//     }
// }
