use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::gvas::property::{PropertyType, StructProperty};
use crate::gvas::types::{Quat, Vector, GUID};
use crate::suitebro::property_map::PropertyMap;

use super::StructType;

#[binrw]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[br(map = |a: PropertyMap| a.try_into().unwrap() )]
#[bw(map = |a| PropertyMap::from(a))]
pub struct Transform {
    pub rotation: Quat,
    pub translation: Vector,
    pub scale: Vector,
}

impl<'a> From<&'a PropertyType> for &'a StructProperty {
    fn from(value: &'a PropertyType) -> Self {
        match value {
            PropertyType::StructProperty(value) => value,
            _ => unreachable!(),
        }
    }
}

impl<'a> From<&'a StructProperty> for &'a StructType {
    fn from(value: &'a StructProperty) -> Self {
        &value.value
    }
}

impl<'a> From<&'a StructType> for &'a Quat {
    fn from(value: &'a StructType) -> Self {
        match value {
            StructType::Quat(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<&Transform> for PropertyMap {
    fn from(value: &Transform) -> Self {
        let mut map = Self::default();

        map.0["Rotation"] = PropertyType::StructProperty(StructProperty {
            value: StructType::Quat(value.rotation),
            guid: GUID::default(),
        });

        map.0["Translation"] = PropertyType::StructProperty(StructProperty {
            value: StructType::Vector(value.translation),
            guid: GUID::default(),
        });

        map.0["Scale3D"] = PropertyType::StructProperty(StructProperty {
            value: StructType::Vector(value.scale),
            guid: GUID::default(),
        });

        map
    }
}

// todo: error
impl TryFrom<PropertyMap> for Transform {
    type Error = ();

    fn try_from(map: PropertyMap) -> Result<Self, Self::Error> {
        match (&map.0["Rotation"], &map.0["Translation"], &map.0["Scale3D"]) {
            (
                PropertyType::StructProperty(StructProperty {
                    value: StructType::Quat(rotation),
                    ..
                }),
                PropertyType::StructProperty(StructProperty {
                    value: StructType::Vector(translation),
                    ..
                }),
                PropertyType::StructProperty(StructProperty {
                    value: StructType::Vector(scale),
                    ..
                }),
            ) => Ok(Self {
                rotation: *rotation,
                translation: *translation,
                scale: *scale,
            }),
            _ => unreachable!(),
        }
    }
}