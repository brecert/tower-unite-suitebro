use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::gvas::types::{FString, Quat, Vector, GUID};
use crate::suitebro::property_map::PropertyMap;

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub name: FString,
    pub guid: GUID,
    pub unk1: GUID,
    #[serde(flatten)]
    pub tinyrick: TinyRick,
}

fn default_unk_version_1() -> u32 {
    1
}

fn default_unk_version_2() -> u32 {
    517
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[brw(magic = b"tinyrick")]
pub struct TinyRick {
    /// may be some kind of version
    #[serde(skip_serializing)]
    #[serde(default = "default_unk_version_1")]
    pub unk_version_1: u32,
    /// may be some kind of version
    #[serde(skip_serializing)]
    #[serde(default = "default_unk_version_2")]
    pub unk_version_2: u32,

    pub properties: PropertyMap,

    // another count?
    pub unk_count: u32,

    #[br(temp)]
    #[bw(calc = property_sections.len() as u32)]
    pub property_section_count: u32,
    #[br(count = property_section_count)]
    pub property_sections: Vec<PropertySection>,

    pub rotation: Quat,
    pub position: Vector,
    pub scale: Vector,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PropertySection {
    pub name: FString,
    pub properties: PropertyMap,
    pub unk: u32,
}

impl BinRead for PropertySection {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(Self {
            name: <_>::read_options(reader, endian, args)?,
            properties: <_>::read_options(reader, endian, args)?,
            unk: <_>::read_options(reader, endian, args)?,
        })
    }
}

impl BinWrite for PropertySection {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        self.name.write_options(writer, endian, args)?;
        self.properties.write_options(writer, endian, args)?;
        self.unk.write_options(writer, endian, args)
    }
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Items {
    #[br(temp)]
    #[bw(calc = items.len() as u32)]
    pub count: u32,
    #[br(count = count)]
    pub items: Vec<Item>,
}