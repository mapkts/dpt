use crate::{Error, ErrorKind, Result};
use encoding::all::{GB18030, GBK, UTF_8};
use encoding::{DecoderTrap, Encoding};

/// Supported character encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeType {
    UTF8,
    GBK,
    GB18030,
}

/// Decodes a sequence of bytes encoded with either GBK, GB18030 or UTF-8.
pub fn decode(src: &[u8], encoding: EncodeType, dst: &mut String) -> Result<()> {
    dst.clear();
    match encoding {
        EncodeType::GB18030 => GB18030
            .decode_to(src, DecoderTrap::Strict, dst)
            .map_err(|e| Error::new(ErrorKind::Decode(e.to_owned().to_string()))),
        EncodeType::GBK => GBK
            .decode_to(src, DecoderTrap::Strict, dst)
            .map_err(|e| Error::new(ErrorKind::Decode(e.to_owned().to_string()))),
        EncodeType::UTF8 => UTF_8
            .decode_to(src, DecoderTrap::Strict, dst)
            .map_err(|e| Error::new(ErrorKind::Decode(e.to_owned().to_string()))),
    }
}
