# `cargo xtask` command catalog

RusticUI consolidates contributor automation in the [`xtask`](../../crates/xtask) crate
so CI and local workflows execute the same Rust-first logic. This page highlights the
new developer-experience helpers introduced in this release and explains how they
interact with the existing suite of formatters, docs builders, and coverage jobs.

## `cargo xtask new-component`

The `new-component` generator bootstraps every asset required to launch a new
RusticUI surface:

- Headless + Material Rust modules with inline notes describing how to connect
  state machines and telemetry pipelines.
- Jest/mocha-compatible TypeScript scaffolding (`RusticAdapter.tsx`,
  `RusticAdapter.spec.tsx`) that re-export the same automation identifiers as the
  Rust crates.
- Story placeholders (`RusticAdapter.stories.tsx`) so design and QA teams can
  visualise automation metadata before UI code ships.
- Documentation stubs under `docs/src/pages/system/components/` wired with the
  generated automation identifier.
- Headless + Material regression test placeholders to remind teams to backfill
  coverage once the real logic lands.

Run the command from the workspace root:

```bash
cargo xtask new-component MenuSurface --dry-run
```

Key flags:

- `--material-only` / `--headless-only` limit the output set when teams need to
  stage work across multiple releases.
- `--overwrite` allows regenerating scaffolds after manual tweaks.
- `--dry-run` prints the proposed file list without touching disk. CI pipelines
  use this flag to confirm templates stay in sync.

## `cargo xtask dev`

The `dev` command orchestrates long-running hot-reload processes under a single
supervisor so contributors do not juggle separate terminals for the docs site and
example gallery. The helper:

- Launches the Next.js docs server via `pnpm --dir docs run dev`, respecting
  custom host/port flags.
- Boots the Leptos-powered example gallery (`cargo run -p rustic-docs --bin
  rustic-docs-server --features ssr`) while reusing `target/dev` to keep rebuilds
  fast.
- Streams stdout/stderr into `target/logs/dev.log` with human-friendly prefixes,
  making it trivial to audit CI runs or share transcripts.

Usage examples:

```bash
# Preview the plan without spawning processes
cargo xtask dev --dry-run

# Launch only the docs server on a custom port
cargo xtask dev --skip-gallery --docs-port 4200
```

Combine `cargo xtask dev` with the existing docs and quick-start harnesses to
reproduce production parity locally without bespoke shell scripts.
