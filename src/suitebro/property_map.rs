use binrw::{binrw, until, BinRead, BinWrite};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::gvas::{
    property::{Property, PropertyValue},
    types::FString,
};

#[binrw]
#[derive(Debug, Default)]
pub struct PropertyList(
    #[br(parse_with = until(|property: &Property| property.value.is_none()))] Vec<Property>,
);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PropertyMap(
    // order matters, use indexmap
    pub IndexMap<String, PropertyValue>,
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
