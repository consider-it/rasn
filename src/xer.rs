extern crate alloc;

pub mod de;
pub mod enc;

const BIT_STRING_TYPE_TAG: &str = "BIT_STRING";
const BOOLEAN_TYPE_TAG: &str = "BOOLEAN";
const BOOLEAN_TRUE_TAG: &str = "true";
const BOOLEAN_FALSE_TAG: &str = "false";
const INTEGER_TYPE_TAG: &str = "INTEGER";
const NULL_TYPE_TAG: &str = "NULL";
const OBJECT_IDENTIFIER_TYPE_TAG: &str = "OBJECT_IDENTIFIER";
const OCTET_STRING_TYPE_TAG: &str = "OCTET_STRING";
const BMP_STRING_TYPE_TAG: &str = "BMPString";
const IA5_STRING_TYPE_TAG: &str = "IA5String";
const VISIBLE_STRING_TYPE_TAG: &str = "VisibleString";
const UTF8_STRING_TYPE_TAG: &str = "UTF8String";
const GENERAL_STRING_TYPE_TAG: &str = "GeneralString";
const PRINTABLE_STRING_TYPE_TAG: &str = "PrintableString";
const NUMERIC_STRING_TYPE_TAG: &str = "NumericString";
const GENERALIZED_TIME_TYPE_TAG: &str = "GeneralizedTime";
const UTC_TIME_TYPE_TAG: &str = "UTCTime";

/// Attempts to decode `T` from `input` using XER.
/// # Errors
/// Returns error specific to XER decoder if decoding is not possible.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::error::DecodeError> {
    T::decode(&mut de::Decoder::new(input)?)
}

/// Attempts to encode `value` to XER.
/// # Errors
/// Returns error specific to XER encoder if encoding is not possible.
pub fn encode<T: crate::Encode>(
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut encoder = enc::Encoder::new();
    value.encode(&mut encoder, T::IDENTIFIER)?;
    Ok(encoder.finish())
}
