# MUI Lab (Rust)

Experimental widgets for the Rust port of Material UI.  APIs in this crate are
**unstable** and may change at any time.  Each component is guarded behind a
Cargo feature so that consumers explicitly opt in to new widgets.

## Stability

The code in this crate is considered pre-production and is not covered by the
standard SemVer guarantees.  Breaking changes may land in any release.  Use it
at your own risk and pin versions accordingly.

## Contributing

Community contributions are welcome!  To minimize churn:

- Gate new experimental widgets behind a dedicated Cargo feature.
- Include thorough unit tests covering localization and keyboard navigation.
- Provide locale packs via `LocalizationProvider` so the community can share
  translations.
- Document any new feature in this README before submitting a pull request.

