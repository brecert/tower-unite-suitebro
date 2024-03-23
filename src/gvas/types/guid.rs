use binrw::{BinRead, BinWrite};
use gvas::types::Guid;
use serde::{Deserialize, Serialize, Serializer};

use crate::byte_size::StaticByteSize;

fn serialize_guid<S: Serializer>(guid: &Guid, serializer: S) -> Result<S::Ok, S::Error> {
    if guid.is_zero() {
        serializer.serialize_str("00000000-0000-0000-0000-000000000000")
    } else {
        guid.serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GUID(#[serde(serialize_with = "serialize_guid")] pub Guid);

impl GUID {
    pub fn is_zero(self: &Self) -> bool {
        self.0.is_zero()
    }
}

impl BinRead for GUID {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(Self(Guid::new(<_>::read_options(reader, endian, args)?)))
    }
}

impl BinWrite for GUID {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        self.0 .0.write_options(writer, endian, args)
    }
}

impl StaticByteSize for GUID {
    const BYTE_SIZE: usize = 16;
}
