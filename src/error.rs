use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WaduError {
    #[error("Got a GistError")]
    GistError,
    #[error("There was an IO issue")]
    IOError(#[from] io::Error),
}
