//! # Encoding XER.
use core::{
    borrow::Borrow, fmt::Write, ops::{Deref, Not}
};

use crate::{
    alloc::{
        string::{String, ToString},
        vec::Vec,
    },
    types::{Any, BitStr, Enumerated, GeneralizedTime, UtcTime},
    xer::{
        BIT_STRING_TYPE_TAG, BMP_STRING_TYPE_TAG, GENERALIZED_TIME_TYPE_TAG,
        GENERAL_STRING_TYPE_TAG, IA5_STRING_TYPE_TAG, INTEGER_TYPE_TAG, NULL_TYPE_TAG,
        NUMERIC_STRING_TYPE_TAG, OBJECT_IDENTIFIER_TYPE_TAG, OCTET_STRING_TYPE_TAG,
        PRINTABLE_STRING_TYPE_TAG, UTC_TIME_TYPE_TAG, UTF8_STRING_TYPE_TAG,
        VISIBLE_STRING_TYPE_TAG,
    },
};
use alloc::borrow::Cow;
use num_bigint::BigInt;
use xml_no_std::{
    attribute::Attribute, name::Name, namespace::Namespace, writer::XmlEvent, EventWriter,
    ParserConfig,
};

use crate::error::{EncodeError, XerEncodeErrorKind};

use super::{BOOLEAN_FALSE_TAG, BOOLEAN_TRUE_TAG, BOOLEAN_TYPE_TAG};

macro_rules! wrap_in_tags {
    ($this:ident, $tag:ident, $inner:ident, $($args:expr)*) => {{
        let xml_tag = $this.field_tag_stack.pop().unwrap_or($tag);
        $this.write(XmlEvent::StartElement {
            name: Name::local(xml_tag),
            attributes: Cow::Borrowed(&[]),
            namespace: Namespace::empty().borrow(),
        })?;
        $this.$inner($($args),*)?;
        $this.write(XmlEvent::EndElement {
            name: Some(Name::local(xml_tag)),
        })?;
        Ok(())
    }};
}

macro_rules! try_wrap_in_tags {
    ($this:ident, $inner:ident, $($args:expr)*) => {{
        let xml_tag = $this.field_tag_stack
            .pop()
            .ok_or_else(|| XerEncodeErrorKind::FieldName)?;
        $this.write(XmlEvent::StartElement {
            name: Name::local(xml_tag),
            attributes: Cow::Borrowed(&[]),
            namespace: Namespace::empty().borrow(),
        })?;
        $this.$inner($($args),*)?;
        $this.write(XmlEvent::EndElement {
            name: Some(Name::local(xml_tag)),
        })?;
        Ok(())
    }};
}

