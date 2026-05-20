//! Falsifiable witnesses for outcome-side Sema classification.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use rkyv::rancor::Error as RkyvError;
use signal_sema::{
    ArchivedSemaObservation, ArchivedSemaOutcome, SemaObservation, SemaOperation, SemaOutcome,
    ToSemaOperation, ToSemaOutcome,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn outcomes() -> [SemaOutcome; 7] {
    [
        SemaOutcome::Asserted,
        SemaOutcome::Mutated,
        SemaOutcome::Retracted,
        SemaOutcome::Matched,
        SemaOutcome::Subscribed,
        SemaOutcome::Validated,
        SemaOutcome::NoChange,
    ]
}

#[test]
fn sema_outcome_record_heads_are_stable() {
    let cases = [
        (SemaOutcome::Asserted, "Asserted"),
        (SemaOutcome::Mutated, "Mutated"),
        (SemaOutcome::Retracted, "Retracted"),
        (SemaOutcome::Matched, "Matched"),
        (SemaOutcome::Subscribed, "Subscribed"),
        (SemaOutcome::Validated, "Validated"),
        (SemaOutcome::NoChange, "NoChange"),
    ];

    for (outcome, head) in cases {
        assert_eq!(outcome.as_record_head(), head);
        assert_eq!(SemaOutcome::from_record_head(head), Some(outcome));
        assert!(
            CANONICAL.lines().any(|line| line == head),
            "examples/canonical.nota missing {head}"
        );
    }

    assert_eq!(SemaOutcome::from_record_head("Changed"), None);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExampleCommand {
    Insert,
    Read,
}

impl ToSemaOperation for ExampleCommand {
    fn to_sema_operation(&self) -> SemaOperation {
        match self {
            Self::Insert => SemaOperation::Assert,
            Self::Read => SemaOperation::Match,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExampleEffect {
    Inserted,
    Read,
}

impl ToSemaOutcome for ExampleEffect {
    fn to_sema_outcome(&self) -> SemaOutcome {
        match self {
            Self::Inserted => SemaOutcome::Asserted,
            Self::Read => SemaOutcome::Matched,
        }
    }
}

#[test]
fn component_effects_project_to_sema_outcomes() {
    let cases = [
        (ExampleEffect::Inserted, SemaOutcome::Asserted),
        (ExampleEffect::Read, SemaOutcome::Matched),
    ];

    for (effect, outcome) in cases {
        assert_eq!(effect.to_sema_outcome(), outcome);
    }
}

#[test]
fn sema_outcome_projects_to_itself() {
    for outcome in outcomes() {
        assert_eq!(outcome.to_sema_outcome(), outcome);
    }
}

#[test]
fn sema_observation_composes_command_and_effect_projection() {
    let asserted =
        SemaObservation::from_projection(&ExampleCommand::Insert, &ExampleEffect::Inserted);
    let matched = SemaObservation::from_projection(&ExampleCommand::Read, &ExampleEffect::Read);

    assert_eq!(
        asserted,
        SemaObservation::new(SemaOperation::Assert, SemaOutcome::Asserted)
    );
    assert_eq!(
        matched,
        SemaObservation::new(SemaOperation::Match, SemaOutcome::Matched)
    );
}

#[test]
fn sema_outcomes_round_trip_through_nota() {
    for outcome in outcomes() {
        let mut encoder = Encoder::new();
        outcome.encode(&mut encoder).expect("encode");
        let encoded = encoder.into_string();
        assert_eq!(encoded, outcome.as_record_head());

        let mut decoder = Decoder::new(&encoded);
        let decoded = SemaOutcome::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, outcome);
    }
}

#[test]
fn sema_observation_round_trips_through_nota() {
    let observation = SemaObservation::new(SemaOperation::Subscribe, SemaOutcome::Subscribed);
    let mut encoder = Encoder::new();
    observation.encode(&mut encoder).expect("encode");
    let encoded = encoder.into_string();

    let mut decoder = Decoder::new(&encoded);
    let decoded = SemaObservation::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, observation);
}

#[test]
fn sema_outcomes_round_trip_through_rkyv() {
    for outcome in outcomes() {
        let bytes = rkyv::to_bytes::<RkyvError>(&outcome).expect("archive");
        let archived =
            rkyv::access::<ArchivedSemaOutcome, RkyvError>(&bytes).expect("access archive");
        let decoded: SemaOutcome =
            rkyv::deserialize::<SemaOutcome, RkyvError>(archived).expect("deserialize");
        assert_eq!(decoded, outcome);
    }
}

#[test]
fn sema_observation_round_trips_through_rkyv() {
    let observation = SemaObservation::new(SemaOperation::Retract, SemaOutcome::Retracted);
    let bytes = rkyv::to_bytes::<RkyvError>(&observation).expect("archive");
    let archived =
        rkyv::access::<ArchivedSemaObservation, RkyvError>(&bytes).expect("access archive");
    let decoded: SemaObservation =
        rkyv::deserialize::<SemaObservation, RkyvError>(archived).expect("deserialize");
    assert_eq!(decoded, observation);
}
