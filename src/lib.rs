//! Sema execution vocabulary.
//!
//! Public component contracts use domain-local operation verbs. This crate
//! owns the lower database-operation vocabulary that a daemon lowers into
//! when it asks `sema-engine` to read or change durable state.

use nota_codec::NotaEnum;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// The closed operation set a Sema engine executes against typed records.
///
/// Atomicity is structural in the engine request/commit shape; it is not a
/// separate operation.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum SemaOperation {
    Assert,
    Mutate,
    Retract,
    Match,
    Subscribe,
    Validate,
}

impl SemaOperation {
    pub const fn as_record_head(self) -> &'static str {
        match self {
            Self::Assert => "Assert",
            Self::Mutate => "Mutate",
            Self::Retract => "Retract",
            Self::Match => "Match",
            Self::Subscribe => "Subscribe",
            Self::Validate => "Validate",
        }
    }

    pub fn from_record_head(name: &str) -> Option<Self> {
        match name {
            "Assert" => Some(Self::Assert),
            "Mutate" => Some(Self::Mutate),
            "Retract" => Some(Self::Retract),
            "Match" => Some(Self::Match),
            "Subscribe" => Some(Self::Subscribe),
            "Validate" => Some(Self::Validate),
            _ => None,
        }
    }

    pub const fn class(self) -> OperationClass {
        match self {
            Self::Assert | Self::Mutate | Self::Retract => OperationClass::Write,
            Self::Match => OperationClass::Read,
            Self::Subscribe => OperationClass::Stream,
            Self::Validate => OperationClass::Validation,
        }
    }

    pub const fn is_write(self) -> bool {
        matches!(self.class(), OperationClass::Write)
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OperationClass {
    Write,
    Read,
    Stream,
    Validation,
}
