# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
### Added
- Checkbox toggles now leverage a tri-state enum (`Off`/`On`/`Indeterminate`)
  with helpers for programmatically setting and clearing the indeterminate
  state while preserving controlled/uncontrolled ergonomics.
- `aria::aria_checked` accepts the new `AriaChecked` helper so adapters emit
  `mixed` values alongside a `data-indeterminate` flag for animation hooks.
