//! Sema execution vocabulary.
//!
//! Public component contracts use domain-local operation verbs. This
//! crate owns the lower database-operation vocabulary that a daemon
//! lowers into when it asks `sema-engine` to read or change durable
//! state.
//!
//! See `ARCHITECTURE.md` for the layer position and what the crate
//! does and does not own. The migration plan that introduced this
//! crate lives in the primary workspace as
//! `reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`
//! and
//! `reports/designer/239-signal-architecture-migration-plan.md`.

pub mod identity;
pub mod operation;
pub mod pattern;

pub use identity::{ArchivedRevision, ArchivedSlot, Revision, Slot};
pub use operation::{ArchivedSemaOperation, OperationClass, SemaOperation, ToSemaOperation};
pub use pattern::{
    ArchivedBind, ArchivedPatternField, ArchivedWildcard, Bind, PatternField, Wildcard,
};
