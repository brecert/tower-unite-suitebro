use std::{io::Cursor, marker::PhantomData};

use binrw::BinWrite;

pub trait StaticByteSize {
    const BYTE_SIZE: usize;
}

impl<T: StaticByteSize> ByteSize for T {
    fn byte_size(&self) -> usize {
        Self::BYTE_SIZE
    }
}

pub trait ByteSize {
    fn byte_size(&self) -> usize;
}

// this is so bad lol
// impl<T: for<'a> BinWrite<Args<'a> = ()>> ByteSize for T {
//     fn byte_size(&self) -> usize {
//         let mut bytes = Vec::new();
//         let mut value = Cursor::new(&mut bytes);
//         self.write_options(&mut value, binrw::Endian::Little, ())
//             .unwrap();
//         bytes.len()
//     }
// }

// impl<T: ByteSize> ByteSize for &T {
//     fn byte_size(&self) -> usize {
//         (**self).byte_size()
//     }
// }

// impl<T: ByteSize> ByteSize for Box<T> {
//     fn byte_size(&self) -> usize {
//         self.as_ref().byte_size()
//     }
// }

impl<T: ByteSize> ByteSize for Option<T> {
    fn byte_size(&self) -> usize {
        match self {
            Some(inner) => inner.byte_size(),
            None => 0,
        }
    }
}

impl<T: ByteSize> ByteSize for Vec<T> {
    fn byte_size(&self) -> usize {
        self.iter().map(|val| val.byte_size()).sum()
    }
}

// impl<T: StaticByteSize, const N: usize> StaticByteSize for [T; N] {
//     const BYTE_SIZE: usize = T::BYTE_SIZE * N;
// }

impl<T: ByteSize, const N: usize> ByteSize for [T; N] {
    fn byte_size(&self) -> usize {
        self.iter().map(|val| val.byte_size()).sum()
    }
}

impl<T: ByteSize> ByteSize for [T] {
    fn byte_size(&self) -> usize {
        self.iter().map(|val| val.byte_size()).sum()
    }
}

impl<T> ByteSize for PhantomData<T> {
    fn byte_size(&self) -> usize {
        0
    }
}

macro_rules! impl_static_size {
    ($ty:ty, $size:literal) => {
        impl StaticByteSize for $ty {
            const BYTE_SIZE: usize = $size;
        }
    };
}

#[macro_export]
macro_rules! impl_static_size_struct {
    ($type:ty { $($field_type:ty),* }) => {
        impl crate::byte_size::StaticByteSize for $type {
            const BYTE_SIZE: usize = $(<$field_type as crate::byte_size::StaticByteSize>::BYTE_SIZE + )* 0;
        }
    };
}

impl_static_size!(i8, 1);
impl_static_size!(i16, 2);
impl_static_size!(i32, 4);
impl_static_size!(i64, 8);
impl_static_size!(i128, 16);

impl_static_size!(u8, 1);
impl_static_size!(u16, 2);
impl_static_size!(u32, 4);
impl_static_size!(u64, 8);
impl_static_size!(u128, 16);

impl_static_size!(f32, 4);
impl_static_size!(f64, 8);

impl_static_size!(std::num::NonZeroI8, 1);
impl_static_size!(std::num::NonZeroI16, 2);
impl_static_size!(std::num::NonZeroI32, 4);
impl_static_size!(std::num::NonZeroI64, 8);
impl_static_size!(std::num::NonZeroI128, 16);

impl_static_size!(std::num::NonZeroU8, 1);
impl_static_size!(std::num::NonZeroU16, 2);
impl_static_size!(std::num::NonZeroU32, 4);
impl_static_size!(std::num::NonZeroU64, 8);
impl_static_size!(std::num::NonZeroU128, 16);
