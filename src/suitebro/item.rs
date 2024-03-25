use std::io::{Cursor, Read, Seek, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use serde::{Deserialize, Serialize};
use uesave::{
    read_array, read_properties_until_none, write_properties_none_terminated, write_string,
    Properties, Quat, Readable, Vector, Writable,
};

use super::PropertyList;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub name: String,
    pub guid: uuid::Uuid,
    // pub unk_has_state: u32,
    // if it's None then the id is 0 :)
    pub steam_item_id: u64,
    #[serde(flatten)]
    pub tinyrick: Option<TinyRick>,
    pub rotation: Quat,
    pub position: Vector,
    pub scale: Vector,
}

impl<R: Read + Seek> Readable<R> for Item {
    fn read(reader: &mut uesave::Context<R>) -> uesave::TResult<Self> {
        let name = uesave::read_string(reader)?;
        let guid = uuid::Uuid::read(reader)?;
        let unk_has_state = reader.read_u32::<LE>()? != 0;
        let steam_item_id = reader.read_u64::<LE>()?;

        let tinyrick = if unk_has_state {
            let tinyrick_size = reader.read_u32::<LE>()?;

            let mut buf = vec![0u8; tinyrick_size as usize];
            reader.read_exact(&mut buf)?;

            Some(reader.stream(&mut Cursor::new(buf), TinyRick::read)?)
        } else {
            None
        };

        let rotation = Quat::read(reader)?;
        let position = Vector::read(reader)?;
        let scale = Vector::read(reader)?;

        Ok(Item {
            name,
            guid,
            steam_item_id,
            tinyrick,
            rotation,
            position,
            scale,
        })
    }
}

impl<R: Write + Seek> Writable<R> for Item {
    fn write(&self, writer: &mut uesave::Context<R>) -> uesave::TResult<()> {
        write_string(writer, &self.name)?;
        self.guid.write(writer)?;
        writer.write_u32::<LE>(if self.tinyrick.is_none() { 0u32 } else { 1u32 })?;
        writer.write_u64::<LE>(self.steam_item_id)?;
        if let Some(tinyrick) = &self.tinyrick {
            let mut buf: Vec<u8> = vec![];
            let mut cursor = Cursor::new(&mut buf);
            writer.stream(&mut cursor, |ctx| tinyrick.write(ctx))?;

            writer.write_u32::<LE>(buf.len() as u32)?;
            writer.write_all(&buf)?;
        }
        self.rotation.write(writer)?;
        self.position.write(writer)?;
        self.scale.write(writer)?;
        Ok(())
    }
}

const TINYRICK_MAGIC: &[u8; 8] = b"tinyrick";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TinyRick {
    pub format_version: u32,
    pub unreal_version: u32,

    pub properties: Properties,

    // another count?
    // never seen above 0, we'll have be 0 for now
    // it could also be the ending like for other property lists..
    // pub unk_count: u32,
    pub property_sections: Vec<PropertyList>,
}

impl<R: Read + Seek> Readable<R> for TinyRick {
    fn read(reader: &mut uesave::Context<R>) -> uesave::TResult<Self> {
        let magic = reader.read_u64::<LE>()?;
        if magic != u64::from_le_bytes(*TINYRICK_MAGIC) {
            return Err(uesave::Error::BadMagic(
                String::from_utf8_lossy(&magic.to_le_bytes()).to_string(),
            ));
        }

        let format_version = reader.read_u32::<LE>()?;
        let unreal_version = reader.read_u32::<LE>()?;
        let properties = read_properties_until_none(reader)?;
        let _unknown_count = reader.read_u32::<LE>()?;
        let actor_count = reader.read_u32::<LE>()?;
        let actors = read_array(actor_count, reader, PropertyList::read)?;

        Ok(TinyRick {
            format_version,
            unreal_version,
            properties,
            property_sections: actors,
        })
    }
}

impl<R: Write + Seek> Writable<R> for TinyRick {
    fn write(&self, writer: &mut uesave::Context<R>) -> uesave::TResult<()> {
        writer.write_all(TINYRICK_MAGIC)?;
        writer.write_u32::<LE>(self.format_version)?;
        writer.write_u32::<LE>(self.unreal_version)?;
        write_properties_none_terminated(writer, &self.properties)?;
        writer.write_u32::<LE>(0)?; // unknown count
        writer.write_u32::<LE>(self.property_sections.len() as u32)?;
        for actor in &self.property_sections {
            actor.write(writer)?;
        }
        Ok(())
    }
}
