//! Native server entry point for the documentation showcase.
//!
//! The binary is designed for containerized execution. It wires tracing
//! subscribers and relies on `rustic_docs::server_main` to stand up the Axum
//! router that supports SSR and static export jobs.

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), rustic_docs::ServerError> {
    init_tracing();
    rustic_docs::server_main().await
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("`rustic-docs-server` must run on a native target");
}

#[cfg(not(target_arch = "wasm32"))]
fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rustic_docs=debug".into()),
        )
        .with_target(false)
        .try_init();
}
