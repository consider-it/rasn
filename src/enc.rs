//! Generic ASN.1 encoding framework.

use crate::types::{self, AsnType, Constraints, Enumerated, Tag};

pub use rasn_derive::Encode;

/// A **data type** that can be encoded to a ASN.1 data format.
pub trait Encode: AsnType {
    /// Encodes `self`'s data into the given `Encoder`.
    ///
    /// **Note for implementors** You typically do not need to implement this.
    /// The default implementation will call `Encode::encode_with_tag` with
    /// your types associated `AsnType::TAG`. You should only ever need to
    /// implement this if you have a type that *cannot* be implicitly tagged,
    /// such as a `CHOICE` type, in which case you want to implement encoding
    /// in `encode`.
    fn encode<E: Encoder>(
        &self,
        encoder: &mut E,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        self.encode_with_tag_and_constraints(encoder, Self::TAG, Self::CONSTRAINTS, identifier)
    }

    /// Encode this value with `tag` into the given `Encoder`.
    ///
    /// **Note** For `CHOICE` and other types that cannot be implicitly tagged
    /// this will **explicitly tag** the value, for all other types, it will
    /// **implicitly** tag the value.
    fn encode_with_tag<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        self.encode_with_tag_and_constraints(encoder, tag, Self::CONSTRAINTS, identifier)
    }

    fn encode_with_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        self.encode_with_tag_and_constraints(encoder, Self::TAG, constraints, identifier)
    }

    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error>;
}

/// A **data format** encode any ASN.1 data type.
pub trait Encoder {
    type Ok;
    type Error: Error + Into<crate::error::EncodeError> + From<crate::error::EncodeError>;

    /// Returns codec variant of `Codec` that current encoder is encoding.
    fn codec(&self) -> crate::Codec;

    /// Encode an unknown ASN.1 value.
    fn encode_any(&mut self, tag: Tag, value: &types::Any) -> Result<Self::Ok, Self::Error>;