pub struct Encoder {
    field_tag_stack: Vec<&'static str>,
    writer: EventWriter,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            writer: xml_no_std::EmitterConfig::new()
                .write_document_declaration(false)
                .create_writer(),
            field_tag_stack: Vec::new(),
        }
    }

    pub fn finish(self) -> Vec<u8> {
        self.writer.into_inner().into_bytes()
    }

    fn write(&mut self, event: XmlEvent<'_>) -> Result<(), EncodeError> {
        self.writer.write(event).map_err(|e| {
            EncodeError::from(XerEncodeErrorKind::XmlEncodingError {
                upstream: e.to_string(),
            })
        })
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();

    type Error = EncodeError;

    fn codec(&self) -> crate::Codec {
        crate::Codec::Xer
    }

    fn encode_any(
        &mut self,
        __tag: crate::Tag,
        value: &crate::types::Any,
    ) -> Result<Self::Ok, Self::Error> {
        try_wrap_in_tags!(self, write_any, value)
    }

    fn encode_bool(&mut self, _tag: crate::Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(self, BOOLEAN_TYPE_TAG, write_bool, value)
    }

    fn encode_bit_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &crate::types::BitStr,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(self, BIT_STRING_TYPE_TAG, write_bitstring, value)
    }

    fn encode_enumerated<E: crate::types::Enumerated>(
        &mut self,
        _tag: crate::Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(self, write_enumerated, value)
    }

    fn encode_object_identifier(
        &mut self,
        _tag: crate::Tag,
        value: &[u32],
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            OBJECT_IDENTIFIER_TYPE_TAG,
            write_object_identifier,
            value
        )
    }

    fn encode_integer(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &num_bigint::BigInt,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(self, INTEGER_TYPE_TAG, write_integer, value)
    }

    fn encode_null(&mut self, _tag: crate::Tag) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(self, NULL_TYPE_TAG, write_null,)
    }

    fn encode_octet_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(self, OCTET_STRING_TYPE_TAG, write_octet_string, value)
    }

    fn encode_general_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &crate::types::GeneralString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            GENERAL_STRING_TYPE_TAG,
            write_string_type,
            &String::from_utf8(value.deref().to_vec()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_utf8_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &str,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(self, UTF8_STRING_TYPE_TAG, write_string_type, value)
    }

    fn encode_visible_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &crate::types::VisibleString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            VISIBLE_STRING_TYPE_TAG,
            write_string_type,
            &value.to_string()
        )
    }

    fn encode_ia5_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &crate::types::Ia5String,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            IA5_STRING_TYPE_TAG,
            write_string_type,
            &value.to_string()
        )
    }

    fn encode_printable_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &crate::types::PrintableString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            PRINTABLE_STRING_TYPE_TAG,
            write_string_type,
            &String::from_utf8(value.as_bytes().to_vec()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_numeric_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &crate::types::NumericString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            NUMERIC_STRING_TYPE_TAG,
            write_string_type,
            &String::from_utf8(value.as_bytes().to_vec()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_teletex_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &crate::types::TeletexString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bmp_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &crate::types::BmpString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            BMP_STRING_TYPE_TAG,
            write_string_type,
            &String::from_utf8(value.to_bytes()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_generalized_time(
        &mut self,
        _tag: crate::Tag,
        value: &crate::types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            GENERALIZED_TIME_TYPE_TAG,
            write_generalized_time,
            value
        )
    }

    fn encode_utc_time(
        &mut self,
        _tag: crate::Tag,
        value: &crate::types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(self, UTC_TIME_TYPE_TAG, write_utc_time, value)
    }

    fn encode_explicit_prefix<V: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_sequence<C, F>(
        &mut self,
        _tag: crate::Tag,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        while let Some(id) = C::FIELDS.reverse_iter().next_back() {
            self.field_tag_stack.push(id.name);
        }
    }

    fn encode_sequence_of<E: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        value: &[E],
        _constraints: crate::types::Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_set<C, F>(&mut self, _tag: crate::Tag, value: F) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        todo!()
    }

    fn encode_set_of<E: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        value: &crate::types::SetOf<E>,
        _constraints: crate::types::Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_some<E: crate::Encode>(&mut self, value: &E) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_some_with_tag_and_constraints<E: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_some(value)
    }

    fn encode_none<E: crate::Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        self.field_tag_stack.pop();
        Ok(())
    }

    fn encode_none_with_tag(&mut self, _tag: crate::Tag) -> Result<Self::Ok, Self::Error> {
        self.encode_none()
    }

    fn encode_choice<E: crate::Encode + crate::types::Choice>(
        &mut self,
        _constraints: crate::types::Constraints,
        _tag: crate::Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<crate::Tag, Self::Error>,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_extension_addition<E: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: E,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_extension_addition_group<E>(
        &mut self,
        value: Option<&E>,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: crate::Encode + crate::types::Constructed,
    {
        todo!()
    }
}

impl Encoder {
    fn write_bool(&mut self, value: bool) -> Result<(), EncodeError> {
        if value {
            self.write(XmlEvent::StartElement {
                name: Name::local(BOOLEAN_TRUE_TAG),
                attributes: Cow::Borrowed(&[]),
                namespace: Namespace::empty().borrow(),
            })?;
            self.write(XmlEvent::EndElement {
                name: Some(Name::local(BOOLEAN_TRUE_TAG)),
            })
        } else {
            self.write(XmlEvent::StartElement {
                name: Name::local(BOOLEAN_FALSE_TAG),
                attributes: Cow::Borrowed(&[]),
                namespace: Namespace::empty().borrow(),
            })?;
            self.write(XmlEvent::EndElement {
                name: Some(Name::local(BOOLEAN_FALSE_TAG)),
            })
        }
    }

    fn write_bitstring(&mut self, value: &BitStr) -> Result<(), EncodeError> {
        if value.is_empty() {
            Ok(())
        } else {
            self.write(XmlEvent::Characters(
                value
                    .iter()
                    .map(|bit| if *bit { '1' } else { '0' })
                    .collect::<String>()
                    .as_str(),
            ))
        }
    }

    fn write_enumerated<E: Enumerated>(&mut self, value: &E) -> Result<(), EncodeError> {
        self.write(XmlEvent::StartElement {
            name: Name::local(value.identifier()),
            attributes: Cow::Borrowed(&[]),
            namespace: Namespace::empty().borrow(),
        })?;
        self.write(XmlEvent::EndElement {
            name: Some(Name::local(value.identifier())),
        })
    }

    fn write_integer(&mut self, value: &BigInt) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(&value.to_str_radix(10)))
    }

    fn write_object_identifier(&mut self, value: &[u32]) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(
            &value
                .iter()
                .map(|arc| arc.to_string())
                .collect::<Vec<String>>()
                .join("."),
        ))
    }

    fn write_null(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    fn write_octet_string(&mut self, value: &[u8]) -> Result<(), EncodeError> {
        if value.is_empty() {
            Ok(())
        } else {
            self.write(XmlEvent::Characters(
                value
                    .iter()
                    .try_fold(String::new(), |mut acc, byte| {
                        write!(&mut acc, "{byte:02X?}").map(|_| acc)
                    })
                    .map_err(|e| XerEncodeErrorKind::XmlEncodingError {
                        upstream: e.to_string(),
                    })?
                    .as_str(),
            ))
        }
    }

    fn write_string_type(&mut self, value: &str) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(value))
    }

    fn write_generalized_time(&mut self, value: &GeneralizedTime) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(
            &String::from_utf8(
                crate::ber::enc::Encoder::datetime_to_canonical_generalized_time_bytes(value),
            )
            .map_err(|e| XerEncodeErrorKind::XmlEncodingError {
                upstream: e.to_string(),
            })?,
        ))
    }

    fn write_utc_time(&mut self, value: &UtcTime) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(
            &String::from_utf8(
                crate::ber::enc::Encoder::datetime_to_canonical_utc_time_bytes(value),
            )
            .map_err(|e| XerEncodeErrorKind::XmlEncodingError {
                upstream: e.to_string(),
            })?,
        ))
    }

    fn write_any(&mut self, value: &Any) -> Result<(), EncodeError> {
        let mut reader = ParserConfig::default().create_reader(value.contents.iter());
        while let Ok(evt) = reader.next() {
            match evt {
                xml_no_std::reader::XmlEvent::StartDocument { .. } => {
                    return Err(XerEncodeErrorKind::XmlEncodingError {
                        upstream: "Any-type values must not contain XML prolog!".to_string(),
                    }
                    .into())
                }
                xml_no_std::reader::XmlEvent::EndDocument => break,
                xml_no_std::reader::XmlEvent::ProcessingInstruction { name, data } => {
                    self.write(XmlEvent::ProcessingInstruction {
                        name: &name,
                        data: data.as_deref(),
                    })?;
                }
                xml_no_std::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    self.write(XmlEvent::StartElement {
                        name: name.borrow(),
                        namespace: namespace.borrow(),
                        attributes: attributes
                            .iter()
                            .map(|attr| Attribute::new(attr.name.borrow(), &attr.value))
                            .collect(),
                    })?;
                }
                xml_no_std::reader::XmlEvent::EndElement { name } => {
                    self.write(XmlEvent::EndElement {
                        name: Some(name.borrow()),
                    })?;
                }
                xml_no_std::reader::XmlEvent::CData(cdata) => {
                    self.write(XmlEvent::CData(&cdata))?;
                }
                xml_no_std::reader::XmlEvent::Comment(comment) => {
                    self.write(XmlEvent::Comment(&comment))?;
                }
                xml_no_std::reader::XmlEvent::Characters(characters) => {
                    self.write(XmlEvent::Characters(&characters))?;
                }
                xml_no_std::reader::XmlEvent::Whitespace(_) => {
                    continue;
                }
            }
        }

        Ok(())
    }
}
