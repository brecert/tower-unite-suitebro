use binrw::{binrw, binwrite, BinRead};
use serde::{Deserialize, Serialize};

use crate::byte_size::ByteSize;
use crate::gvas::types::{FString, GUID};
use crate::gvas::types::{LinearColor, Quat, Vector, Rotator};
use crate::suitebro::property_map::PropertyMap;

type SteamID = PropertyMap;
type PlayerTrustSaveData = PropertyMap;
type Colorable = PropertyMap;
type ItemPhysics = PropertyMap;
type Transform = PropertyMap;
type ItemSpawnDefaults = PropertyMap;
type WeatherManifestEntry = PropertyMap;
type ItemConnectionData = PropertyMap;
type SplineSaveData = PropertyMap;

#[binwrite]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StructType {
    // todo: builtin types
    LinearColor(LinearColor),
    Quat(Quat),
    Vector(Vector),
    GUID(GUID),
    Rotator(Rotator),

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
            Self::LinearColor(_) => "LinearColor".into(),
            Self::Quat(_) => "Quat".into(),
            Self::Vector(_) => "Vector".into(),
            Self::SteamID(_) => "SteamID".into(),
            Self::PlayerTrustSaveData(_) => "PlayerTrustSaveData".into(),
            Self::Colorable(_) => "Colorable".into(),
            Self::ItemPhysics(_) => "ItemPhysics".into(),
            Self::Transform(_) => "Transform".into(),
            Self::ItemSpawnDefaults(_) => "ItemSpawnDefaults".into(),
            Self::GUID(_) => "Guid".into(),
            Self::WeatherManifestEntry(_) => "WeatherManifestEntry".into(),
            Self::ItemConnectionData(_) => "ItemConnectionData".into(),
            Self::SplineSaveData(_) => "SplineSaveData".into(),
            Self::Rotator(_) => "Rotator".into(),
        }
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
            _ => Err(binrw::error::Error::AssertFail {
                pos: reader.stream_position()?,
                message: format!("No StructType variant for {:?}", args.0),
            }),
        }
    }
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StructProperty {
    #[br(temp)]
    #[bw(calc = self.value.byte_size() as u64)]
    pub size: u64,
    pub struct_type: FString,
    pub guid: GUID,
    #[br(temp, assert(seperator == 0))]
    #[bw(calc = 0)]
    pub seperator: u8,
    // pub key_name: FString,
    // #[br(args { ty: struct_type.as_str() })]
    #[br(args_raw = struct_type.clone())]
    pub value: StructType,
}
