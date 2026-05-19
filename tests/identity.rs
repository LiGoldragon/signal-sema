//! Falsifiable witnesses for [`Slot<T>`] and [`Revision`].
//!
//! These are wire-identity primitives — the rkyv archive shape and
//! the small constructor surface are the contract.

use rkyv::rancor::Error as RkyvError;
use signal_sema::{ArchivedRevision, ArchivedSlot, Revision, Slot};

#[derive(
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
struct Marker;

#[test]
fn slot_number_round_trips() {
    let slot: Slot<Marker> = Slot::new(7);
    assert_eq!(slot.number(), 7);
}

#[test]
fn slot_typed_families_are_distinct_types() {
    // Two slots of different families with the same number are not
    // interchangeable at the type level; this is just a
    // compile-time witness that the phantom parameter is honoured.
    let _slot_marker: Slot<Marker> = Slot::new(1);
    let _slot_unit: Slot<()> = Slot::new(1);
}

#[test]
fn slot_round_trips_through_rkyv() {
    let slot: Slot<Marker> = Slot::new(42);
    let bytes = rkyv::to_bytes::<RkyvError>(&slot).expect("archive Slot");
    let archived = rkyv::access::<ArchivedSlot<Marker>, RkyvError>(&bytes).expect("access archive");
    let decoded: Slot<Marker> =
        rkyv::deserialize::<Slot<Marker>, RkyvError>(archived).expect("deserialize");
    assert_eq!(decoded, slot);
    assert_eq!(decoded.number(), 42);
}

#[test]
fn revision_initial_is_zero() {
    assert_eq!(Revision::initial(), Revision::new(0));
    assert_eq!(Revision::initial().number(), 0);
}

#[test]
fn revision_round_trips_through_rkyv() {
    let revision = Revision::new(123);
    let bytes = rkyv::to_bytes::<RkyvError>(&revision).expect("archive Revision");
    let archived = rkyv::access::<ArchivedRevision, RkyvError>(&bytes).expect("access archive");
    let decoded: Revision =
        rkyv::deserialize::<Revision, RkyvError>(archived).expect("deserialize");
    assert_eq!(decoded, revision);
    assert_eq!(decoded.number(), 123);
}
