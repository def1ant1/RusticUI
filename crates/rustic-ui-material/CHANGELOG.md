# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
### Added
- Centralised overlay helpers (`portal`, `modal`, `popover`, `popper`,
  `transition`) that wrap the headless state machines with Material themed
  attribute builders, automation ids, and SSR ready HTML serializers for multi
  framework adapters.
- App bar adapters now hydrate from the headless `AppBarState`, aligning
  automation ids, analytics markers, and SSR friendly attributes across Yew,
  Leptos, Dioxus, and Sycamore renderers.
- Material renderers for alerts, backdrops, form controls, input adornments,
  sliders, circular progress, linear progress, and skeleton placeholders built
  on the shared headless state machines with centralized styling helpers.
