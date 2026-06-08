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

#[cfg(feature = "nota-text")]
use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Marker for a position that captures the matched value into the
/// pattern's bind set. Encodes as the empty NOTA record `(Bind)`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bind;

#[cfg(feature = "nota-text")]
impl NotaEncode for Bind {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap(["Bind".to_owned()])
    }
}

#[cfg(feature = "nota-text")]
impl NotaDecode for Bind {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let children = NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "Bind", 1)?;
        let head = children[0]
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom { type_name: "Bind" })?;
        if head != "Bind" {
            return Err(NotaDecodeError::UnknownVariant {
                enum_name: "Bind",
                variant: head.to_owned(),
            });
        }
        Ok(Self)
    }
}

/// Marker for a position that accepts any value and does not bind
/// it. Encodes as the empty NOTA record `(Wildcard)`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wildcard;

#[cfg(feature = "nota-text")]
impl NotaEncode for Wildcard {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap(["Wildcard".to_owned()])
    }
}

#[cfg(feature = "nota-text")]
impl NotaDecode for Wildcard {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let children =
            NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "Wildcard", 1)?;
        let head = children[0]
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom {
                type_name: "Wildcard",
            })?;
        if head != "Wildcard" {
            return Err(NotaDecodeError::UnknownVariant {
                enum_name: "Wildcard",
                variant: head.to_owned(),
            });
        }
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

#[cfg(feature = "nota-text")]
impl<T: NotaEncode> NotaEncode for PatternField<T> {
    fn to_nota(&self) -> String {
        match self {
            Self::Wildcard => Wildcard.to_nota(),
            Self::Bind => Bind.to_nota(),
            Self::Match(value) => value.to_nota(),
        }
    }
}

#[cfg(feature = "nota-text")]
impl<T: NotaDecode> NotaDecode for PatternField<T> {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        if let Some(children) = block.as_delimited(Delimiter::Parenthesis)
            && let Some(head) = children.first().and_then(Block::demote_to_string)
        {
            match head {
                "Bind" => return Bind::from_nota_block(block).map(|_| Self::Bind),
                "Wildcard" => {
                    return Wildcard::from_nota_block(block).map(|_| Self::Wildcard);
                }
                _ => {}
            }
        }
        T::from_nota_block(block).map(Self::Match)
    }
}
