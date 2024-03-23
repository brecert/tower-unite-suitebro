use std::io;

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::byte_size::{ByteSize, StaticByteSize};
use crate::gvas::types::{FString, GUID};

pub mod struct_type;
use struct_type::StructType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructProperty {
    #[serde(default)]
    #[serde(skip_serializing_if = "GUID::is_zero")]
    pub guid: GUID,
    #[serde(flatten)]
    pub value: StructType,
}

impl BinRead for StructProperty {
    type Args<'a> = ();

    fn read_options<R: io::Read + io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        // todo: use for transmuting for performance
        let size = u64::read_options(reader, endian, args)?;
        let struct_type = FString::read_options(reader, endian, args)?;
        let guid = GUID::read_options(reader, endian, args)?;
        let seperator = u8::read_options(reader, endian, args)?;
        let value = StructType::read_options(reader, endian, struct_type)?;
            
        assert_eq!(size, value.byte_size() as u64);
        assert_eq!(seperator, 0);

        Ok(StructProperty { guid, value })
    }
}

impl BinWrite for StructProperty {
    type Args<'a> = ();

    fn write_options<W: io::Write + io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let size = self.value.byte_size() as u64;
        let struct_type = self.value.type_name();
        let seperator = 0u8;

        size.write_options(writer, endian, args)?;
        struct_type.write_options(writer, endian, args)?;
        self.guid.write_options(writer, endian, args)?;
        seperator.write_options(writer, endian, args)?;
        self.value.write_options(writer, endian, args)?;
        Ok(())
    }
}

impl ByteSize for StructProperty {
    fn byte_size(&self) -> usize {
        u64::BYTE_SIZE
            + self.value.type_name().byte_size()
            + self.guid.byte_size()
            + u8::BYTE_SIZE
            + self.value.byte_size()
    }
}
