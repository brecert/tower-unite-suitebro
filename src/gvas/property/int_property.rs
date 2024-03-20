use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::byte_size::ByteSize;

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct IntProperty {
    #[br(temp)]
    #[bw(calc = self.value.byte_size() as u64)]
    pub size: u64,

    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    pub seperator: u8,

    pub value: i32,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FloatProperty {
    #[br(temp)]
    #[bw(calc = self.value.byte_size() as u64)]
    pub size: u64,

    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    pub seperator: u8,

    pub value: f32,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DoubleProperty {
    #[br(temp)]
    #[bw(calc = self.value.byte_size() as u64)]
    pub size: u64,

    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    pub seperator: u8,

    pub value: f64,
}
