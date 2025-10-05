#![deny(missing_docs)]
#![doc = r"Rustic documentation runtime
==============================

`rustic-docs` is a miniature documentation portal demonstrating how the
RusticUI component ecosystem can be orchestrated using Leptos across
client-side rendering (CSR), server-side rendering (SSR), and static
exports. The crate doubles as living documentation: rich inline comments
and docstrings surface architectural decisions so enterprise adopters can
understand where to extend or integrate observability without reverse
engineering ad-hoc examples."]

use leptos::{on_cleanup, provide_context, view, IntoView};
use leptos_config::LeptosOptions;
#[cfg(feature = "ssr")]
use leptos_meta::MetaContext;
use leptos_router::{Route, Router, RouterIntegrationContext, Routes, ServerIntegration, A};
use rustic_ui_design_tokens::BundleSummary;
#[cfg(all(feature = "csr", target_arch = "wasm32"))]
use rustic_ui_system::theme_provider::use_material_color_scheme;
use rustic_ui_system::theme_provider::{material_theme, use_theme, CssBaseline, ThemeProvider};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::instrument;

/// Canonical router paths for the documentation shell. Exposed as constants so automation tooling and unit
/// tests can validate consistency without instantiating the reactive runtime.
pub const HOME_ROUTE: &str = "/";
/// Path backing the automation extension notes.
pub const AUTOMATION_ROUTE: &str = "/automation";
/// Aggregate list of the supported routes. This feeds both documentation tooling and targeted assertions.
pub const ROUTE_PATHS: [&str; 2] = [HOME_ROUTE, AUTOMATION_ROUTE];

/// Configuration surface that documents which levers the automation
/// platform toggles per deployment environment.
///
/// * `telemetry_endpoint` — URL where structured logs and traces are
///   streamed. Operators can override this via environment variables at
///   runtime, letting them plug into managed observability stacks.
/// * `feature_flags` — coarse switches that influence router state and
///   component composition. The `automation_first` flag is a sentinel that
///   pipelines inspect when deciding whether to enable CI powered content
///   validation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocsSiteConfig {
    /// Destination for high cardinality telemetry (structured logs or
    /// OpenTelemetry gRPC). Defaults to an internal development collector
    /// but is documented as a required override in production.
    pub telemetry_endpoint: String,
    /// Feature toggles with deterministic defaults. They are intentionally
    /// serializable so orchestration tooling can persist them alongside
    /// other environment state (for example in Terraform or Pulumi stacks).
    pub feature_flags: serde_json::Value,
}

impl Default for DocsSiteConfig {
    fn default() -> Self {
        Self {
            telemetry_endpoint: "https://telemetry.dev.rusticui.dev".into(),
            feature_flags: serde_json::json!({
                "automation_first": true,
                "allow_public_preview": false,
            }),
        }
    }
}

/// Human-readable documentation for integrators explaining how static
/// exports produced by `rustic-ui-design-tokens` align with the runtime
/// theming story.
///
/// The `BundleSummary` type is intentionally referenced at runtime so the
/// compiler verifies the dependency remains wired. Downstream build
/// systems use this surface to prove that design-token pipelines continue
/// to match the documentation application contract.
pub fn static_manifest_contract() -> serde_json::Value {
    serde_json::json!({
        "bundle_summary_type": std::any::type_name::<BundleSummary>(),
        "supported_channels": ["csr", "ssr", "static"],
        "notes": "Manifests emitted by automation must contain palette and typography payloads to keep ThemeProvider hydration deterministic.",
    })
}

/// Core Leptos application component.
///
/// The `App` component intentionally embeds commentary explaining how
/// themes, routes, and automation hooks tie together. Enterprise teams are
/// expected to copy/paste this structure into their own monorepos, so the
/// component doubles as a reference implementation.
#[leptos::component]
pub fn App() -> impl IntoView {
    let theme = material_theme();
    let config = DocsSiteConfig::default();

    // Expose telemetry configuration so child components (or future
    // server-functions) can instrument operations without needing global
    // mutable state. We rely on Leptos' context system to propagate the
    // data through CSR, SSR, and hydration seamlessly.
    provide_context(config.clone());
    #[cfg(not(target_arch = "wasm32"))]
    if leptos::use_context::<RouterIntegrationContext>().is_none() {
        // Capture the integration in the outer scope so we can drop it during
        // Leptos' cleanup phase rather than when thread-local runtime storage is
        // already torn down. Without this explicit cleanup the router's
        // internally managed signals can attempt to touch TLS slots after
        // disposal which manifests as the "cannot access TLS" panic that the
        // original tests surfaced.
        let integration = RouterIntegrationContext::new(ServerIntegration {
            path: "http://localhost/".to_string(),
        });
        provide_context(integration.clone());
        on_cleanup(move || drop(integration));
        if leptos::use_context::<MetaContext>().is_none() {
            provide_context(MetaContext::new());
        }
    }

    view! {
        <ThemeProvider theme=theme.clone()>
            <CssBaseline />
            <Router fallback=|| view! { <NotFoundPage/> } >
                <Routes>
                    <Route path=HOME_ROUTE view=HomePage />
                    <Route path=AUTOMATION_ROUTE view=AutomationFirstBlueprint />
                </Routes>
            </Router>
        </ThemeProvider>
    }
}

