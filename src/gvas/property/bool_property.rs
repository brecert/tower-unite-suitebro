use binrw::binrw;
use serde::{Deserialize, Serialize};

#[binrw]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct BoolProperty {
    #[br(temp, assert(size == 0))]
    #[bw(calc = 0)]
    pub size: u64,
    #[br(map = |val: u8| val > 0x00)]
    #[bw(map = |val| if *val { 1u8 } else { 0u8 })]
    pub value: bool,
    #[br(temp, assert(indicator == 0))]
    #[bw(calc = 0)]
    pub indicator: u8, // todo: assert is 0
}
