use std::io::Cursor;

use binrw::{BinRead, BinWrite, VecArgs};
use serde::{Deserialize, Serialize};

use crate::byte_size::{ByteSize, StaticByteSize};
use crate::gvas::types::{FString, Quat, Vector, GUID};
use crate::suitebro::property_map::PropertyMap;

trait AdjustErrorPos {
    fn adjust_error_pos(self, offset: u64) -> Self;
}

impl AdjustErrorPos for binrw::error::Error {
    fn adjust_error_pos(self, offset: u64) -> Self {
        use binrw::Error::*;
        let offset = offset as u64;

        match self {
            BadMagic { pos, found } => BadMagic {
                pos: offset + pos,
                found,
            },
            AssertFail { pos, message } => AssertFail {
                pos: offset + pos,
                message,
            },
            Custom { pos, err } => Custom {
                pos: offset + pos,
                err,
            },
            NoVariantMatch { pos } => NoVariantMatch { pos: offset + pos },
            EnumErrors {
                pos,
                variant_errors,
            } => EnumErrors {
                pos: offset + pos,
                variant_errors,
            },
            a => a,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub name: FString,
    pub guid: GUID,
    pub unk_has_state: u32,
    pub steam_item_id: u64,
    #[serde(flatten)]
    pub tinyrick: Option<TinyRick>,
    pub rotation: Quat,
    pub position: Vector,
    pub scale: Vector,
}

impl BinRead for Item {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let name = FString::read_options(reader, endian, ())?;
        let guid = GUID::read_options(reader, endian, ())?;
        let unk_has_state = u32::read_options(reader, endian, ())?;
        let steam_item_id = u64::read_options(reader, endian, ())?;

        let tinyrick = if unk_has_state != 0 {
            let tinyrick_size = u32::read_options(reader, endian, ())?;
            let mut tinyrick_data = vec![0u8; tinyrick_size as usize];
            reader.read_exact(&mut tinyrick_data)?;
            let mut cursor = Cursor::new(tinyrick_data);
            let tinyrick = TinyRick::read_options(&mut cursor, endian, ()).map_err(|error| {
                let offset = reader.stream_position().unwrap() - tinyrick_size as u64;
                error.adjust_error_pos(offset)
            })?;

            if tinyrick_size as u64 != cursor.position() {
                println!(
                    "Warning: Garbage data detected at 0x{:x}",
                    reader.stream_position()?
                )
            }

            Some(tinyrick)
        } else {
            None
        };

        let rotation = Quat::read_options(reader, endian, ())?;
        let position = Vector::read_options(reader, endian, ())?;
        let scale = Vector::read_options(reader, endian, ())?;

        Ok(Self {
            name,
            guid,
            unk_has_state,
            steam_item_id,
            tinyrick,
            rotation,
            position,
            scale,
        })
    }
}

impl BinWrite for Item {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let tinyrick_size = self.tinyrick.byte_size() as u32;

        self.name.write_options(writer, endian, args)?;
        self.guid.write_options(writer, endian, args)?;
        self.unk_has_state.write_options(writer, endian, args)?;
        self.steam_item_id.write_options(writer, endian, args)?;
        tinyrick_size.write_options(writer, endian, args)?;
        self.tinyrick.write_options(writer, endian, args)?;
        self.rotation.write_options(writer, endian, args)?;
        self.position.write_options(writer, endian, args)?;
        self.scale.write_options(writer, endian, args)
    }
}

fn default_format_version() -> u32 {
    1
}

fn default_unreal_version() -> u32 {
    517
}

const TINYRICK_MAGIC: &[u8; 8] = b"tinyrick";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TinyRick {
    #[serde(skip_serializing)]
    #[serde(default = "default_format_version")]
    pub format_version: u32,
    #[serde(skip_serializing)]
    #[serde(default = "default_unreal_version")]
    pub unreal_version: u32,

    pub properties: PropertyMap,

    // another count?
    pub unk_count: u32,

    pub property_sections: Vec<PropertySection>,
}

impl BinRead for TinyRick {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
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
        let property_sections = if property_section_count > 0 {
            <Vec<PropertySection>>::read_options(
                reader,
                endian,
                VecArgs {
                    count: property_section_count as usize,
                    inner: (),
                },
            )?
        } else {
            vec![]
        };

        Ok(Self {
            format_version,
            unreal_version,
            unk_count,
            properties,
            property_sections,
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
        self.property_sections.write_options(writer, endian, args)
    }
}

impl ByteSize for TinyRick {
    fn byte_size(&self) -> usize {
        TINYRICK_MAGIC.byte_size()
            + self.format_version.byte_size()
            + self.unreal_version.byte_size()
            + self.properties.byte_size()
            + self.unk_count.byte_size()
            + u32::BYTE_SIZE
            + self.property_sections.byte_size()
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

impl ByteSize for PropertySection {
    fn byte_size(&self) -> usize {
        self.name.byte_size() + self.properties.byte_size() + self.unk.byte_size()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Items(Vec<Item>);

impl BinRead for Items {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let count = u32::read_options(reader, endian, args)?;
        let items = <Vec<Item>>::read_options(
            reader,
            endian,
            VecArgs {
                count: count as usize,
                inner: (),
            },
        )?;
        Ok(Self(items))
    }
}

impl BinWrite for Items {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let count = self.0.len() as u32;

        count.write_options(writer, endian, args)?;
        self.0.write_options(writer, endian, args)
    }
}
