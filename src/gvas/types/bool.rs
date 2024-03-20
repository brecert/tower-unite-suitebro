use std::io::{Read, Seek, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Serialize)]
pub struct Bool(pub bool);

impl BinRead for Bool {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        u8::read_options(reader, endian, ()).map(|value| Self(value > 0))
    }
}

impl BinWrite for Bool {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        (self.0 as u8).write_options(writer, endian, ())
    }
}
