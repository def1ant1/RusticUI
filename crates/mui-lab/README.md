# MUI Lab (Rust)

Experimental widgets for the Rust port of Material UI.  APIs in this crate are
**unstable** and may change at any time.  Each component is guarded behind a
Cargo feature so that consumers explicitly opt in to new widgets.

## Stability

The code in this crate is considered pre-production and is not covered by the
standard SemVer guarantees.  Breaking changes may land in any release.  Use it
at your own risk and pin versions accordingly.

## Feature Flags

Each preview widget lives behind a Cargo feature so consumers only compile
what they need:

- `autocomplete` – lightweight string matcher for building suggestion UIs.
- `date-picker` – opt-in to the minimal date picker demonstrating the
  `DateAdapter` abstraction.
- `data-grid` – in-memory tabular data manipulation.
- `time-picker` – enables the time picker powered by `TimeAdapter`.
- `masonry` – experimental Masonry layout algorithm.
- `tree-view` – hierarchical tree structure with expand/collapse state.
- `timeline` – ordered collection of timestamped events.
- `localization` – runtime `LocalizationProvider` and built-in `en-US`
  locale pack (requires `serde`).

Adapters for multiple date/time libraries are also feature gated:

- `chrono` – use the `chrono` crate for date and time math.
- `time` – use the `time` crate.

## Contributing

Community contributions are welcome!  To minimize churn:

- Gate new experimental widgets behind a dedicated Cargo feature.
- Include thorough unit tests covering localization and keyboard navigation.
- Provide locale packs via `LocalizationProvider` so the community can share
  translations.
- Document any new feature in this README before submitting a pull request.

