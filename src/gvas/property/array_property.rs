use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::{
    byte_size::ByteSize,
    gvas::types::{Bool, FString, GUID},
};

use super::struct_property::StructType;

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[br(import { count: usize })]
pub struct ArrayStructProperty {
    // todo: is this accurate?
    pub field_name: FString,
    pub value_type: FString, // todo: assert same as ty
    #[br(temp)]
    #[bw(calc = self.values.byte_size() as u64)]
    pub struct_size: u64,
    pub struct_type: FString,
    pub struct_guid: GUID,
    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    pub seperator: u8,
    #[br(args { count: count, inner: struct_type.clone() })]
    pub values: Vec<StructType>,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[br(import { count: usize })]
pub struct ArrayBoolProperty {
    #[br(count = count)]
    pub values: Vec<Bool>,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[br(import { count: usize })]
pub struct ArrayStrProperty {
    #[br(count = count)]
    pub values: Vec<FString>,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[br(import { count: usize, ty: FString })]
pub enum ArrayValue {
    #[br(pre_assert(ty.as_str() == "StructProperty"))]
    StructProperty(#[br(args { count })] ArrayStructProperty),

    #[br(pre_assert(ty.as_str() == "BoolProperty"))]
    BoolProperty(#[br(args { count })] ArrayBoolProperty),

    #[br(pre_assert(ty.as_str() == "StrProperty"))]
    StrProperty(#[br(args {count })] ArrayStrProperty),
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

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ArrayProperty {
    #[br(temp)]
    #[bw(calc = 4 + self.value.byte_size() as u64)]
    pub size: u64,
    #[br(temp)]
    #[bw(calc = value.type_name())]
    pub array_type: FString,
    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    pub seperator: u8,
    #[br(temp)]
    #[bw(calc = value.count() as u32)]
    pub array_len: u32,
    #[br(args { count: array_len as usize, ty: array_type.clone() })]
    #[serde(flatten)]
    pub value: ArrayValue,
}
