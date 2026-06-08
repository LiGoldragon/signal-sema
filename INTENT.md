# INTENT — signal-sema

*The universal Sema classification vocabulary — Layer 3 of the
three-layer signal model. A pure library crate of payloadless
state-action class labels, read-algebra pattern primitives, the
qualitative magnitude scale, and typed wire-identity values that
components carry inside their own typed records. Companion to
`ARCHITECTURE.md` and `Cargo.toml`. Maintenance:
`primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-sema` crate.
Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Per-component domain vocabulary stays in the `signal-<component>`
contracts; frame mechanics stay in `signal-frame`.

## Why this repo exists

`signal-sema` is the home of **Layer 3** in the three-layer model:

- Layer 1 — Contract Operations: domain verbs on the wire (`Submit`,
  `Query`, `Observe`, …), owned by each `signal-<component>` crate.
- Layer 2 — Component Commands: per-daemon typed executable records,
  owned internally by each daemon.
- Layer 3 — Sema Operations (this crate): the universal *payloadless*
  class labels (`Assert`, `Mutate`, `Retract`, `Match`, `Subscribe`,
  `Validate`) used for cross-component observation and introspection.

Daemons project their Component Commands and effects to Sema classes via
`ToSemaOperation` and `ToSemaOutcome`, so observers can filter
cross-component activity by class without knowing per-daemon command
payloads. `signal-sema` is the universal vocabulary substrate every
per-component schema imports rather than re-declaring.

## What this crate owns

- `SemaOperation` — the closed, payloadless command-classification set,
  with the `OperationClass` projection (`Write` / `Read` / `Stream` /
  `Validation`).
- `SemaOutcome` — the closed, payloadless effect-classification set.
- `SemaObservation` — one operation joined with one outcome; structurally
  a small, universal, classification-only (Tier-2-shaped) type carrying
  no timing, sequence, or component payload.
- `Magnitude` — the workspace-universal qualitative strength scale (the
  ordered rungs `Zero` through `Maximum`), a fixed-size leaf type.
- Pattern primitives — `Bind`, `Wildcard`, `PatternField<T>` — the
  read-algebra position markers daemons reuse inside their typed
  Commands.
- Identity primitives — `Slot<Payload>`, `Revision` — typed wire-identity
  values only.

## Constraints

- This is a Rust library crate. No daemon, actor, socket, redb, or
  runtime code.
- No Persona-specific, Criome-specific, or component-specific payload
  records — those belong to the `signal-<component>` contracts.
- The crate does not depend on `signal-frame`; the frame layer and the
  Sema classification vocabulary are separate concerns (other contract
  crates may depend on both).
- `SemaOperation` and `SemaOutcome` are payloadless — variants never
  carry executable payloads or component event payloads.
- `SemaObservation` joins one operation with one outcome and carries no
  timing, sequence, or component payload data.
- `Magnitude` is a closed, ordered scale today; the vocabulary IS the
  schema, and consumption is per-component policy. Never collapse the
  wire vocabulary to fit a current consumption policy — that forces
  writers to flatten distinctions and replays the drift the universal
  vocabulary exists to prevent.
- Field name carries the dimension; the type carries the scale. Records
  hold `certainty: Magnitude`, `priority: Magnitude`, `severity:
  Magnitude` — no wrapping type like `SizeMagnitude` or
  `IntentCertainty` that ties the universal scale to one domain.
- Type names do not restate the `Sema` or `Signal` namespace; the domain
  is implicit.
- All owned records are rkyv-archivable in the default binary library.
  NOTA encode/decode is an explicit `nota-text` feature for human/agent
  edges and text witnesses; production daemon dependency trees must be
  able to use `signal-sema` without compiling a NOTA parser.

## Non-ownership

This crate does not own:

- public component operation vocabulary (Layer 1, in
  `signal-<component>` crates);
- per-daemon executable Command vocabulary (Layer 2, in each daemon);
- request/reply frame mechanics — the frame envelope, handshake,
  exchange identifiers, async correlation, streams, and reply plumbing
  live in `signal-frame`;
- authorization or routing;
- `ReadPlan` operators (`Constrain`, `Project`, `Aggregate`, `Infer`,
  `Recurse`) — those belong to `sema-engine` and component contracts
  that publish their read plans;
- the `Request<Payload>` envelope (`signal-frame`).

## See also

- `ARCHITECTURE.md` — the classification sets, the magnitude scale, the
  pattern/identity primitives, and the SemaObservation Tier-1/Tier-2
  projection design.
- `primary/skills/component-triad.md` §"Verbs come in three layers" —
  the three-layer model this crate anchors.
- `primary/skills/contract-repo.md` §"Public contracts use
  contract-local operation verbs" — why the six Sema words stay off the
  public wire.
- `../sema-engine/ARCHITECTURE.md` — Sema execution vocabulary and read
  plans.
- `primary/skills/naming.md` — the naming discipline `Magnitude` is the
  canonical leaf example of.
