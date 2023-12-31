use std::{io, string};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("Cannot recognize {0}, {1}")]
    UnknownFolder(String, &'static str),
    #[error("IO Error")]
    IOError(#[from] io::Error),
    #[error("Invalid Config File")]
    InvalidFile(#[from] string::FromUtf8Error),
    #[error("Invalid json")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Unknown error")]
    Unknown,
}
