use clap::Parser;

use clap::crate_authors;
use clap::crate_description;

use std::path::PathBuf;
use umivarcalextract as lib;
use umivarcalextract::error::UMIVarCalExtractError;

/// Simple program that writes MBC to the end of the read name, either from a predefined UMI length or from a tag
#[derive(Parser, Debug)]
#[command(version, author = crate_authors!("\n"), about = crate_description!())]
struct Args {
    //Input BAM file (positional)
    #[clap(help = "Input BAM file")]
    input: PathBuf,

    //Output BAM file (positional)
    #[clap(help = "Output BAM file")]
    output: PathBuf,

    //Tag name to extract MBC from
    #[clap(
        short,
        long,
        help = "Tag name to extract MBC from. Mutually exclusive with -l/--umi-length option"
    )]
    tag: Option<String>,

    //Length of UMI to extract into read name
    #[clap(
        short = 'l',
        long,
        help = "Length of UMI to extract into read name. Mutually exclusive with -t/--tag option"
    )]
    umi_length: Option<usize>,
}

fn main() -> Result<(), UMIVarCalExtractError> {
    let args = Args::parse();
    let tag: Option<[u8; 2]> = match args.tag {
        Some(ref tag) => Some(
            tag.as_bytes()
                .try_into()
                .map_err(|_| UMIVarCalExtractError::TagLengthError)?,
        ),
        None => None,
    };
    match (tag, args.umi_length) {
        (Some(ref tag), None) => lib::add_mbc_from_tag(tag, &args.input, &args.output),
        (None, Some(umi_length)) => {
            lib::add_mbc_from_umi_length(umi_length, &args.input, &args.output)
        }
        (None, None) => Err(UMIVarCalExtractError::NoUMIOptionProvided),
        (Some(_), Some(_)) => Err(UMIVarCalExtractError::BothUMIOptionsProvided),
    }
}
