use std::fmt;
use std::io;
use std::io::Error as IOError;
use thiserror::Error;
pub type WasmResult<T> = Result<T, WasmError>;

#[derive(Error, Debug)]
pub enum WasmError {
    #[error("Unexpected: {0}, {1}")]
    UnexpectIO(String, io::Error),
    #[error("Unexpected: {0}")]
    Unexpected(String),
    #[error("unexpected end of magic header")]
    UnexpectedMagic,
    #[error("unexpected end of binary version")]
    UnexpectedVersion,
    #[error("binary reader error {0}")]
    BinaryReaderError(String),
}

impl From<&str> for WasmError {
    fn from(e: &str) -> Self {
        WasmError::Unexpected(e.to_string())
    }
}

impl From<(&str, io::Error)> for WasmError {
    fn from(e: (&str, io::Error)) -> Self {
        WasmError::UnexpectIO(e.0.to_string(), e.1)
    }
}

impl From<String> for WasmError {
    fn from(e: String) -> Self {
        WasmError::Unexpected(e)
    }
}

impl From<IOError> for WasmError {
    fn from(e: IOError) -> Self {
        WasmError::Unexpected(e.to_string())
    }
}

impl From<WasmError> for String {
    fn from(e: WasmError) -> Self {
        format!("{}", e)
    }
}
