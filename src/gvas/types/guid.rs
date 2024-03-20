use binrw::binrw;
use gvas::types::Guid;
use serde::{Deserialize, Serialize, Serializer};

fn serialize_guid<S: Serializer>(guid: &Guid, serializer: S) -> Result<S::Ok, S::Error> {
    if guid.is_zero() {
        serializer.serialize_str("00000000-0000-0000-0000-000000000000")
    } else {
        guid.serialize(serializer)
    }
}

#[binrw]
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GUID(
    #[br(map = Guid)]
    #[bw(map = |x| x.0)]
    #[serde(serialize_with = "serialize_guid")]
    pub Guid,
);
