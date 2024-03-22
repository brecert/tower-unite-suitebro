use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::gvas::property::{IntProperty, PropertyType, StructProperty};
use crate::gvas::types::{LinearColor, GUID};
use crate::suitebro::property_map::PropertyMap;

use super::StructType;

#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[br(map = |a: PropertyMap| a.try_into().unwrap() )]
#[bw(map = |a| PropertyMap::from(a))]
pub struct Colorable {
    pub color: LinearColor,
    pub dynamic_material_index: i32,
}

impl From<&Colorable> for PropertyMap {
    fn from(value: &Colorable) -> Self {
        let mut map = Self::default();
        map.0["Color"] = PropertyType::StructProperty(StructProperty {
            value: StructType::LinearColor(value.color),
            guid: GUID::default(),
        });
        map.0["DynamicMaterialIndex"] = PropertyType::IntProperty(IntProperty {
            value: value.dynamic_material_index,
        });
        map
    }
}

// todo: error
impl TryFrom<PropertyMap> for Colorable {
    type Error = ();

    fn try_from(map: PropertyMap) -> Result<Self, Self::Error> {
        match (&map.0["Color"], &map.0["DynamicMaterialIndex"]) {
            (
                PropertyType::StructProperty(StructProperty {
                    value: StructType::LinearColor(color),
                    ..
                }),
                PropertyType::IntProperty(IntProperty {
                    value: dynamic_material_index,
                }),
            ) => Ok(Colorable {
                color: *color,
                dynamic_material_index: *dynamic_material_index,
            }),
            _ => unreachable!(),
        }
    }
}
