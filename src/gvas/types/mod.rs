use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

pub mod bool;
pub mod fstring;
pub mod guid;

use crate::impl_static_size_struct;

pub use self::bool::Bool;
pub use self::fstring::FString;
pub use self::guid::GUID;

macro_rules! impl_basic_struct_rw {
    ($name:ident { $($field_name:ident),* }) => {
        impl BinRead for $name {
            type Args<'a> = ();

            fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
                reader: &mut R,
                endian: binrw::Endian,
                args: Self::Args<'_>,
            ) -> binrw::prelude::BinResult<Self> {
                Ok(Self {
                    $($field_name: <_>::read_options(reader, endian, args)?,)*
                })
            }
        }

        impl BinWrite for $name {
            type Args<'a> = ();

            fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
                &self,
                writer: &mut W,
                endian: binrw::Endian,
                args: Self::Args<'_>,
            ) -> binrw::prelude::BinResult<()> {
                $(self.$field_name.write_options(writer, endian, args)?;)*
                Ok(())
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl_static_size_struct!(Vector { f32, f32, f32 });
impl_basic_struct_rw!(Vector { x, y, z });

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl_static_size_struct!(LinearColor { f32, f32, f32, f32 });
impl_basic_struct_rw!(LinearColor { r, g, b, a });

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl_static_size_struct!(Quat { f32, f32, f32, f32 });
impl_basic_struct_rw!(Quat { x, y, z, w });

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Rotator {
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
}

impl_static_size_struct!(Rotator { f32, f32, f32 });
impl_basic_struct_rw!(Rotator { pitch, roll, yaw });
