use binrw::{binrw, BinRead, BinWrite, VecArgs};
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

fn default_format_version() -> u32 {
    1
}

fn default_unreal_version() -> u32 {
    517
}

const TINYRICK_MAGIC: &[u8; 8] = b"tinyrick";

// #[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[brw(magic = b"tinyrick")]
pub struct TinyRick {
    /// may be some kind of version
    #[serde(skip_serializing)]
    #[serde(default = "default_format_version")]
    pub format_version: u32,
    /// may be some kind of version
    #[serde(skip_serializing)]
    #[serde(default = "default_unreal_version")]
    pub unreal_version: u32,

    pub properties: PropertyMap,

    // another count?
    pub unk_count: u32,

    // #[br(temp)]
    // #[bw(calc = property_sections.len() as u32)]
    // pub property_section_count: u32,
    // #[br(count = property_section_count)]
    pub property_sections: Vec<PropertySection>,

    pub rotation: Quat,
    pub position: Vector,
    pub scale: Vector,
}

impl BinRead for TinyRick {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        // magic
        let magic = <[u8; 8]>::read_options(reader, endian, args)?;

        if &magic != TINYRICK_MAGIC {
            Err(binrw::Error::BadMagic {
                pos: reader.stream_position()?,
                found: Box::new(magic),
            })?;
        }

        let format_version = u32::read_options(reader, endian, args)?;
        let unreal_version = u32::read_options(reader, endian, args)?;
        let properties = PropertyMap::read_options(reader, endian, args)?;
        let unk_count = u32::read_options(reader, endian, args)?;
        let property_section_count = u32::read_options(reader, endian, args)?;
        let property_sections = <Vec<PropertySection>>::read_options(
            reader,
            endian,
            VecArgs {
                count: property_section_count as usize,
                inner: (),
            },
        )?;
        let rotation = Quat::read_options(reader, endian, args)?;
        let position = Vector::read_options(reader, endian, args)?;
        let scale = Vector::read_options(reader, endian, args)?;

        Ok(Self {
            format_version,
            unreal_version,
            unk_count,
            properties,
            property_sections,
            rotation,
            position,
            scale,
        })
    }
}

impl BinWrite for TinyRick {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let property_section_count = self.property_sections.len() as u32;

        TINYRICK_MAGIC.write_options(writer, endian, args)?;
        self.format_version.write_options(writer, endian, args)?;
        self.unreal_version.write_options(writer, endian, args)?;
        self.properties.write_options(writer, endian, args)?;
        self.unk_count.write_options(writer, endian, args)?;
        property_section_count.write_options(writer, endian, args)?;
        self.rotation.write_options(writer, endian, args)?;
        self.position.write_options(writer, endian, args)?;
        self.scale.write_options(writer, endian, args)
    }
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
