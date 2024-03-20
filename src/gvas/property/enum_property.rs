use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::{byte_size::ByteSize, gvas::types::FString};

#[binrw]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumProperty {
    #[br(temp)]
    #[bw(calc = self.value.byte_size() as u64)]
    pub size: u64,
    pub enum_type: FString,
    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    pub seperator: u8,
    pub value: FString,
}