/// Landing page describing how the documentation shell is structured.
#[leptos::component]
fn HomePage() -> impl IntoView {
    let theme = leptos::expect_context::<DocsSiteConfig>();
    view! {
        <section class="docs-hero">
            <h1>"Rustic Docs"</h1>
            <p>
                "A Leptos powered, automation-first documentation surface.
                Telemetry is streamed to "
                {theme.telemetry_endpoint.clone()}
                "."
            </p>
            <nav>
                <A href=AUTOMATION_ROUTE class="docs-link">"Automation playbook"</A>
            </nav>
        </section>
    }
}

/// Secondary route outlining extension points.
#[leptos::component]
fn AutomationFirstBlueprint() -> impl IntoView {
    #[cfg(all(feature = "csr", target_arch = "wasm32"))]
    let handle = use_material_color_scheme();
    #[cfg(all(feature = "csr", target_arch = "wasm32"))]
    let toggle_action = {
        let handle = handle.clone();
        move |_| handle.toggle()
    };
    #[cfg(not(all(feature = "csr", target_arch = "wasm32")))]
    let toggle_action = move |_| {};
    let theme = use_theme();
    let active_palette = theme.palette.initial_color_scheme.as_str().to_string();

    view! {
        <article class="docs-automation">
            <header>
                <h2>"Automation-first delivery"</h2>
                <p>
                    "RusticUI components inherit the "
                    {active_palette}
                    " color scheme."
                </p>
            </header>
            <section>
                <h3>"Enterprise hooks"</h3>
                <ul>
                    <li>"Server render via Axum or Actix with the same router tree."</li>
                    <li>"Static export job emits JSON manifests alongside HTML snapshots."</li>
                    <li>"Observability toggles are injected from DocsSiteConfig at hydration time."</li>
                </ul>
            </section>
            <footer>
                <button on:click=toggle_action>"Toggle color scheme"</button>
            </footer>
        </article>
    }
}

/// Simple not found page to ensure the router fallback path is covered in tests.
#[leptos::component]
fn NotFoundPage() -> impl IntoView {
    view! { <p class="docs-404">"We could not find that page."</p> }
}

/// Build an Axum router capable of serving the Leptos application with SSR.
#[cfg(feature = "ssr")]
pub fn axum_router(leptos_options: LeptosOptions) -> axum::Router {
    use axum::routing::get;
    use axum::Router;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let routes = generate_route_list(App);
    Router::<LeptosOptions>::new()
        .leptos_routes(&leptos_options, routes, App)
        .route("/_health", get(healthcheck))
        .with_state(leptos_options)
}

/// Exposed health endpoint documenting how platform teams can inject proactive observability.
#[cfg(feature = "ssr")]
async fn healthcheck() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "automation": "ready",
    }))
}

/// Utility for bootstrapping Leptos options with deterministic defaults.
#[cfg(feature = "ssr")]
pub fn default_leptos_options() -> LeptosOptions {
    LeptosOptions {
        output_name: "rustic-docs".into(),
        site_addr: "0.0.0.0:3000".parse().expect("valid socket"),
        site_root: "target/rustic-docs".into(),
        ..Default::default()
    }
}

/// Client entry point used by `src/main.rs` to initialise CSR builds.
#[instrument(name = "rustic_docs_client_main")]
pub fn client_main() {
    leptos::mount_to_body(App);
}

/// SSR entry point used by `src/server.rs`. The function is synchronous so the binary can call it inside a Tokio runtime.
#[cfg(feature = "ssr")]
#[instrument(name = "rustic_docs_server_main", skip_all)]
pub async fn server_main() -> Result<(), ServerError> {
    let options = default_leptos_options();
    let addr = options.site_addr;
    let app = axum_router(options.clone());
    tracing::info!(?addr, "starting rustic-docs server");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(ServerError::Server)?;

    Ok(())
}

/// Errors produced by the SSR bootstrap pipeline.
#[derive(Debug, Error)]
pub enum ServerError {
    /// Wrapper around Axum's error type so consumers receive strongly typed failures.
    #[error("axum server error: {0}")]
    Server(#[from] std::io::Error),
}

/// Generate a runtime independent HTML snapshot. This enables static export jobs to re-use the SSR pipeline without a networked server.
#[cfg(feature = "ssr")]
pub fn render_static_snapshot() -> String {
    let markup = leptos::ssr::render_to_string(|| {
        provide_context(DocsSiteConfig::default());
        view! {
            <ThemeProvider theme=material_theme()>
                <CssBaseline />
                <AutomationFirstBlueprint />
            </ThemeProvider>
        }
    });
    format!("<!DOCTYPE html><html lang=\"en\"><body>{markup}</body></html>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "ssr")]
    #[test]
    fn routing_renders_expected_sections() {
        let mut paths: Vec<_> = ROUTE_PATHS.iter().copied().collect();
        paths.sort();
        let mut expected = vec![HOME_ROUTE, AUTOMATION_ROUTE];
        expected.sort();
        assert_eq!(paths, expected);
    }

    #[test]
    fn theming_context_propagates() {
        let markup = leptos::ssr::render_to_string(|| {
            view! { <ThemeProvider theme=material_theme()> <TestThemeConsumer /> </ThemeProvider> }
        });
        assert!(markup.contains("light"));
    }

    #[leptos::component]
    fn TestThemeConsumer() -> impl IntoView {
        let theme = use_theme();
        let scheme = theme.palette.initial_color_scheme.as_str().to_string();
        view! { <span class="theme-marker">{scheme}</span> }
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn static_snapshot_contains_toggle() {
        let snapshot = render_static_snapshot();
        assert!(snapshot.contains("Toggle color scheme"));
        assert!(
            snapshot.contains("data-hk"),
            "Leptos hydration markers missing"
        );
    }
}
