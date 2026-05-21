//! Sema classification and observation vocabulary.
//!
//! Public component contracts use domain-local operation verbs. This
//! crate owns the payloadless operation and outcome labels that a
//! daemon projects from its component-local executable commands and
//! effects.
//!
//! See `ARCHITECTURE.md` for the layer position and what the crate
//! does and does not own. The migration plan that introduced this
//! crate lives in the primary workspace as
//! `reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`
//! and
//! `reports/designer/239-signal-architecture-migration-plan.md`.

pub mod identity;
pub mod magnitude;
pub mod operation;
pub mod outcome;
pub mod pattern;

pub use identity::{ArchivedRevision, ArchivedSlot, Revision, Slot};
pub use magnitude::{ArchivedMagnitude, Magnitude};
pub use operation::{ArchivedSemaOperation, OperationClass, SemaOperation, ToSemaOperation};
pub use outcome::{
    ArchivedSemaObservation, ArchivedSemaOutcome, SemaObservation, SemaOutcome, ToSemaOutcome,
};
pub use pattern::{
    ArchivedBind, ArchivedPatternField, ArchivedWildcard, Bind, PatternField, Wildcard,
};
