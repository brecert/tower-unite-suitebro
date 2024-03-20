use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::gvas::types::{FString, Quat, Vector, GUID};

use super::property_map::PropertyMap;
// use crate::suitebro::property_map::PropertyMap;

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub name: FString,
    pub guid: GUID,
    pub unk1: GUID,
    #[serde(flatten)]
    pub tinyrick: TinyRick,
    pub unk2: [u8; 8],
    pub rotation: Quat,
    pub position: Vector,
    pub scale: Vector,
}

fn default_unk_version_1() -> u32 {
    1
}

fn default_unk_version_2() -> u32 {
    517
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, PartialEq)]
#[brw(magic = b"tinyrick")]
pub struct TinyRick {
    /// may be some kind of version
    #[serde(skip_serializing)]
    #[serde(default = "default_unk_version_1")]
    pub unk_version_1: u32,
    /// may be some kind of version
    #[serde(skip_serializing)]
    #[serde(default = "default_unk_version_2")]
    pub unk_version_2: u32,

    pub properties: PropertyMap,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Items {
    #[br(temp)]
    #[bw(calc = items.len() as u32)]
    pub count: u32,
    #[br(count = count)]
    pub items: Vec<Item>,
}