    /// Encode a `BOOL` value.
    fn encode_bool(
        &mut self,
        tag: Tag,
        value: bool,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `BIT STRING` value.
    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BitStr,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `ENUMERATED` value.
    fn encode_enumerated<E: Enumerated>(
        &mut self,
        tag: Tag,
        value: &E,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `OBJECT IDENTIFIER` value.
    fn encode_object_identifier(
        &mut self,
        tag: Tag,
        value: &[u32],
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `INTEGER` value.
    fn encode_integer(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &num_bigint::BigInt,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `NULL` value.
    fn encode_null(
        &mut self,
        tag: Tag,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `OCTET STRING` value.
    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `GeneralString` value.
    fn encode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::GeneralString,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `Utf8String` value.
    fn encode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &str,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `VisibleString` value.
    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::VisibleString,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `Ia5String` value.
    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::Ia5String,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `Ia5String` value.
    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::PrintableString,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `NumericString` value.
    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::NumericString,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `TeletexString` value.
    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::TeletexString,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `BmpString` value.
    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BmpString,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `GeneralizedTime` value.
    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `UtcTime` value.
    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a explicitly tagged value.
    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `SEQUENCE` value.
    fn encode_sequence<C, F>(
        &mut self,
        tag: Tag,
        encoder_scope: F,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>;

    /// Encode a `SEQUENCE OF` value.
    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &[E],
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `SET` value.
    fn encode_set<C, F>(
        &mut self,
        tag: Tag,
        value: F,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>;

    /// Encode a `SET OF` value.
    fn encode_set_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &types::SetOf<E>,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode the value a field or skip if it matches the default..
    fn encode_or_default<E: Encode + Default + PartialEq>(
        &mut self,
        value: &E,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error> {
        if value != &E::default() {
            self.encode_some(value, identifier)
        } else {
            self.encode_none::<E>()
        }
    }

    /// Encode the present value of an optional field.
    fn encode_with_tag_or_default<E: Encode + Default + PartialEq>(
        &mut self,
        tag: Tag,
        value: &E,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error> {
        if value != &E::default() {
            self.encode_some_with_tag(tag, value, identifier)
        } else {
            self.encode_none::<E>()
        }
    }

    /// Encode the present value of an optional field.
    fn encode_some<E: Encode>(
        &mut self,
        value: &E,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode the present value of an optional field.
    fn encode_some_with_tag<E: Encode>(
        &mut self,
        tag: Tag,
        value: &E,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_some_with_tag_and_constraints(tag, <_>::default(), value, identifier)
    }

    /// Encode the present value of an optional field.
    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode the absent value of an optional field.
    fn encode_none<E: Encode>(&mut self) -> Result<Self::Ok, Self::Error>;

    /// Encode the absent value with `tag` of an optional field.
    fn encode_none_with_tag(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error>;

    /// Encode the present value of an optional field.
    fn encode_default<E: Encode + PartialEq>(
        &mut self,
        value: &E,
        identifier: Option<&'static str>,
        default: impl FnOnce() -> E,
    ) -> Result<Self::Ok, Self::Error> {
        match (*value != (default)()).then_some(value) {
            Some(value) => self.encode_some(value, identifier),
            None => self.encode_none::<E>(),
        }
    }

    /// Encode the present value of an optional field.
    fn encode_default_with_tag<E: Encode + PartialEq>(
        &mut self,
        tag: Tag,
        value: &E,
        identifier: Option<&'static str>,
        default: impl FnOnce() -> E,
    ) -> Result<Self::Ok, Self::Error> {
        match (*value != (default)()).then_some(value) {
            Some(value) => self.encode_some_with_tag(tag, value, identifier),
            None => self.encode_none_with_tag(tag),
        }
    }

    /// Encode the present value of an optional field.
    fn encode_default_with_tag_and_constraints<E: Encode + PartialEq>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
        identifier: Option<&'static str>,
        default: impl FnOnce() -> E,
    ) -> Result<Self::Ok, Self::Error> {
        match (*value != (default)()).then_some(value) {
            Some(value) => {
                self.encode_some_with_tag_and_constraints(tag, constraints, value, identifier)
            }
            None => self.encode_none_with_tag(tag),
        }
    }

    /// Encode the present constrained value of an optional field.
    fn encode_default_with_constraints<E: Encode + PartialEq>(
        &mut self,
        constraints: Constraints,
        value: &E,
        identifier: Option<&'static str>,
        default: impl FnOnce() -> E,
    ) -> Result<Self::Ok, Self::Error> {
        match (*value != (default)()).then_some(value) {
            Some(value) => {
                self.encode_some_with_tag_and_constraints(E::TAG, constraints, value, identifier)
            }
            None => self.encode_none_with_tag(E::TAG),
        }
    }

    /// Encode a `CHOICE` value.
    fn encode_choice<E: Encode + crate::types::Choice>(
        &mut self,
        constraints: Constraints,
        tag: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a extension addition value.
    fn encode_extension_addition<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: E,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a extension addition group value.
    fn encode_extension_addition_group<E>(
        &mut self,
        value: Option<&E>,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: Encode + crate::types::Constructed;
}

/// A generic error that occurred while trying to encode ASN.1.
pub trait Error: core::fmt::Display {
    /// Creates a new general error using `msg` and current `codec` when encoding ASN.1.
    fn custom<D: core::fmt::Display>(msg: D, codec: crate::Codec) -> Self;
}

impl Error for core::convert::Infallible {
    fn custom<D: core::fmt::Display>(msg: D, codec: crate::Codec) -> Self {
        core::panic!("Infallible error! {}, from: {}", msg, codec)
    }
}

impl<E: Encode> Encode for &'_ E {
    fn encode<EN: Encoder>(
        &self,
        encoder: &mut EN,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        E::encode(self, encoder, identifier)
    }

    fn encode_with_tag<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        E::encode_with_tag(self, encoder, tag, identifier)
    }

    fn encode_with_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        E::encode_with_constraints(self, encoder, constraints, identifier)
    }

    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        E::encode_with_tag_and_constraints(self, encoder, tag, constraints, identifier)
    }
}

impl Encode for () {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder.encode_null(tag, identifier).map(drop)
    }
}

impl<E: Encode> Encode for Option<E> {
    fn encode<EN: Encoder>(
        &self,
        encoder: &mut EN,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        match self {
            Some(value) => encoder.encode_some::<E>(value, identifier),
            None => encoder.encode_none::<E>(),
        }
        .map(drop)
    }

    fn encode_with_tag<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        match self {
            Some(value) => encoder.encode_some_with_tag(tag, value, identifier),
            None => encoder.encode_none::<E>(),
        }
        .map(drop)
    }

    fn encode_with_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        match self {
            Some(value) => encoder.encode_some_with_tag_and_constraints(
                Self::TAG,
                constraints,
                value,
                identifier,
            ),
            None => encoder.encode_none::<E>(),
        }
        .map(drop)
    }

    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        match self {
            Some(value) => {
                encoder.encode_some_with_tag_and_constraints(tag, constraints, value, identifier)
            }

            None => encoder.encode_none::<E>(),
        }
        .map(drop)
    }
}

impl Encode for bool {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder.encode_bool(tag, *self, identifier).map(drop)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+) => {
        $(
            impl Encode for $int {
                fn encode_with_tag_and_constraints<E: Encoder>(&self, encoder: &mut E, tag: Tag, constraints: Constraints, identifier: Option<&'static str>) -> Result<(), E::Error> {
                    encoder.encode_integer(
                        tag,
                        constraints,
                        &(*self).into(),
                        identifier
                    ).map(drop)
                }
            }
        )+
    }
}

impl_integers! {
    i8,
    i16,
    i32,
    i64,
    isize,
    u8,
    u16,
    u32,
    u64,
    usize
}

impl<const START: i128, const END: i128> Encode for types::ConstrainedInteger<START, END> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_integer(tag, constraints, self, identifier)
            .map(drop)
    }
}

impl Encode for types::Integer {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_integer(tag, constraints, self, identifier)
            .map(drop)
    }
}

