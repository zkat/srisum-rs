use std::io;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum SrisumError {
    #[error("File failed integrity check: {0}")]
    IntegrityError(ssri::Error),
    #[error("Failed to parse integrity string")]
    ParseIntegrityError(#[from] ssri::Error),
    #[error("I/O Error")]
    Io(#[from] io::Error),
}
