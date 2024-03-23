use std::io::BufWriter;
use std::path::PathBuf;
use std::{fs::File, path::Path};

use argh::FromArgs;
use binrw::io::BufReader;
use binrw::{BinReaderExt, BinWriterExt};
use tower_suitebro::suitebro::SuiteBro;

#[derive(FromArgs, PartialEq, Debug)]
/// Convert a save file to json
#[argh(subcommand, name = "to-json")]
pub struct ToJSONArgs {
    /// save file to convert to json
    #[argh(option, short = 'i')]
    input: PathBuf,

    /// output location for the json
    #[argh(option, short = 'o')]
    output: PathBuf,

    /// overwrite the output file?
    #[argh(switch, short = '!')]
    overwrite: bool,
}

pub fn to_json(input: &Path, output: &Path, overwrite: bool) -> anyhow::Result<()> {
    let input_file = File::open(&input)?;
    let mut reader = BufReader::new(input_file);
    let save: SuiteBro = reader.read_le()?;

    let output_file = match overwrite {
        true => File::create(&output)?,
        false => File::create_new(&output)?,
    };
    let writer = BufWriter::new(output_file);

    serde_json::to_writer_pretty(writer, &save)?;

    Ok(())
}

#[derive(FromArgs, PartialEq, Debug)]
/// Convert json to a save file
#[argh(subcommand, name = "to-save")]
pub struct ToSaveArgs {
    /// json to convert to save file
    #[argh(option, short = 'i')]
    input: PathBuf,

    /// output location for the save file
    #[argh(option, short = 'o')]
    output: PathBuf,

    /// overwrite the output file?
    #[argh(switch, short = '!')]
    overwrite: bool,
}

pub fn from_json(input: &Path, output: &Path, overwrite: bool) -> anyhow::Result<()> {
    let input_file = File::open(&input)?;
    let reader = BufReader::new(input_file);
    let save: SuiteBro = serde_json::from_reader(reader)?;

    let output_file = match overwrite {
        true => File::create(&output)?,
        false => File::create_new(&output)?,
    };

    let mut writer = BufWriter::new(output_file);
    writer.write_le(&save)?;

    Ok(())
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommand {
    ToJSON(ToJSONArgs),
    ToSave(ToSaveArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Root of args
pub struct Args {
    #[argh(subcommand)]
    subcommand: SubCommand,
}

pub fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    match args.subcommand {
        SubCommand::ToJSON(args) => to_json(&args.input, &args.output, args.overwrite),
        SubCommand::ToSave(args) => from_json(&args.input, &args.output, args.overwrite),
    }
}
