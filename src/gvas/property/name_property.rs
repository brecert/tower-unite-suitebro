use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::{
    byte_size::{ByteSize, StaticByteSize},
    gvas::types::FString,
};

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct NameProperty {
    #[br(temp)]
    #[bw(calc = self.value.byte_size() as u64)]
    size: u64,
    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    seperator: u8,
    value: FString,
}

impl ByteSize for NameProperty {
    fn byte_size(&self) -> usize {
        u64::BYTE_SIZE + u8::BYTE_SIZE + self.value.byte_size()
    }
}
