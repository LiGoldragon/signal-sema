//! Falsifiable witnesses for the ordered [`Magnitude`] vocabulary.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use rkyv::rancor::Error as RkyvError;
use signal_sema::{ArchivedMagnitude, Magnitude};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn magnitudes() -> [Magnitude; 7] {
    [
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
        let mut encoder = Encoder::new();
        magnitude.encode(&mut encoder).expect("encode");
        let encoded = encoder.into_string();
        assert_eq!(encoded, magnitude.as_record_head());

        let mut decoder = Decoder::new(&encoded);
        let decoded = Magnitude::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, magnitude);
    }
}

#[test]
fn magnitude_nota_rejects_unknown_head() {
    let mut decoder = Decoder::new("VeryMedium");
    assert!(Magnitude::decode(&mut decoder).is_err());
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
fn magnitude_order_is_minimum_to_maximum() {
    let magnitudes = magnitudes();

    for pair in magnitudes.windows(2) {
        assert!(
            pair[0] < pair[1],
            "{:?} should be less than {:?}",
            pair[0],
            pair[1]
        );
    }

    assert_eq!(magnitudes.first(), Some(&Magnitude::Minimum));
    assert_eq!(magnitudes.last(), Some(&Magnitude::Maximum));
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
