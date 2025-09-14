# Migrating from React/JS to Rust WebAssembly

Many teams start with a React application and gradually move performance
critical paths to Rust compiled to WebAssembly.  The `mui-*` crates expose the
same design vocabulary as their JavaScript counterparts which makes the
migration incremental.

1. **Identify hot components** – profiler output in React often reveals the
   components that dominate runtime.  Re‑implement these in a Rust framework
   such as [Yew](https://yew.rs) or [Leptos](https://leptos.dev) and compile to
   WebAssembly using `wasm-bindgen`.
2. **Share styles and themes** – the `Theme` structure is serializable with
   `serde`, allowing existing JSON theme definitions to be reused directly in
   Rust.
3. **Leverage cargo workspaces** – colocate Rust crates and JavaScript packages
   in a single repository.  Automated scripts can build both via CI ensuring the
   generated assets stay in lock‑step.
4. **Deploy static artifacts** – `trunk build --release` produces an optimized
   WebAssembly binary.  Serve it from a CDN alongside the `cargo doc` output for
   near‑zero runtime overhead and effortless horizontal scaling.

> Tip: WebAssembly modules are immutable and cacheable.  Enable HTTP cache
> headers so repeat visitors avoid re‑downloading the module.
