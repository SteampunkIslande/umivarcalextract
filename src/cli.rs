use clap::Command;
use clap::CommandFactory;
use clap::Parser;

use clap::crate_authors;
use clap::crate_description;

use std::path::PathBuf;

/// Simple program that writes MBC to the end of the read name, either from a predefined UMI length or from a tag
#[derive(Parser, Debug)]
#[command(version, author = crate_authors!("\n"), about = crate_description!())]
pub struct Args {
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

impl Args {
    pub fn input(&self) -> &PathBuf {
        &self.input
    }

    pub fn output(&self) -> &PathBuf {
        &self.output
    }

    pub fn tag(&self) -> &Option<String> {
        &self.tag
    }

    pub fn umi_length(&self) -> Option<usize> {
        self.umi_length
    }
}

pub fn build_cli() -> Command {
    Args::command()
}

pub fn parse() -> Args {
    Args::parse()
}
