use std::io;

use binrw::binrw;
use binrw::{BinRead, BinWrite};
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

pub use self::array_property::ArrayProperty;
pub use self::bool_property::BoolProperty;
pub use self::byte_property::ByteProperty;
pub use self::enum_property::EnumProperty;
pub use self::int_property::{DoubleProperty, FloatProperty, IntProperty};
pub use self::name_property::NameProperty;
pub use self::object_property::ObjectProperty;
pub use self::str_property::StrProperty;
pub use self::struct_property::StructProperty;

use crate::gvas::types::FString;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl PropertyType {
    fn type_name(self: &Self) -> &'static str {
        match self {
            Self::StructProperty(_) => "StructProperty",
            Self::ArrayProperty(_) => "ArrayProperty",
            Self::StrProperty(_) => "StrProperty",
            Self::BoolProperty(_) => "BoolProperty",
            Self::IntProperty(_) => "IntProperty",
            Self::FloatProperty(_) => "FloatProperty",
            Self::NameProperty(_) => "NameProperty",
            Self::EnumProperty(_) => "EnumProperty",
            Self::ByteProperty(_) => "ByteProperty",
            Self::ObjectProperty(_) => "ObjectProperty",
        }
    }
}

impl BinRead for PropertyType {
    type Args<'a> = ();

    fn read_options<R: io::Read + io::Seek>(
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

        let property_type = FString::read_options(reader, endian, args)?;

        match property_type.as_str() {
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
                message: format!("No PropertyType variant for {:?}", property_type),
            }),
        }
    }
}

// there are many ways to avoid implementing this manually but none as simple.
impl BinWrite for PropertyType {
    type Args<'a> = ();

    fn write_options<W: io::Write + io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: (),
    ) -> binrw::BinResult<()> {
        FString::from(self.type_name()).write_options(writer, endian, args)?;
        match self {
            Self::StructProperty(property) => property.write_options(writer, endian, args),
            Self::ArrayProperty(property) => property.write_options(writer, endian, args),
            Self::StrProperty(property) => property.write_options(writer, endian, args),
            Self::BoolProperty(property) => property.write_options(writer, endian, args),
            Self::IntProperty(property) => property.write_options(writer, endian, args),
            Self::FloatProperty(property) => property.write_options(writer, endian, args),
            Self::NameProperty(property) => property.write_options(writer, endian, args),
            Self::EnumProperty(property) => property.write_options(writer, endian, args),
            Self::ByteProperty(property) => property.write_options(writer, endian, args),
            Self::ObjectProperty(property) => property.write_options(writer, endian, args),
        }
    }
}

#[binrw]
#[derive(Debug)]
pub struct Property {
    pub name: FString,
    #[br(if(name.as_str() != "None"))]
    pub value: Option<PropertyType>,
}
