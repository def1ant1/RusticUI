// build.rs intentionally documents how SSR/static export orchestration is wired.
//
// The script does not perform heavy work; instead it registers the knobs that
// CI/CD platforms and release automation can toggle. By emitting `rerun` hints
// we ensure that when infrastructure teams change the export directory or
// observability wiring, Cargo recompiles the crate with the updated metadata.
//
// The inline comments outline how enterprise adopters can attach scaling hooks
// (for example rotating CDN buckets or multi-region telemetry endpoints) while
// keeping the build pipeline reproducible.

fn main() {
    // Allow operators to override where static snapshots are exported. This is
    // particularly useful when plugging the crate into managed artifact stores
    // such as AWS S3 or GCS. The environment variable is intentionally named to
    // be self-explanatory for platform engineers.
    println!("cargo:rerun-if-env-changed=RUSTIC_DOCS_EXPORT_DIR");

    // Document where distributed tracing collectors may be defined. Runtime
    // initialisation reads the same variable, giving enterprises a single place
    // to set observability targets without duplicating configuration across
    // build scripts and server binaries.
    println!("cargo:rerun-if-env-changed=RUSTIC_DOCS_TRACING_ENDPOINT");
}
