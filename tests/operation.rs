use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use rkyv::rancor::Error as RkyvError;
use signal_sema::{ArchivedSemaOperation, OperationClass, SemaOperation};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn operations() -> [SemaOperation; 6] {
    [
        SemaOperation::Assert,
        SemaOperation::Mutate,
        SemaOperation::Retract,
        SemaOperation::Match,
        SemaOperation::Subscribe,
        SemaOperation::Validate,
    ]
}

#[test]
fn sema_operation_record_heads_are_stable() {
    let cases = [
        (SemaOperation::Assert, "Assert"),
        (SemaOperation::Mutate, "Mutate"),
        (SemaOperation::Retract, "Retract"),
        (SemaOperation::Match, "Match"),
        (SemaOperation::Subscribe, "Subscribe"),
        (SemaOperation::Validate, "Validate"),
    ];

    for (operation, head) in cases {
        assert_eq!(operation.as_record_head(), head);
        assert_eq!(SemaOperation::from_record_head(head), Some(operation));
        assert!(
            CANONICAL.lines().any(|line| line == head),
            "examples/canonical.nota missing {head}"
        );
    }

    assert_eq!(SemaOperation::from_record_head("Submit"), None);
}

#[test]
fn sema_operation_classes_are_explicit() {
    assert_eq!(SemaOperation::Assert.class(), OperationClass::Write);
    assert_eq!(SemaOperation::Mutate.class(), OperationClass::Write);
    assert_eq!(SemaOperation::Retract.class(), OperationClass::Write);
    assert_eq!(SemaOperation::Match.class(), OperationClass::Read);
    assert_eq!(SemaOperation::Subscribe.class(), OperationClass::Stream);
    assert_eq!(SemaOperation::Validate.class(), OperationClass::Validation);
}

#[test]
fn sema_operations_round_trip_through_nota() {
    for operation in operations() {
        let mut encoder = Encoder::new();
        operation.encode(&mut encoder).expect("encode");
        let encoded = encoder.into_string();
        assert_eq!(encoded, operation.as_record_head());

        let mut decoder = Decoder::new(&encoded);
        let decoded = SemaOperation::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, operation);
    }
}

#[test]
fn sema_operations_round_trip_through_rkyv() {
    for operation in operations() {
        let bytes = rkyv::to_bytes::<RkyvError>(&operation).expect("archive");
        let archived =
            rkyv::access::<ArchivedSemaOperation, RkyvError>(&bytes).expect("access archive");
        let decoded: SemaOperation =
            rkyv::deserialize::<SemaOperation, RkyvError>(archived).expect("deserialize");
        assert_eq!(decoded, operation);
    }
}
