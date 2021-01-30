//! Filetype conversions and character encodings.
mod xlsx2csv;
pub use self::xlsx2csv::xlsx2csv;

mod decode;
pub use self::decode::decode;
pub use self::decode::Encoding;
