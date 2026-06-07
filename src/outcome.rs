//! Outcome-side Sema classification.
//!
//! Component daemons execute component-local commands and produce
//! component-local effects. [`SemaOutcome`] is the universal,
//! payloadless classification of those effects for observation.

use nota_next::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::{SemaOperation, ToSemaOperation};

/// The closed outcome set visible to generic Sema observers.
///
/// Outcomes do not carry executable or domain payloads. Component
/// events carry those details separately; this enum classifies what
/// happened in the broad workspace vocabulary.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum SemaOutcome {
    /// A new typed fact / event / row was appended.
    Asserted,
    /// An existing typed record transitioned at stable identity.
    Mutated,
    /// A typed record was tombstoned, removed, or retracted.
    Retracted,
    /// Typed records were read by key, range, pattern, or plan.
    Matched,
    /// A state-plus-delta observation stream was opened.
    Subscribed,
    /// A dry-run validation or planning request completed.
    Validated,
    /// The request completed without changing observable state.
    NoChange,
}

impl SemaOutcome {
    /// PascalCase NOTA record-head identifier for this outcome.
    pub const fn as_record_head(self) -> &'static str {
        match self {
            Self::Asserted => "Asserted",
            Self::Mutated => "Mutated",
            Self::Retracted => "Retracted",
            Self::Matched => "Matched",
            Self::Subscribed => "Subscribed",
            Self::Validated => "Validated",
            Self::NoChange => "NoChange",
        }
    }

    /// Parse a PascalCase NOTA record-head identifier back to a
    /// [`SemaOutcome`].
    pub fn from_record_head(name: &str) -> Option<Self> {
        match name {
            "Asserted" => Some(Self::Asserted),
            "Mutated" => Some(Self::Mutated),
            "Retracted" => Some(Self::Retracted),
            "Matched" => Some(Self::Matched),
            "Subscribed" => Some(Self::Subscribed),
            "Validated" => Some(Self::Validated),
            "NoChange" => Some(Self::NoChange),
            _ => None,
        }
    }

    /// Stable second-byte classification code for compact observations.
    pub const fn log_variant(self) -> u64 {
        let byte = match self {
            Self::Asserted => 0,
            Self::Mutated => 1,
            Self::Retracted => 2,
            Self::Matched => 3,
            Self::Subscribed => 4,
            Self::Validated => 5,
            Self::NoChange => 6,
        };
        byte as u64
    }
}

/// Projection from a component-local effect into the universal Sema
/// outcome classification vocabulary.
pub trait ToSemaOutcome {
    fn to_sema_outcome(&self) -> SemaOutcome;
}

impl ToSemaOutcome for SemaOutcome {
    fn to_sema_outcome(&self) -> SemaOutcome {
        *self
    }
}

/// Universal observation label produced from a component command and
/// the effect it caused.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct SemaObservation {
    pub operation: SemaOperation,
    pub outcome: SemaOutcome,
}

impl SemaObservation {
    pub const fn new(operation: SemaOperation, outcome: SemaOutcome) -> Self {
        Self { operation, outcome }
    }

    pub fn from_projection<Command, Effect>(command: &Command, effect: &Effect) -> Self
    where
        Command: ToSemaOperation,
        Effect: ToSemaOutcome,
    {
        Self::new(command.to_sema_operation(), effect.to_sema_outcome())
    }

    /// Stable compact observation code: operation byte, outcome byte, class byte.
    pub const fn log_variant(&self) -> u64 {
        let operation = self.operation.log_variant();
        let outcome = self.outcome.log_variant();
        let class = match self.operation.class() {
            crate::OperationClass::Write => 0,
            crate::OperationClass::Read => 1,
            crate::OperationClass::Stream => 2,
            crate::OperationClass::Validation => 3,
        };
        operation | (outcome << 8) | ((class as u64) << 16)
    }
}
