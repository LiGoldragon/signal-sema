//! Falsifiable witnesses for [`Bind`], [`Wildcard`], and
//! [`PatternField`]. Each case is the canonical text shape; if a
//! future change drifts the wire form, exactly one of these tests
//! breaks and names the variant that drifted.
//!
//! Inherited from `signal-core` per the architecture migration;
//! pattern primitives now live in `signal-sema` because they pair
//! with the [`SemaOperation::Match`] and [`SemaOperation::Subscribe`]
//! read algebra. See `ARCHITECTURE.md` and the migration reports.

#[cfg(feature = "nota-text")]
use nota_next::{NotaDecode, NotaEncode, NotaSource};
use rkyv::rancor::Error as RkyvError;
#[cfg(feature = "nota-text")]
use signal_sema::PatternField;
use signal_sema::{ArchivedBind, ArchivedWildcard, Bind, Wildcard};

#[cfg(feature = "nota-text")]
fn encode<T: NotaEncode>(value: &T) -> String {
    value.to_nota()
}

#[cfg(feature = "nota-text")]
fn decode<T: NotaDecode>(text: &str) -> T {
    NotaSource::new(text).parse::<T>().unwrap()
}

// ─── Bind / Wildcard direct NOTA round trips ────────────────────

#[test]
#[cfg(feature = "nota-text")]
fn bind_emits_typed_record() {
    assert_eq!(encode(&Bind), "(Bind)");
}

#[test]
#[cfg(feature = "nota-text")]
fn wildcard_emits_typed_record() {
    assert_eq!(encode(&Wildcard), "(Wildcard)");
}

#[test]
#[cfg(feature = "nota-text")]
fn bind_round_trips() {
    let _: Bind = decode("(Bind)");
}

#[test]
#[cfg(feature = "nota-text")]
fn wildcard_round_trips() {
    let _: Wildcard = decode("(Wildcard)");
}

// ─── Bind / Wildcard rkyv round trips ───────────────────────────

#[test]
fn bind_round_trips_through_rkyv() {
    let bytes = rkyv::to_bytes::<RkyvError>(&Bind).expect("archive Bind");
    let archived = rkyv::access::<ArchivedBind, RkyvError>(&bytes).expect("access archive");
    let _decoded: Bind = rkyv::deserialize::<Bind, RkyvError>(archived).expect("deserialize");
}

#[test]
fn wildcard_round_trips_through_rkyv() {
    let bytes = rkyv::to_bytes::<RkyvError>(&Wildcard).expect("archive Wildcard");
    let archived = rkyv::access::<ArchivedWildcard, RkyvError>(&bytes).expect("access archive");
    let _decoded: Wildcard =
        rkyv::deserialize::<Wildcard, RkyvError>(archived).expect("deserialize");
}

// ─── PatternField<String> ───────────────────────────────────────

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_string_bind_emits_bind_record() {
    let pattern: PatternField<String> = PatternField::Bind;
    assert_eq!(encode(&pattern), "(Bind)");
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_string_wildcard_emits_wildcard_record() {
    let pattern: PatternField<String> = PatternField::Wildcard;
    assert_eq!(encode(&pattern), "(Wildcard)");
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_string_match_emits_bare_when_camel_case() {
    let pattern: PatternField<String> = PatternField::Match("alice".into());
    assert_eq!(encode(&pattern), "alice");
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_string_match_emits_bare_when_pascal_case() {
    let pattern: PatternField<String> = PatternField::Match("User".into());
    assert_eq!(encode(&pattern), "User");
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_string_match_emits_bracket_string_when_not_bare_eligible() {
    let pattern: PatternField<String> = PatternField::Match("hello world".into());
    assert_eq!(encode(&pattern), "[hello world]");
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_string_round_trip_bind() {
    let decoded: PatternField<String> = decode("(Bind)");
    assert_eq!(decoded, PatternField::Bind);
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_string_round_trip_wildcard() {
    let decoded: PatternField<String> = decode("(Wildcard)");
    assert_eq!(decoded, PatternField::Wildcard);
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_string_round_trip_match_pascal_case() {
    let decoded: PatternField<String> = decode("User");
    assert_eq!(decoded, PatternField::Match("User".into()));
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_string_round_trip_match_bare_ident() {
    // Bare camelCase / kebab-case content decodes as the Match
    // payload and remains bare when re-encoded.
    let decoded: PatternField<String> = decode("alice");
    assert_eq!(decoded, PatternField::Match("alice".into()));
}

// ─── PatternField<u64> — bare integer in slot/numeric position ──

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_u64_match_emits_bare_integer() {
    let pattern: PatternField<u64> = PatternField::Match(42);
    assert_eq!(encode(&pattern), "42");
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_u64_round_trip_bind() {
    let decoded: PatternField<u64> = decode("(Bind)");
    assert_eq!(decoded, PatternField::Bind);
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_u64_round_trip_match() {
    let decoded: PatternField<u64> = decode("100");
    assert_eq!(decoded, PatternField::Match(100));
}

// ─── Negative — `@name` and `_` no longer parse as patterns ─────

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_does_not_treat_at_sigil_as_bind() {
    // `@` has no pattern-marker role here. It remains plain text
    // unless a structural macro type gives it a local meaning.
    let decoded: PatternField<String> = decode("@name");
    assert_eq!(decoded, PatternField::Match("@name".into()));
}

#[test]
#[cfg(feature = "nota-text")]
fn pattern_field_underscore_is_just_an_identifier() {
    // `_` is a valid bare atom per the nota grammar; in a
    // PatternField<String> position it decodes as Match("_"),
    // **not** as Wildcard. Wildcard is now only the typed
    // record `(Wildcard)`.
    let decoded: PatternField<String> = decode("_");
    assert_eq!(decoded, PatternField::Match("_".into()));
}
