use binrw::{BinRead, BinWrite};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    byte_size::ByteSize,
    gvas::{
        property::{Property, PropertyType},
        types::FString,
    },
};

#[derive(Debug, Default)]
pub struct PropertyList(Vec<Property>);

impl BinRead for PropertyList {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let mut properties = vec![];

        loop {
            let property = Property::read_options(reader, endian, args)?;
            if property.value.is_none() {
                break;
            }
            properties.push(property);
        }

        Ok(PropertyList(properties))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PropertyMap(
    // order matters, use indexmap
    pub IndexMap<String, PropertyType>,
);

impl BinRead for PropertyMap {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let list = PropertyList::read_options(reader, endian, ())?;
        let props = list
            .0
            .into_iter()
            .map_while(|item| item.value.map(|value| (item.name.0, value)));

        Ok(PropertyMap(IndexMap::from_iter(props)))
    }
}

impl BinWrite for PropertyMap {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        // todo: this may not be symmetrical
        for (name, value) in self.0.iter() {
            FString(name.clone()).write_options(writer, endian, ())?;
            value.write_options(writer, endian, ())?;
        }

        FString::from("None").write_options(writer, endian, ())?;

        Ok(())
    }
}

impl ByteSize for PropertyMap {
    fn byte_size(&self) -> usize {
        self.0
            .iter()
            .map(|(key, property)| FString::from(key.as_str()).byte_size() + property.byte_size())
            .sum::<usize>()
            + FString::from("None").byte_size()
    }
}
