#![feature(seek_stream_len)]

pub mod byte_size;
pub mod suitebro;

// #[cfg(test)]
// mod tests {
//     use crate::suitebro::SuiteBro;
//     use binrw::BinReaderExt;
//     use std::error::Error;
//     use std::io::Cursor;

//     macro_rules! test_serialization {
//         ($name:ident, $ty:ty, $input:expr) => {
//             #[test]
//             fn $name() -> Result<(), Box<dyn Error>> {
//                 let input = $input;
//                 let mut reader = Cursor::new(&input);
//                 let value: $ty = reader.read_le()?;

//                 let json = serde_json::to_string(&value)?;
//                 let deserialized: $ty = serde_json::from_str(&json)?;

//                 assert_eq!(value, deserialized);

//                 Ok(())
//             }
//         };
//     }

//     test_serialization!(
//         test_serialize_suitbro,
//         SuiteBro,
//         include_bytes!("../assets/OneItem")
//     );
// }
