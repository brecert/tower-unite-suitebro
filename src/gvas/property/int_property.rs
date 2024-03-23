use crate::byte_size::StaticByteSize;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

macro_rules! number_property {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
        #[serde(transparent)]
        pub struct $name($type);

        impl BinRead for $name {
            type Args<'a> = ();

            fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
                reader: &mut R,
                endian: binrw::Endian,
                args: Self::Args<'_>,
            ) -> binrw::prelude::BinResult<Self> {
                let size = u64::read_options(reader, endian, args)?;
                let seperator = u8::read_options(reader, endian, args)?;
                let value = <$type>::read_options(reader, endian, args)?;

                assert_eq!(size, <$type>::BYTE_SIZE as u64);
                assert_eq!(seperator, 0);

                Ok(Self(value))
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
                let size = <$type>::BYTE_SIZE as u64;
                let seperator = 0u8;

                size.write_options(writer, endian, args)?;
                seperator.write_options(writer, endian, args)?;
                self.0.write_options(writer, endian, args)
            }
        }

        impl StaticByteSize for $name {
            const BYTE_SIZE: usize = u64::BYTE_SIZE + u8::BYTE_SIZE + <$type>::BYTE_SIZE;
        }
    };
}

number_property!(IntProperty, i32);
number_property!(FloatProperty, f32);
number_property!(DoubleProperty, f64);
