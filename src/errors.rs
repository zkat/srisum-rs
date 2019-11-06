use std::io;

use ssri;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SrisumError {
    #[error("File failed integrity check: {0}")]
    IntegrityError(ssri::Error),
    #[error("Failed to parse integrity string")]
    ParseIntegrityError(#[from] ssri::Error),
    #[error("I/O Error")]
    Io(#[from] io::Error),
}
