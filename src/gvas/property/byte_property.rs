use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::{byte_size::ByteSize, gvas::types::FString};

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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