impl Encode for types::OctetString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_octet_string(tag, constraints, self, identifier)
            .map(drop)
    }
}

impl Encode for types::Utf8String {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_utf8_string(tag, constraints, self, identifier)
            .map(drop)
    }
}

impl Encode for &'_ str {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_utf8_string(tag, constraints, self, identifier)
            .map(drop)
    }
}

impl Encode for types::ObjectIdentifier {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_object_identifier(tag, self, identifier)
            .map(drop)
    }
}

impl Encode for types::Oid {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_object_identifier(tag, self, identifier)
            .map(drop)
    }
}

impl Encode for types::UtcTime {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder.encode_utc_time(tag, self, identifier).map(drop)
    }
}

impl Encode for types::GeneralizedTime {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_generalized_time(tag, self, identifier)
            .map(drop)
    }
}

impl Encode for types::Any {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        _identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder.encode_any(tag, self).map(drop)
    }
}

impl<E: Encode> Encode for alloc::boxed::Box<E> {
    fn encode<EN: Encoder>(
        &self,
        encoder: &mut EN,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        E::encode(self, encoder, identifier)
    }

    fn encode_with_tag<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        E::encode_with_tag(self, encoder, tag, identifier)
    }

    fn encode_with_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        E::encode_with_constraints(self, encoder, constraints, identifier)
    }

    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        E::encode_with_tag_and_constraints(self, encoder, tag, constraints, identifier)
    }
}

impl<E: Encode> Encode for alloc::vec::Vec<E> {
    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        encoder
            .encode_sequence_of(tag, self, constraints, identifier)
            .map(drop)
    }
}

impl<E: Encode> Encode for alloc::collections::BTreeSet<E> {
    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        encoder
            .encode_set_of(tag, self, constraints, identifier)
            .map(drop)
    }
}

impl<E: Encode, const N: usize> Encode for [E; N] {
    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), EN::Error> {
        encoder
            .encode_sequence_of(tag, self, constraints, identifier)
            .map(drop)
    }
}

impl<T: AsnType, V: Encode> Encode for types::Implicit<T, V> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        V::encode_with_tag_and_constraints(&self.value, encoder, tag, constraints, identifier)
            .map(drop)
    }
}

impl<T: AsnType, V: Encode> Encode for types::Explicit<T, V> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_explicit_prefix(tag, &self.value, identifier)
            .map(drop)
    }
}
