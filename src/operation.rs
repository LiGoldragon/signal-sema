//! The closed Sema operation classification vocabulary.
//!
//! [`SemaOperation`] is the set of payloadless state-action labels
//! that component-local executable commands project into for generic
//! observation.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// The closed operation classification set for typed state actions.
///
/// This enum does not carry executable payloads. Component daemons
/// own executable command records and project them into this enum
/// for observation and filtering.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
pub enum SemaOperation {
    /// Insert or append a typed record.
    Assert,
    /// Replace or transition an existing typed record.
    Mutate,
    /// Tombstone, remove, or retract a typed record.
    Retract,
    /// Read typed records by key, range, pattern, or plan.
    Match,
    /// Open state-plus-delta observation over typed records.
    Subscribe,
    /// Dry-run validation or planning without committing.
    Validate,
}

impl SemaOperation {
    /// PascalCase NOTA record-head identifier for this operation.
    /// Stable across the public NOTA surface; agents may treat this
    /// as a canonical naming for the operation in chat or text.
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

    /// Parse a PascalCase NOTA record-head identifier back to a
    /// [`SemaOperation`]. Returns [`None`] for any other identifier.
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

    /// The kind of work this operation does against typed-record
    /// state. Used by daemons that need to dispatch on the broad
    /// class of effect (transactional write, read, streaming, or
    /// dry-run).
    pub const fn class(self) -> OperationClass {
        match self {
            Self::Assert | Self::Mutate | Self::Retract => OperationClass::Write,
            Self::Match => OperationClass::Read,
            Self::Subscribe => OperationClass::Stream,
            Self::Validate => OperationClass::Validation,
        }
    }

    /// True when this operation changes durable state on commit.
    pub const fn is_write(self) -> bool {
        matches!(self.class(), OperationClass::Write)
    }

    /// Stable low-byte classification code for compact observations.
    ///
    /// This is vocabulary-local metadata, not a `signal-frame` trait impl:
    /// the frame kernel and the Sema classification vocabulary stay separate.
    pub const fn log_variant(self) -> u64 {
        let byte = match self {
            Self::Assert => 0,
            Self::Mutate => 1,
            Self::Retract => 2,
            Self::Match => 3,
            Self::Subscribe => 4,
            Self::Validate => 5,
        };
        byte as u64
    }
}

/// Projection from a component-local executable command into the
/// universal Sema operation classification vocabulary.
///
/// Component commands keep their executable payloads in their owning
/// daemon crate. They implement this trait so observer streams can
/// filter and summarize state effects using the workspace-wide
/// [`SemaOperation`] classes without making `SemaOperation` carry
/// executable payloads.
pub trait ToSemaOperation {
    fn to_sema_operation(&self) -> SemaOperation;
}

impl ToSemaOperation for SemaOperation {
    fn to_sema_operation(&self) -> SemaOperation {
        *self
    }
}

/// The broad class of effect a [`SemaOperation`] has against typed
/// state. The classes are stable; new Sema operations declare their
/// class as part of their definition.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
pub enum OperationClass {
    /// Transactional change to typed-record state. `Assert`,
    /// `Mutate`, `Retract`.
    Write,
    /// Read-only access to typed-record state. `Match`.
    Read,
    /// Streaming initial-plus-delta observation. `Subscribe`.
    Stream,
    /// Dry-run validation or planning. `Validate`.
    Validation,
}
