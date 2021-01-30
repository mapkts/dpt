use encoding_rs::{CoderResult, GB18030, GBK, UTF_8};

/// Supported character encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    UTF8,
    GBK,
    GB18030,
}

/// Decodes a sequence of bytes encoded with GBK, GB18030 or UTF-8, returns a `Cow<'_, str>`.
pub fn decode(src: &[u8], encoding: Encoding, dst: &mut String) {
    let mut decoder = match encoding {
        Encoding::GB18030 => GB18030.new_decoder(),
        Encoding::GBK => GBK.new_decoder(),
        Encoding::UTF8 => UTF_8.new_decoder_with_bom_removal(),
    };

    loop {
        let result = decoder.decode_to_string(src, dst, true).0;
        match result {
            CoderResult::InputEmpty => {
                break;
            }
            CoderResult::OutputFull => {
                dst.reserve(128);
            }
        }
    }
}
