#![feature(seek_stream_len)]

pub mod byte_size;
pub mod suitebro;

#[cfg(test)]
mod tests {
    use crate::suitebro::{SuiteBro, get_tower_types};
    use std::error::Error;
    use std::io::Cursor;
    use uesave::{Readable, Writable};

    macro_rules! test_rw {
        ($name:ident, $ty:ty, $input:expr) => {
            #[test]
            fn $name() -> Result<(), Box<dyn Error>> {
                let input = $input;
                let mut reader = Cursor::new(&input);

                let mut output = vec![];
                let mut writer = Cursor::new(&mut output);
                let value =
                    uesave::Context::run_with_types(&get_tower_types(), &mut reader, |ctx| {
                        <$ty as Readable<_>>::read(ctx)
                    })
                    .expect("error reading");
                uesave::Context::run_with_types(&get_tower_types(), &mut writer, |ctx| {
                    <$ty as Writable<_>>::write(&value, ctx)
                })
                .expect("error writing");

                assert_eq!(&input[..], &output[..]);

                Ok(())
            }
        };
    }

    test_rw!(
        test_serialize_suitbro,
        SuiteBro,
        include_bytes!("../assets/OneItem")
    );
}
