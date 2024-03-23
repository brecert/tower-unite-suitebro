use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::{
    byte_size::{ByteSize, StaticByteSize},
    gvas::types::FString,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumProperty {
    pub enum_type: FString,
    pub value: FString,
}

impl BinRead for EnumProperty {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let size = u64::read_options(reader, endian, args)?;
        let enum_type = FString::read_options(reader, endian, args)?;
        let seperator = u8::read_options(reader, endian, args)?;
        let value = FString::read_options(reader, endian, args)?;

        assert_eq!(seperator, 0);
        assert_eq!(size, value.byte_size() as u64);

        Ok(Self { enum_type, value })
    }
}

impl BinWrite for EnumProperty {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let size = self.value.byte_size() as u64;
        let seperator = 0u8;

        size.write_options(writer, endian, args)?;
        self.enum_type.write_options(writer, endian, args)?;
        seperator.write_options(writer, endian, args)?;
        self.value.write_options(writer, endian, args)
    }
}

impl ByteSize for EnumProperty {
    fn byte_size(&self) -> usize {
        u64::BYTE_SIZE + self.enum_type.byte_size() + u8::BYTE_SIZE + self.value.byte_size()
    }
}
