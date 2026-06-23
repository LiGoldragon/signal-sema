//! Falsifiable witnesses for the [`SemaOperation`] vocabulary and
//! the [`OperationClass`] classification.
//!
//! If a future change drifts the canonical NOTA text shape, the
//! rkyv archive layout, or the operation-class mapping, exactly one
//! of these tests breaks and names the variant that drifted.

#[cfg(feature = "nota-text")]
use nota::{NotaEncode, NotaSource};
use rkyv::rancor::Error as RkyvError;
use signal_sema::{ArchivedSemaOperation, OperationClass, SemaOperation, ToSemaOperation};

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
fn sema_operation_is_write_matches_class() {
    for operation in operations() {
        let class = operation.class();
        let expected = matches!(class, OperationClass::Write);
        assert_eq!(
            operation.is_write(),
            expected,
            "is_write disagreed with class for {operation:?} ({class:?})"
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExampleCommand {
    Insert,
    Read,
    OpenStream,
}

impl ToSemaOperation for ExampleCommand {
    fn to_sema_operation(&self) -> SemaOperation {
        match self {
            Self::Insert => SemaOperation::Assert,
            Self::Read => SemaOperation::Match,
            Self::OpenStream => SemaOperation::Subscribe,
        }
    }
}

#[test]
fn component_commands_project_to_sema_operation_classes() {
    let cases = [
        (ExampleCommand::Insert, SemaOperation::Assert),
        (ExampleCommand::Read, SemaOperation::Match),
        (ExampleCommand::OpenStream, SemaOperation::Subscribe),
    ];

    for (command, operation) in cases {
        assert_eq!(command.to_sema_operation(), operation);
    }
}

#[test]
fn sema_operation_projects_to_itself() {
    for operation in operations() {
        assert_eq!(operation.to_sema_operation(), operation);
    }
}

#[test]
#[cfg(feature = "nota-text")]
fn sema_operations_round_trip_through_nota() {
    for operation in operations() {
        let encoded = operation.to_nota();
        assert_eq!(encoded, operation.as_record_head());

        let decoded = NotaSource::new(&encoded)
            .parse::<SemaOperation>()
            .expect("decode");
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
