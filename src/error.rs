use std::io;
use thiserror::Error;

pub type GfsResult<T> = Result<T, GfsError>;

#[derive(Error, Debug)]
pub enum GfsError {
    #[error("IO Error")]
    IoError(#[from] io::Error),
    #[error("The specified entry was not found")]
    EntryNotFound,
    #[error("Filesystem Error Occurred: {0}")]
    Other(String),
}
