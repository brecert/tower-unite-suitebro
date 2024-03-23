use binrw::{binwrite, BinRead};
use serde::{Deserialize, Serialize};

use crate::gvas::types::{FString, LinearColor, Quat, Rotator, Vector, GUID};
use crate::suitebro::property_map::PropertyMap;

pub mod colorable;
pub mod transform;

pub use colorable::Colorable;
pub use transform::Transform;

pub type WorkshopFile = u64;

pub type SteamID = PropertyMap;
pub type PlayerTrustSaveData = PropertyMap;
// type Colorable = PropertyMap;
pub type ItemPhysics = PropertyMap;
// pub type Transform = PropertyMap;
pub type ItemSpawnDefaults = PropertyMap;
pub type WeatherManifestEntry = PropertyMap;
pub type ItemConnectionData = PropertyMap;
pub type SplineSaveData = PropertyMap;

#[binwrite]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "struct_type", content = "value")]
pub enum StructType {
    // todo: builtin types
    LinearColor(LinearColor),
    Quat(Quat),
    Vector(Vector),
    GUID(GUID),
    Rotator(Rotator),

    // not sure where this fits
    WorkshopFile(WorkshopFile),

    // custom types
    SteamID(SteamID),
    PlayerTrustSaveData(PlayerTrustSaveData),
    Colorable(Colorable),
    ItemPhysics(ItemPhysics),
    Transform(Transform),
    ItemSpawnDefaults(ItemSpawnDefaults),
    WeatherManifestEntry(WeatherManifestEntry),
    ItemConnectionData(ItemConnectionData),
    SplineSaveData(SplineSaveData),
}

impl StructType {
    pub fn type_name(&self) -> FString {
        match &self {
            Self::LinearColor(_) => "LinearColor",
            Self::Quat(_) => "Quat",
            Self::Vector(_) => "Vector",
            Self::SteamID(_) => "SteamID",
            Self::PlayerTrustSaveData(_) => "PlayerTrustSaveData",
            Self::Colorable(_) => "Colorable",
            Self::ItemPhysics(_) => "ItemPhysics",
            Self::Transform(_) => "Transform",
            Self::ItemSpawnDefaults(_) => "ItemSpawnDefaults",
            Self::GUID(_) => "Guid",
            Self::WeatherManifestEntry(_) => "WeatherManifestEntry",
            Self::ItemConnectionData(_) => "ItemConnectionData",
            Self::SplineSaveData(_) => "SplineSaveData",
            Self::Rotator(_) => "Rotator",
            Self::WorkshopFile(_) => "WorkshopFile",
        }
        .into()
    }
}

impl BinRead for StructType {
    type Args<'a> = FString;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        macro_rules! read_struct_type {
            ($name:ident) => {{
                let value = $name::read_options(reader, endian, ())?;
                Ok(Self::$name(value))
            }};
        }

        match args.as_str() {
            "LinearColor" => read_struct_type!(LinearColor),
            "Quat" => read_struct_type!(Quat),
            "Vector" => read_struct_type!(Vector),
            "SteamID" => read_struct_type!(SteamID),
            "PlayerTrustSaveData" => read_struct_type!(PlayerTrustSaveData),
            "Colorable" => read_struct_type!(Colorable),
            "ItemPhysics" => read_struct_type!(ItemPhysics),
            "Transform" => read_struct_type!(Transform),
            "ItemSpawnDefaults" => read_struct_type!(ItemSpawnDefaults),
            "Guid" => read_struct_type!(GUID),
            "WeatherManifestEntry" => read_struct_type!(WeatherManifestEntry),
            "ItemConnectionData" => read_struct_type!(ItemConnectionData),
            "SplineSaveData" => read_struct_type!(SplineSaveData),
            "Rotator" => read_struct_type!(Rotator),
            "WorkshopFile" => read_struct_type!(WorkshopFile),
            _ => Err(binrw::error::Error::AssertFail {
                pos: reader.stream_position()?,
                message: format!("No StructType variant for {:?}", args.0),
            }),
        }
    }
}