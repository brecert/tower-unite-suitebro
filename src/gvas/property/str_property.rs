use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::{byte_size::ByteSize, gvas::types::FString};

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct StrProperty {
    #[br(temp)]
    #[bw(calc = self.value.byte_size() as u64)]
    pub size: u64,
    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    pub seperator: u8,
    pub value: FString,
}
