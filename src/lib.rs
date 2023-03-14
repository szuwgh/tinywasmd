mod binary;
mod util;

use crate::util::error::WasmError;

#[macro_export]
macro_rules! format_err {
    ($offset:expr, $($arg:tt)*) => {
        WasmError::BinaryReaderError( format_args!($($arg)*).to_string() )
    }
}

#[macro_export]
macro_rules! reader_bail {
    ($($arg:tt)*) => {return Err(format_err!($($arg)*))};
}

struct WASM {}
