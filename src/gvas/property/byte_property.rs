use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::{
    byte_size::{ByteSize, StaticByteSize},
    gvas::types::FString,
};

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteProperty {
    #[br(temp)]
    #[bw(calc = self.value.byte_size() as u64)]
    size: u64,
    name: FString,
    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    seperator: u8,
    #[br(count = size)]
    value: Vec<u8>,
}

impl ByteSize for ByteProperty {
    fn byte_size(&self) -> usize {
        u64::BYTE_SIZE + self.name.byte_size() + u8::BYTE_SIZE + self.value.byte_size()
    }
}
