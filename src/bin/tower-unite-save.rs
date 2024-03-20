use std::path::PathBuf;
use std::{fs::File, path::Path};

use argh::FromArgs;
use binrw::{BinReaderExt, BinWriterExt};
use gvas_rs::suitebro::SuiteBro;

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
    let mut input_file = File::open(&input)?;
    let save: SuiteBro = input_file.read_le()?;

    let output_file = match overwrite {
        true => File::create(&output)?,
        false => File::create_new(&output)?,
    };
    serde_json::to_writer_pretty(&output_file, &save)?;

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
    let save: SuiteBro = serde_json::from_reader(&input_file)?;

    let mut output_file = match overwrite {
        true => File::create(&output)?,
        false => File::create_new(&output)?,
    };

    output_file.write_le(&save)?;

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
