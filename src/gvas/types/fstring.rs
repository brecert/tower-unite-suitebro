use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use unreal_helpers::{UnrealReadExt, UnrealWriteExt};

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FString(
    /// The raw byte string.
    pub String,
);

impl FString {
    pub fn as_str(&self) -> &str {
        &self.0
        // match &self.0 {
        //     Some(value) => value.as_str(),
        //     None => return "",
        // }
    }
}
impl BinRead for FString {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        match reader.read_fstring() {
            Ok(string) => Ok(FString(string.unwrap_or_default())),
            Err(error) => {
                let pos = reader.stream_position()?;
                Err(binrw::Error::Custom {
                    pos,
                    err: Box::new(error),
                })
            }
        }
    }
}

impl BinWrite for FString {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let value = match self.0.as_str() {
            "" => None,
            value => Some(value),
        };
        match writer.write_fstring(value) {
            Ok(_) => Ok(()),
            Err(error) => {
                let pos = writer.stream_position()?;
                Err(binrw::Error::Custom {
                    pos,
                    err: Box::new(error),
                })
            }
        }
    }
}

impl From<&str> for FString {
    fn from(value: &str) -> Self {
        FString(String::from(value))
    }
}

// impl Deref for FString {
//     type Target = <String as Deref>::Target;

//     fn deref(&self) -> &Self::Target {
//         self.0.deref()
//     }
// }

// impl DerefMut for FString {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.0.deref_mut()
//     }
// }

// impl Borrow<str> for FString {
//     fn borrow(&self) -> &str {
//         self.0.borrow()
//     }
// }
