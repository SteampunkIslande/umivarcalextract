use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum UMIVarCalExtractError {
    #[error(transparent)]
    FileReadError(#[from] std::io::Error),

    #[error("Tag not found")]
    TagNotFound,

    #[error("Tag must be 2 characters")]
    TagLengthError,

    #[error("Either tag or umi_length must be provided")]
    NoUMIOptionProvided,

    #[error("Only one of tag or umi_length can be provided")]
    BothUMIOptionsProvided,

    #[error("Cannot extract {0} nucleotides from read")]
    UMIExtractionError(usize),
}
