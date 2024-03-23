use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::{byte_size::StaticByteSize, gvas::types::Bool};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct BoolProperty(bool);

impl BinRead for BoolProperty {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let size = u64::read_options(reader, endian, args)?;
        let value = Bool::read_options(reader, endian, args)?.0;
        let indicator = u8::read_options(reader, endian, args)?;

        assert!(size == 0);
        assert!(indicator == 0);

        Ok(Self(value))
    }
}

impl BinWrite for BoolProperty {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        Bool(self.0).write_options(writer, endian, args)
    }
}

impl StaticByteSize for BoolProperty {
    const BYTE_SIZE: usize = u8::BYTE_SIZE;
}