use binrw::{BinRead, BinWrite, VecArgs};
use serde::{Deserialize, Serialize};

use crate::{
    byte_size::{ByteSize, StaticByteSize},
    gvas::types::FString,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteProperty {
    name: FString,
    value: Vec<u8>,
}

impl BinRead for ByteProperty {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let size = u64::read_options(reader, endian, args)?;
        let name = FString::read_options(reader, endian, args)?;
        let seperator = u8::read_options(reader, endian, args)?;
        let value = <Vec<u8>>::read_options(
            reader,
            endian,
            VecArgs {
                count: size as usize,
                inner: (),
            },
        )?;

        assert_eq!(seperator, 0);
        assert_eq!(size, value.byte_size() as u64);

        Ok(Self { name, value })
    }
}

impl BinWrite for ByteProperty {
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
        self.name.write_options(writer, endian, args)?;
        seperator.write_options(writer, endian, args)?;
        self.value.write_options(writer, endian, args)
    }
}

impl ByteSize for ByteProperty {
    fn byte_size(&self) -> usize {
        u64::BYTE_SIZE + self.name.byte_size() + u8::BYTE_SIZE + self.value.byte_size()
    }
}
