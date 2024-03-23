use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::byte_size::ByteSize;
use crate::gvas::types::{FString, GUID};

pub mod struct_type;
use struct_type::StructType;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructProperty {
    #[br(temp)]
    #[bw(calc = self.value.byte_size() as u64)]
    pub size: u64,
    // might not always align but we're optimizing for usability not verbosity with potential accuracy
    #[br(temp)]
    #[bw(calc = self.value.type_name())]
    pub struct_type: FString,
    #[serde(default)]
    #[serde(skip_serializing_if = "GUID::is_zero")]
    pub guid: GUID,
    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    pub seperator: u8,
    // pub key_name: FString,
    // #[br(args { ty: struct_type.as_str() })]
    #[br(args_raw = struct_type.clone())]
    #[serde(flatten)]
    pub value: StructType,
}
