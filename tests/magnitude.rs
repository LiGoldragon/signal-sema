//! Falsifiable witnesses for the ordered [`Magnitude`] vocabulary.

use nota_next::{NotaEncode, NotaSource};
use rkyv::rancor::Error as RkyvError;
use signal_sema::{ArchivedMagnitude, Magnitude};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn magnitudes() -> [Magnitude; 8] {
    [
        Magnitude::Zero,
        Magnitude::Minimum,
        Magnitude::VeryLow,
        Magnitude::Low,
        Magnitude::Medium,
        Magnitude::High,
        Magnitude::VeryHigh,
        Magnitude::Maximum,
    ]
}

#[test]
fn magnitude_record_heads_are_stable() {
    let cases = [
        (Magnitude::Zero, "Zero"),
        (Magnitude::Minimum, "Minimum"),
        (Magnitude::VeryLow, "VeryLow"),
        (Magnitude::Low, "Low"),
        (Magnitude::Medium, "Medium"),
        (Magnitude::High, "High"),
        (Magnitude::VeryHigh, "VeryHigh"),
        (Magnitude::Maximum, "Maximum"),
    ];

    for (magnitude, head) in cases {
        assert_eq!(magnitude.as_record_head(), head);
        assert_eq!(Magnitude::from_record_head(head), Some(magnitude));
        assert!(
            CANONICAL.lines().any(|line| line == head),
            "examples/canonical.nota missing {head}"
        );
    }

    assert_eq!(Magnitude::from_record_head("VeryMedium"), None);
}

#[test]
fn magnitudes_round_trip_through_nota() {
    for magnitude in magnitudes() {
        let encoded = magnitude.to_nota();
        assert_eq!(encoded, magnitude.as_record_head());

        let decoded = NotaSource::new(&encoded)
            .parse::<Magnitude>()
            .expect("decode");
        assert_eq!(decoded, magnitude);
    }
}

#[test]
fn magnitude_nota_rejects_unknown_head() {
    assert!(NotaSource::new("VeryMedium").parse::<Magnitude>().is_err());
}

#[test]
fn magnitudes_round_trip_through_rkyv() {
    for magnitude in magnitudes() {
        let bytes = rkyv::to_bytes::<RkyvError>(&magnitude).expect("archive");
        let archived =
            rkyv::access::<ArchivedMagnitude, RkyvError>(&bytes).expect("access archive");
        let decoded: Magnitude =
            rkyv::deserialize::<Magnitude, RkyvError>(archived).expect("deserialize");
        assert_eq!(decoded, magnitude);
    }
}

#[test]
fn magnitude_order_is_zero_to_maximum() {
    let magnitudes = magnitudes();

    for pair in magnitudes.windows(2) {
        assert!(
            pair[0] < pair[1],
            "{:?} should be less than {:?}",
            pair[0],
            pair[1]
        );
    }

    assert_eq!(magnitudes.first(), Some(&Magnitude::Zero));
    assert_eq!(magnitudes.last(), Some(&Magnitude::Maximum));
}

#[test]
fn magnitude_storage_discriminants_keep_legacy_rungs_stable() {
    let discriminants = [
        (Magnitude::Minimum as u8, 0),
        (Magnitude::VeryLow as u8, 1),
        (Magnitude::Low as u8, 2),
        (Magnitude::Medium as u8, 3),
        (Magnitude::High as u8, 4),
        (Magnitude::VeryHigh as u8, 5),
        (Magnitude::Maximum as u8, 6),
        (Magnitude::Zero as u8, 7),
    ];

    for (actual, expected) in discriminants {
        assert_eq!(actual, expected);
    }
}

#[test]
fn canonical_examples_cover_every_magnitude() {
    let canonical_heads: Vec<&str> = CANONICAL.lines().collect();

    for magnitude in magnitudes() {
        let head = magnitude.as_record_head();
        let count = canonical_heads.iter().filter(|line| **line == head).count();
        assert_eq!(
            count, 1,
            "examples/canonical.nota must contain {head} exactly once"
        );
    }
}
