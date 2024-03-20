use binrw::binrw;
use serde::{Deserialize, Serialize};

pub mod bool;
pub mod fstring;
pub mod guid;

pub use self::bool::Bool;
pub use self::fstring::FString;
pub use self::guid::GUID;

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Rotator {
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
}