//! Typed identity primitives for Sema-operation payloads.
//!
//! [`Slot<Payload>`] addresses a stable identity for a typed
//! record family. [`Revision`] names a particular generation of
//! the value at that slot. Together they let `Mutate` and
//! `Retract` point unambiguously at a row in durable state.
//!
//! These are **identity values only**. Allocation, lookup, and
//! compare-and-set behavior belong to the Sema engine; this crate
//! only defines the wire shape and the typed family marker.

use std::marker::PhantomData;

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// A stable identity for a typed record of family `Payload`.
///
/// The phantom-typed family parameter keeps slots for different
/// record families from being silently confused at the type level.
/// On the wire the slot is just its numeric identifier; the family
/// is reconstructed from the schema position in which the slot
/// appears.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct Slot<Payload> {
    number: u64,
    payload: PhantomData<Payload>,
}

impl<Payload> Slot<Payload> {
    /// Wrap a raw slot number. The Sema engine owns allocation;
    /// callers should normally receive slots from the engine rather
    /// than construct them ad-hoc.
    pub const fn new(number: u64) -> Self {
        Self {
            number,
            payload: PhantomData,
        }
    }

    /// The wire-level numeric identity of this slot, ignoring the
    /// typed family marker.
    pub const fn number(&self) -> u64 {
        self.number
    }
}

/// A monotonic generation counter for the value at a [`Slot`].
///
/// `Mutate` and `Retract` use the expected revision to detect
/// concurrent change; bumping the revision on each transition is
/// the engine's responsibility.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct Revision {
    number: u64,
}

impl Revision {
    /// Wrap a raw revision number.
    pub const fn new(number: u64) -> Self {
        Self { number }
    }

    /// The first revision a fresh slot holds.
    pub const fn initial() -> Self {
        Self::new(0)
    }

    /// The wire-level numeric value of this revision.
    pub const fn number(&self) -> u64 {
        self.number
    }
}
