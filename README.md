# signal-sema

Sema-engine operation vocabulary: `Assert / Mutate / Retract / Match /
Subscribe / Validate`. Speakable on the wire when a component
explicitly exposes typed-table operations; consumed as a library by
component daemons when they lower their public domain operations into
typed-record state effects.

This crate is part of the signal-architecture migration that splits
the former `signal-core` into `signal-frame` (frame mechanics) and
`signal-sema` (Sema execution vocabulary). The migration plan lives
in the primary workspace at
`reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`
and `reports/designer/239-signal-architecture-migration-plan.md`.

## What this crate owns

- `SemaOperation` — the closed six-operation set.
- `OperationClass` — broad classification of operation effect.
- `Bind`, `Wildcard`, `PatternField<T>` — the read-algebra pattern
  primitives that pair with `Match` and `Subscribe` payloads.
- `Slot<Payload>`, `Revision` — the typed-record identity values
  that `Mutate` and `Retract` address state with.

## What this crate does not own

- Frame envelope, handshake, exchange identifiers, async
  correlation, streams, reply plumbing — those live in
  `signal-frame`.
- Public component operation vocabulary — each `signal-<component>`
  defines its own domain verbs.
- `ReadPlan` operators (`Constrain` / `Project` / `Aggregate` /
  `Infer` / `Recurse`) — those belong in `sema-engine` and in
  component contracts that publish their read plans.

See `ARCHITECTURE.md` for the layer diagram and the full constraint
set.
