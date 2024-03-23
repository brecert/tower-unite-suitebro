use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::{
    byte_size::{ByteSize, StaticByteSize},
    gvas::types::FString,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct StrProperty(FString);

impl BinRead for StrProperty {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let size = u64::read_options(reader, endian, args)?;
        let seperator = u8::read_options(reader, endian, args)?;
        let value = FString::read_options(reader, endian, args)?;

        assert_eq!(size, value.byte_size() as u64);
        assert_eq!(seperator, 0);

        Ok(Self(value))
    }
}

impl BinWrite for StrProperty {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let size = self.0.byte_size() as u64;
        let seperator = 0u8;

        size.write_options(writer, endian, args)?;
        seperator.write_options(writer, endian, args)?;
        self.0.write_options(writer, endian, args)
    }
}

impl ByteSize for StrProperty {
    fn byte_size(&self) -> usize {
        u64::BYTE_SIZE + u8::BYTE_SIZE + self.0.byte_size()
    }
}
