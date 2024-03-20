use binrw::BinRead;
use binrw::{binrw, binwrite};
use serde::{Deserialize, Serialize};

pub mod array_property;
pub mod bool_property;
pub mod byte_property;
pub mod enum_property;
pub mod int_property;
pub mod name_property;
pub mod object_property;
pub mod str_property;
pub mod struct_property;

use self::array_property::ArrayProperty;
use self::bool_property::BoolProperty;
use self::byte_property::ByteProperty;
use self::enum_property::EnumProperty;
use self::int_property::{FloatProperty, IntProperty};
use self::name_property::NameProperty;
use self::object_property::ObjectProperty;
use self::str_property::StrProperty;
use self::struct_property::StructProperty;

use crate::gvas::types::FString;

#[binwrite]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PropertyType {
    StructProperty(StructProperty),
    ArrayProperty(ArrayProperty),
    StrProperty(StrProperty),
    BoolProperty(BoolProperty),
    IntProperty(IntProperty),
    FloatProperty(FloatProperty),
    NameProperty(NameProperty),
    EnumProperty(EnumProperty),
    ByteProperty(ByteProperty),
    ObjectProperty(ObjectProperty),
}

impl BinRead for PropertyType {
    type Args<'a> = FString;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        macro_rules! read_property_type {
            ($name:ident) => {{
                let value = $name::read_options(reader, endian, ())?;
                Ok(Self::$name(value))
            }};
        }

        match args.as_str() {
            "StructProperty" => read_property_type!(StructProperty),
            "ArrayProperty" => read_property_type!(ArrayProperty),
            "StrProperty" => read_property_type!(StrProperty),
            "BoolProperty" => read_property_type!(BoolProperty),
            "IntProperty" => read_property_type!(IntProperty),
            "FloatProperty" => read_property_type!(FloatProperty),
            "NameProperty" => read_property_type!(NameProperty),
            "EnumProperty" => read_property_type!(EnumProperty),
            "ByteProperty" => read_property_type!(ByteProperty),
            "ObjectProperty" => read_property_type!(ObjectProperty),
            _ => Err(binrw::error::Error::AssertFail {
                pos: reader.stream_position()?,
                message: format!("No PropertyType variant for {:?}", args.0),
            }),
        }
    }
}

// todo: debug inner value only?
#[binrw]
#[derive(Debug, Serialize, PartialEq)]
pub struct PropertyValue {
    #[serde(skip_serializing)]
    pub property_type: FString,
    #[br(args_raw = property_type.clone())]
    #[serde(flatten)]
    pub value: PropertyType,
}

impl<'de> Deserialize<'de> for PropertyValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = PropertyType::deserialize(deserializer)?;
        let property_type = match value {
            PropertyType::StructProperty(_) => "StructProperty",
            PropertyType::ArrayProperty(_) => "ArrayProperty",
            PropertyType::StrProperty(_) => "StrProperty",
            PropertyType::BoolProperty(_) => "BoolProperty",
            PropertyType::IntProperty(_) => "IntProperty",
            PropertyType::FloatProperty(_) => "FloatProperty",
            PropertyType::NameProperty(_) => "NameProperty",
            PropertyType::EnumProperty(_) => "EnumProperty",
            PropertyType::ByteProperty(_) => "ByteProperty",
            PropertyType::ObjectProperty(_) => "ObjectProperty",
        };
        Ok(PropertyValue {
            value,
            property_type: property_type.into(),
        })
    }
}

#[binrw]
#[derive(Debug)]
pub struct Property {
    pub name: FString,
    #[br(if(name.as_str() != "None"))]
    pub value: Option<PropertyValue>,
}
