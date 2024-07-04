use umivarcalextract as lib;

pub mod cli;

use cli::parse;
use umivarcalextract::error::UMIVarCalExtractError;

fn main() -> Result<(), UMIVarCalExtractError> {
    let args = parse();
    let tag: Option<[u8; 2]> = match args.tag() {
        Some(ref tag) => Some(
            tag.as_bytes()
                .try_into()
                .map_err(|_| UMIVarCalExtractError::TagLengthError)?,
        ),
        None => None,
    };
    match (tag, args.umi_length()) {
        (Some(ref tag), None) => lib::add_mbc_from_tag(tag, args.input(), args.output()),
        (None, Some(umi_length)) => {
            lib::add_mbc_from_umi_length(umi_length, args.input(), args.output())
        }
        (None, None) => Err(UMIVarCalExtractError::NoUMIOptionProvided),
        (Some(_), Some(_)) => Err(UMIVarCalExtractError::BothUMIOptionsProvided),
    }
}
