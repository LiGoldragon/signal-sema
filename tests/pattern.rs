//! Falsifiable witnesses for [`Bind`], [`Wildcard`], and
//! [`PatternField`]. Each case is the canonical text shape; if a
//! future change drifts the wire form, exactly one of these tests
//! breaks and names the variant that drifted.
//!
//! Inherited from `signal-core` per the architecture migration;
//! pattern primitives now live in `signal-sema` because they pair
//! with the [`SemaOperation::Match`] and [`SemaOperation::Subscribe`]
//! read algebra. See `ARCHITECTURE.md` and the migration reports.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use rkyv::rancor::Error as RkyvError;
use signal_sema::{ArchivedBind, ArchivedWildcard, Bind, PatternField, Wildcard};

fn encode<T: NotaEncode>(value: &T) -> String {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).unwrap();
    encoder.into_string()
}

fn decode<T: NotaDecode>(text: &str) -> T {
    let mut decoder = Decoder::new(text);
    T::decode(&mut decoder).unwrap()
}

// ─── Bind / Wildcard direct NOTA round trips ────────────────────

#[test]
fn bind_emits_typed_record() {
    assert_eq!(encode(&Bind), "(Bind)");
}

#[test]
fn wildcard_emits_typed_record() {
    assert_eq!(encode(&Wildcard), "(Wildcard)");
}

#[test]
fn bind_round_trips() {
    let _: Bind = decode("(Bind)");
}

#[test]
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
fn pattern_field_string_bind_emits_bind_record() {
    let pattern: PatternField<String> = PatternField::Bind;
    assert_eq!(encode(&pattern), "(Bind)");
}

#[test]
fn pattern_field_string_wildcard_emits_wildcard_record() {
    let pattern: PatternField<String> = PatternField::Wildcard;
    assert_eq!(encode(&pattern), "(Wildcard)");
}

#[test]
fn pattern_field_string_match_emits_bare_when_camel_case() {
    // camelCase / kebab-case content emits as bare identifiers
    // per nota-codec's strict bare-string alphabet. PascalCase
    // content is reserved for enum variants per the three-case
    // rule and emits as quoted (see next test).
    let pattern: PatternField<String> = PatternField::Match("alice".into());
    assert_eq!(encode(&pattern), "alice");
}

#[test]
fn pattern_field_string_match_emits_quoted_for_pascal_case() {
    // PascalCase content at a String position emits as quoted —
    // bare PascalCase is reserved for variants. The decode side
    // rejects bare PascalCase as PascalCaseAtStringPosition;
    // emitting quoted keeps encode/decode symmetric.
    let pattern: PatternField<String> = PatternField::Match("User".into());
    assert_eq!(encode(&pattern), "\"User\"");
}

#[test]
fn pattern_field_string_match_emits_quoted_when_not_bare_eligible() {
    let pattern: PatternField<String> = PatternField::Match("hello world".into());
    assert_eq!(encode(&pattern), "\"hello world\"");
}

#[test]
fn pattern_field_string_round_trip_bind() {
    let decoded: PatternField<String> = decode("(Bind)");
    assert_eq!(decoded, PatternField::Bind);
}

#[test]
fn pattern_field_string_round_trip_wildcard() {
    let decoded: PatternField<String> = decode("(Wildcard)");
    assert_eq!(decoded, PatternField::Wildcard);
}

#[test]
fn pattern_field_string_round_trip_match_quoted() {
    let decoded: PatternField<String> = decode("\"User\"");
    assert_eq!(decoded, PatternField::Match("User".into()));
}

#[test]
fn pattern_field_string_round_trip_match_bare_ident() {
    // Bare camelCase / kebab-case content decodes as the Match
    // payload. PascalCase bare is rejected per the three-case
    // rule; see `pattern_field_string_match_emits_quoted_for_pascal_case`
    // for the round-trip canonicalisation.
    let decoded: PatternField<String> = decode("alice");
    assert_eq!(decoded, PatternField::Match("alice".into()));
}

// ─── PatternField<u64> — bare integer in slot/numeric position ──

#[test]
fn pattern_field_u64_match_emits_bare_integer() {
    let pattern: PatternField<u64> = PatternField::Match(42);
    assert_eq!(encode(&pattern), "42");
}

#[test]
fn pattern_field_u64_round_trip_bind() {
    let decoded: PatternField<u64> = decode("(Bind)");
    assert_eq!(decoded, PatternField::Bind);
}

#[test]
fn pattern_field_u64_round_trip_match() {
    let decoded: PatternField<u64> = decode("100");
    assert_eq!(decoded, PatternField::Match(100));
}

// ─── Negative — `@name` and `_` no longer parse as patterns ─────

#[test]
fn pattern_field_does_not_treat_at_sigil_as_bind() {
    // The `@` byte is rejected at the lexer (UnexpectedChar);
    // the decoder sees a hard error before any pattern dispatch.
    let mut decoder = Decoder::new("@name");
    let result: nota_codec::Result<PatternField<String>> = PatternField::decode(&mut decoder);
    assert!(result.is_err(), "@-sigil must not decode as Bind");
}

#[test]
fn pattern_field_underscore_is_just_an_identifier() {
    // `_` is a valid bare identifier per the nota grammar; in a
    // PatternField<String> position it decodes as Match("_"),
    // **not** as Wildcard. Wildcard is now only the typed
    // record `(Wildcard)`.
    let decoded: PatternField<String> = decode("_");
    assert_eq!(decoded, PatternField::Match("_".into()));
}
