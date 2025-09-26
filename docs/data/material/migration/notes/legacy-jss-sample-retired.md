# Legacy JSS migration sample retirement

<p class="description">The historical Next.js v4→v5 migration sample has been archived in favour of repeatable, automation-first tooling.</p>

## Why the sample was removed

- The v4-specific JSS stack is no longer maintained in this repository, and keeping a hybrid Emotion + JSS demo incurred unbounded maintenance without delivering production value.
- Modern MUI workspaces should begin from the [`material-ui-nextjs-pages-router`](https://github.com/mui/material-ui/tree/master/examples/material-ui-nextjs-pages-router) samples, which stay in lockstep with the main branch and showcase both JavaScript and TypeScript wiring.
- Our Rust-first automation renders the dedicated migration example redundant because the same verification matrix now runs via typed tasks instead of ad-hoc shell scripts.

## Recommended automation-driven alternatives

- Run `cargo xtask doc` after large refactors to rebuild the documentation stack through a single, audited entry point. This guarantees parity between the Material docs, mdBook, and the rendered SSR shells.
- Use `cargo xtask update-components` whenever you touch shared component scaffolding so the generated TypeScript and Rust bindings stay synchronized.
- Pair those Rust-first commands with the officially supported [JSS codemod](https://github.com/mui/material-ui/blob/master/packages/mui-codemod/README.md#jss-to-styled) to migrate residual styling utilities without manual copy/paste work.
- For CI environments, wire the commands above into your pipelines instead of cloning obsolete starter kits. The xtask binary exposes stable exit codes and verbose logging tailored to large organizations.

## Migration quickstart

1. Start from either [`material-ui-nextjs-pages-router`](https://github.com/mui/material-ui/tree/master/examples/material-ui-nextjs-pages-router) or [`material-ui-nextjs-pages-router-ts`](https://github.com/mui/material-ui/tree/master/examples/material-ui-nextjs-pages-router-ts) depending on your language preference.
2. Apply `npx @mui/codemod@latest v5.0.0/jss-to-styled <path>` to progressively phase out remaining JSS utilities.
3. Execute `cargo xtask doc` followed by `pnpm lint` to ensure the automation stack and linting pass concurrently before shipping.
4. Commit the changes as small, reviewable slices so the automation suite can provide targeted feedback.

By consolidating on the Rust-first toolchain we minimize manual, repetitive steps while scaling the documentation and SSR validation story for enterprise contributors.
