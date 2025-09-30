# Shared InputBase blueprint utilities

This crate keeps the InputBase form examples aligned across frameworks.
It centralizes:

- The analytics namespace and automation markers that QA tooling consumes.
- Opinionated `InputState` builders for both controlled and uncontrolled flows.
- Server rendering helpers used by the bootstrap scripts to generate deterministic snapshots.

Update this crate whenever you introduce a new attribute or behaviour in the Material `InputBase` renderer.
Downstream examples call into these helpers so CI picks up the new behaviour automatically instead of relying on copy/pasted snippets.
