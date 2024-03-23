use binrw::{BinRead, BinWrite, VecArgs};
use serde::{Deserialize, Serialize};

use crate::{
    byte_size::{ByteSize, StaticByteSize},
    gvas::types::{Bool, FString, GUID},
};

use super::struct_property::struct_type::StructType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArrayStructProperty {
    // todo: is this accurate?
    pub field_name: FString,
    pub value_type: FString, // todo: assert same as ty
    pub struct_type: FString,
    #[serde(default)]
    #[serde(skip_serializing_if = "GUID::is_zero")]
    pub guid: GUID,
    pub values: Vec<StructType>,
}

impl BinRead for ArrayStructProperty {
    type Args<'a> = (usize,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let field_name = FString::read_options(reader, endian, ())?;
        let value_type = FString::read_options(reader, endian, ())?;
        let array_size = u64::read_options(reader, endian, ())?;
        let struct_type = FString::read_options(reader, endian, ())?;
        let guid = GUID::read_options(reader, endian, ())?;
        let seperator = u8::read_options(reader, endian, ())?;
        let values = <Vec<StructType>>::read_options(
            reader,
            endian,
            VecArgs {
                count: args.0,
                inner: struct_type.clone(),
            },
        )?;

        assert_eq!(seperator, 0);
        assert_eq!(array_size, values.byte_size() as u64);
        assert!(values.iter().all(|value| value.type_name() == struct_type));

        Ok(Self {
            field_name,
            value_type,
            struct_type,
            guid,
            values,
        })
    }
}

impl BinWrite for ArrayStructProperty {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let array_size = self.values.byte_size() as u64;
        let seperator = 0u8;

        assert!(self
            .values
            .iter()
            .all(|value| value.type_name() == self.struct_type));

        self.field_name.write_options(writer, endian, args)?;
        self.value_type.write_options(writer, endian, args)?;
        array_size.write_options(writer, endian, args)?;
        self.struct_type.write_options(writer, endian, args)?;
        self.guid.write_options(writer, endian, args)?;
        seperator.write_options(writer, endian, args)?;
        self.values.write_options(writer, endian, args)
    }
}

impl ByteSize for ArrayStructProperty {
    fn byte_size(&self) -> usize {
        self.field_name.byte_size()
            + self.value_type.byte_size()
            + u64::BYTE_SIZE
            + self.struct_type.byte_size()
            + self.guid.byte_size()
            + u8::BYTE_SIZE
            + self.values.byte_size()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct ArrayBoolProperty {
    pub values: Vec<Bool>,
}

impl BinRead for ArrayBoolProperty {
    type Args<'a> = (usize,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (count,): Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(Self {
            values: <_>::read_options(
                reader,
                endian,
                VecArgs {
                    count: count,
                    inner: (),
                },
            )?,
        })
    }
}

impl BinWrite for ArrayBoolProperty {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        self.values.write_options(writer, endian, args)
    }
}

impl ByteSize for ArrayBoolProperty {
    fn byte_size(&self) -> usize {
        self.values.byte_size()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct ArrayStrProperty {
    pub values: Vec<FString>,
}

impl BinRead for ArrayStrProperty {
    type Args<'a> = (usize,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (count,): Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(Self {
            values: <_>::read_options(
                reader,
                endian,
                VecArgs {
                    count: count,
                    inner: (),
                },
            )?,
        })
    }
}

impl BinWrite for ArrayStrProperty {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        self.values.write_options(writer, endian, args)
    }
}

impl ByteSize for ArrayStrProperty {
    fn byte_size(&self) -> usize {
        self.values.byte_size()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArrayValue {
    StructProperty(ArrayStructProperty),
    BoolProperty(ArrayBoolProperty),
    StrProperty(ArrayStrProperty),
}

impl ArrayValue {
    pub fn count(&self) -> usize {
        match self {
            Self::StructProperty(prop) => prop.values.len(),
            Self::BoolProperty(prop) => prop.values.len(),
            Self::StrProperty(prop) => prop.values.len(),
        }
    }

    pub fn type_name(&self) -> FString {
        match self {
            Self::StructProperty(_) => "StructProperty".into(),
            Self::BoolProperty(_) => "BoolProperty".into(),
            Self::StrProperty(_) => "StrProperty".into(),
        }
    }
}

impl BinRead for ArrayValue {
    type Args<'a> = (usize, FString);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (count, ty): Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let value = match ty.as_str() {
            "StructProperty" => Self::StructProperty(<_>::read_options(reader, endian, (count,))?),
            "BoolProperty" => Self::BoolProperty(<_>::read_options(reader, endian, (count,))?),
            "StrProperty" => Self::StrProperty(<_>::read_options(reader, endian, (count,))?),
            _ => Err(binrw::Error::NoVariantMatch {
                pos: reader.stream_position()?,
            })?,
        };
        Ok(value)
    }
}

impl BinWrite for ArrayValue {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        match self {
            Self::StructProperty(value) => value.write_options(writer, endian, args),
            Self::BoolProperty(value) => value.write_options(writer, endian, args),
            Self::StrProperty(value) => value.write_options(writer, endian, args),
        }
    }
}

impl ByteSize for ArrayValue {
    fn byte_size(&self) -> usize {
        match self {
            Self::StructProperty(property) => property.byte_size(),
            Self::BoolProperty(property) => property.byte_size(),
            Self::StrProperty(property) => property.byte_size(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArrayProperty(ArrayValue);

impl BinRead for ArrayProperty {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let _size = u64::read_options(reader, endian, args)?;
        let array_type = FString::read_options(reader, endian, args)?;
        let seperator = u8::read_options(reader, endian, args)?;
        let count = u32::read_options(reader, endian, args)?;
        let value = ArrayValue::read_options(reader, endian, (count as usize, array_type))?;

        // todo: assert size
        assert_eq!(seperator, 0);

        Ok(ArrayProperty(value))
    }
}

impl BinWrite for ArrayProperty {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let size = 4 + self.0.byte_size() as u64;
        let array_type = self.0.type_name();
        let seperator = 0u8;
        let count = self.0.count() as u32;

        size.write_options(writer, endian, args)?;
        array_type.write_options(writer, endian, args)?;
        seperator.write_options(writer, endian, args)?;
        count.write_options(writer, endian, args)?;
        self.0.write_options(writer, endian, args)
    }
}

impl ByteSize for ArrayProperty {
    fn byte_size(&self) -> usize {
        u64::BYTE_SIZE
            + self.0.type_name().byte_size()
            + u8::BYTE_SIZE
            + u32::BYTE_SIZE
            + self.0.byte_size()
    }
}
