# signal-sema Skills

This repository follows the workspace skills in `/home/li/primary/skills/`.

## Local Discipline

- Keep the crate small and vocabulary-shaped.
- Do not add runtime dependencies.
- Do not introduce component-domain names.
- Do not reintroduce `SignalVerb`; the Sema word is `SemaOperation`,
  and under the three-layer model it is a *payloadless classification*
  label only — never an executable carrier.
- Do not add executable payloads to `SemaOperation` variants. Typed
  executable records (Layer 2 Component Commands) belong in each
  component's daemon crate, not here.
- Add a constraint test when adding an architectural rule.
