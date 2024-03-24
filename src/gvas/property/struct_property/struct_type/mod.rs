use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::byte_size::ByteSize;
use crate::gvas::types::{FString, LinearColor, Quat, Rotator, Vector, GUID};
use crate::suitebro::property_map::PropertyMap;

// pub mod colorable;
// pub mod transform;

// pub use colorable::Colorable;
// pub use transform::Transform;

pub type WorkshopFile = u64;

pub type SteamID = PropertyMap;
pub type PlayerTrustSaveData = PropertyMap;
pub type Colorable = PropertyMap;
pub type ItemPhysics = PropertyMap;
pub type Transform = PropertyMap;
pub type ItemSpawnDefaults = PropertyMap;
pub type WeatherManifestEntry = PropertyMap;
pub type ItemConnectionData = PropertyMap;
pub type SplineSaveData = PropertyMap;
pub type SkyVolumeSettings = PropertyMap;
pub type PostProcessVolumeSettings = PropertyMap;
pub type FogVolumeSettings = PropertyMap;

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
    SkyVolumeSettings(SkyVolumeSettings),
    PostProcessVolumeSettings(PostProcessVolumeSettings),
    FogVolumeSettings(FogVolumeSettings),
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
            Self::SkyVolumeSettings(_) => "SkyVolumeSettings",
            Self::PostProcessVolumeSettings(_) => "PostProcessVolumeSettings",
            Self::FogVolumeSettings(_) => "FogVolumeSettings",
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
            "SkyVolumeSettings" => read_struct_type!(SkyVolumeSettings),
            "PostProcessVolumeSettings" => read_struct_type!(PostProcessVolumeSettings),
            "FogVolumeSettings" => read_struct_type!(FogVolumeSettings),

            _ => Err(binrw::error::Error::AssertFail {
                pos: reader.stream_position()?,
                message: format!("No StructType variant for {:?}", args.0),
            }),
        }
    }
}

impl BinWrite for StructType {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        match self {
            Self::LinearColor(value) => value.write_options(writer, endian, args),
            Self::Quat(value) => value.write_options(writer, endian, args),
            Self::Vector(value) => value.write_options(writer, endian, args),
            Self::SteamID(value) => value.write_options(writer, endian, args),
            Self::PlayerTrustSaveData(value) => value.write_options(writer, endian, args),
            Self::Colorable(value) => value.write_options(writer, endian, args),
            Self::ItemPhysics(value) => value.write_options(writer, endian, args),
            Self::Transform(value) => value.write_options(writer, endian, args),
            Self::ItemSpawnDefaults(value) => value.write_options(writer, endian, args),
            Self::GUID(value) => value.write_options(writer, endian, args),
            Self::WeatherManifestEntry(value) => value.write_options(writer, endian, args),
            Self::ItemConnectionData(value) => value.write_options(writer, endian, args),
            Self::SplineSaveData(value) => value.write_options(writer, endian, args),
            Self::Rotator(value) => value.write_options(writer, endian, args),
            Self::WorkshopFile(value) => value.write_options(writer, endian, args),
            Self::SkyVolumeSettings(value) => value.write_options(writer, endian, args),
            Self::PostProcessVolumeSettings(value) => value.write_options(writer, endian, args),
            Self::FogVolumeSettings(value) => value.write_options(writer, endian, args),
        }
    }
}

impl ByteSize for StructType {
    fn byte_size(&self) -> usize {
        match self {
            Self::LinearColor(value) => value.byte_size(),
            Self::Quat(value) => value.byte_size(),
            Self::Vector(value) => value.byte_size(),
            Self::SteamID(value) => value.byte_size(),
            Self::PlayerTrustSaveData(value) => value.byte_size(),
            Self::Colorable(value) => value.byte_size(),
            Self::ItemPhysics(value) => value.byte_size(),
            Self::Transform(value) => value.byte_size(),
            Self::ItemSpawnDefaults(value) => value.byte_size(),
            Self::GUID(value) => value.byte_size(),
            Self::WeatherManifestEntry(value) => value.byte_size(),
            Self::ItemConnectionData(value) => value.byte_size(),
            Self::SplineSaveData(value) => value.byte_size(),
            Self::Rotator(value) => value.byte_size(),
            Self::WorkshopFile(value) => value.byte_size(),
            Self::SkyVolumeSettings(value) => value.byte_size(),
            Self::PostProcessVolumeSettings(value) => value.byte_size(),
            Self::FogVolumeSettings(value) => value.byte_size(),
        }
    }
}
