//! Utilities that empower DPT (Data Processing Toolkit for Material Department).
//!
//! The main entries of this library are several core modules, including:
//!
//! - [`st`]: Performs preset statistical aggregations for ST records.
//! - [`jde`]: In charges of connecting JDE and downloading data from JDE automatically.
//! - [`convert`]: Converts .xlsx to .csv and performs character encoding and decoding.
mod error;
mod logger;
mod reader;

pub mod convert;
pub mod iter;
pub mod jde;
pub mod st;

pub use crate::error::{Error, ErrorKind, Result};
pub use crate::logger::Logger;
pub use crate::reader::{CsvReader, CsvReaderOptions};
