//! Pattern marker records and the [`PatternField`] read-algebra
//! primitive.
//!
//! [`Bind`] and [`Wildcard`] are normal typed values that appear
//! in `Match` and `Subscribe` payloads to mark unbound or
//! captured positions. They encode as the typed empty records
//! `(Bind)` and `(Wildcard)`.
//!
//! [`PatternField<T>`] generalises this: at any position in a
//! `Match` or `Subscribe` payload that takes a typed value of type
//! `T`, callers may instead supply [`PatternField::Bind`] (capture
//! the bound value) or [`PatternField::Wildcard`] (accept any
//! value). A concrete value is encoded as the value itself —
//! [`PatternField`] is transparent over [`PatternField::Match`].

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, Result, Token};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Marker for a position that captures the matched value into the
/// pattern's bind set. Encodes as the empty NOTA record `(Bind)`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bind;

impl NotaEncode for Bind {
    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        encoder.start_record("Bind")?;
        encoder.end_record()
    }
}

impl NotaDecode for Bind {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self> {
        decoder.expect_record_head("Bind")?;
        decoder.expect_record_end()?;
        Ok(Self)
    }
}

/// Marker for a position that accepts any value and does not bind
/// it. Encodes as the empty NOTA record `(Wildcard)`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wildcard;

impl NotaEncode for Wildcard {
    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        encoder.start_record("Wildcard")?;
        encoder.end_record()
    }
}

impl NotaDecode for Wildcard {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self> {
        decoder.expect_record_head("Wildcard")?;
        decoder.expect_record_end()?;
        Ok(Self)
    }
}

/// A pattern position over typed value `T`: either a wildcard
/// (accept anything), a bind (capture the matched value), or a
/// concrete value to match against.
///
/// `PatternField` is **transparent** over [`Self::Match`]: the
/// inner value's NOTA encoding is used directly. [`Self::Bind`]
/// and [`Self::Wildcard`] encode as the typed records `(Bind)` and
/// `(Wildcard)`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternField<T> {
    /// Accept any value at this position; do not bind it.
    Wildcard,
    /// Capture the matched value at this position into the pattern's
    /// bind set.
    Bind,
    /// Require the matched value to equal the inner value.
    Match(T),
}

impl<T: NotaEncode> NotaEncode for PatternField<T> {
    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        match self {
            Self::Wildcard => Wildcard.encode(encoder),
            Self::Bind => Bind.encode(encoder),
            Self::Match(value) => value.encode(encoder),
        }
    }
}

impl<T: NotaDecode> NotaDecode for PatternField<T> {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self> {
        match decoder.peek_token()? {
            Some(Token::LParen) => {
                let head = decoder.peek_record_head()?;
                match head.as_str() {
                    "Bind" => {
                        let _marker = Bind::decode(decoder)?;
                        Ok(Self::Bind)
                    }
                    "Wildcard" => {
                        let _marker = Wildcard::decode(decoder)?;
                        Ok(Self::Wildcard)
                    }
                    _ => Ok(Self::Match(T::decode(decoder)?)),
                }
            }
            _ => Ok(Self::Match(T::decode(decoder)?)),
        }
    }
}
