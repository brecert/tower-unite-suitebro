
use serde::{Deserialize, Serialize};
use uesave::{Properties, Quat, Vector};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub name: String,
    pub guid: uuid::Uuid,
    // pub unk_has_state: u32,
    // if it's None then the id is 0 :)
    pub steam_item_id: u64,
    #[serde(flatten)]
    pub tinyrick: Option<TinyRick>,
    pub rotation: Quat,
    pub position: Vector,
    pub scale: Vector,
}

const TINYRICK_MAGIC: &[u8; 8] = b"tinyrick";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TinyRick {
    pub format_version: u32,
    pub unreal_version: u32,

    pub properties: Properties,

    // another count?
    pub unk_count: u32,

    pub property_sections: Vec<PropertySection>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PropertySection {
    pub name: String,
    pub properties: Properties,
    pub unk: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Items(Vec<Item>);