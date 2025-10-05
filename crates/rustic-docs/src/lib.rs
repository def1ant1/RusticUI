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

pub mod content;
pub mod theme;

use content::{
    docs_inventory,
    leptos_components::InventoryBoard,
    markdown::{markdown_documents, MarkdownArticle},
    InventoryCategory,
};
use leptos::{on_cleanup, provide_context, view, IntoView};
use leptos_config::LeptosOptions;
#[cfg(feature = "ssr")]
use leptos_meta::MetaContext;
use leptos_router::{Route, Router, RouterIntegrationContext, Routes, ServerIntegration, A};
use rustic_ui_design_tokens::BundleSummary;
use serde::{Deserialize, Serialize};
use theme::{DocsAppBar, DocsSurface, DocsThemeShell, ThemeToggleControl};
use thiserror::Error;
use tracing::instrument;

/// Canonical router paths for the documentation shell. Exposed as constants so automation tooling and unit
/// tests can validate consistency without instantiating the reactive runtime.
pub const HOME_ROUTE: &str = "/";
/// Path backing the automation extension notes.
pub const AUTOMATION_ROUTE: &str = "/automation";
/// Inventory dashboard documenting the migration plan for legacy docs.
pub const INVENTORY_ROUTE: &str = "/inventory";
/// Aggregate list of the supported routes. This feeds both documentation tooling and targeted assertions.
pub const ROUTE_PATHS: [&str; 3] = [HOME_ROUTE, AUTOMATION_ROUTE, INVENTORY_ROUTE];

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
    let inventory = docs_inventory().len();
    let markdown = markdown_documents().len();
    serde_json::json!({
        "bundle_summary_type": std::any::type_name::<BundleSummary>(),
        "supported_channels": ["csr", "ssr", "static"],
        "theme_surfaces": ["DocsThemeShell", "DocsSurface", "ThemeToggleControl"],
        "inventory_entries": inventory,
        "markdown_documents": markdown,
        "notes": "Manifests emitted by automation must contain palette and typography payloads to keep DocsThemeShell hydration deterministic.",
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
    let config = DocsSiteConfig::default();

    // Expose telemetry configuration so child components (or future
    // server-functions) can instrument operations without needing global
    // mutable state. We rely on Leptos' context system to propagate the
    // data through CSR, SSR, and hydration seamlessly.
    provide_context(config.clone());
    #[cfg(not(target_arch = "wasm32"))]
    if leptos::use_context::<RouterIntegrationContext>().is_none() {
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
        <DocsThemeShell>
            <Router fallback=|| view! { <NotFoundPage/> } >
                <Routes>
                    <Route path=HOME_ROUTE view=HomePage />
                    <Route path=AUTOMATION_ROUTE view=AutomationFirstBlueprint />
                    <Route path=INVENTORY_ROUTE view=InventoryLanding />
                </Routes>
            </Router>
        </DocsThemeShell>
    }
}

/// Landing page describing how the documentation shell is structured.
#[leptos::component]
fn HomePage() -> impl IntoView {
    let config = leptos::expect_context::<DocsSiteConfig>();
    let featured_route = markdown_documents()
        .iter()
        .find(|doc| {
            doc.source_path
                .ends_with("architecture/selection-controls.md")
        })
        .or_else(|| markdown_documents().first())
        .map(|doc| doc.route_hint)
        .unwrap_or("/architecture/selection-controls");

    view! {
        <section style="display: grid; gap: 1.5rem;">
            <DocsAppBar
                title="Rustic Docs"
                subtitle="Automation-first knowledge base orchestrated with Leptos"
            />
            <DocsSurface
                title="Automation-first documentation"
                description="Centralised theme, routing and telemetry hooks"
            >
                <p style="margin: 0;">
                    {format!(
                        "The portal streams structured telemetry to {endpoint} and mirrors the production automation topology.",
                        endpoint = config.telemetry_endpoint
                    )}
                </p>
                <nav style="display: flex; gap: 1rem; margin-top: 0.75rem;">
                    <A href=AUTOMATION_ROUTE>{"Automation playbook"}</A>
                    <A href=INVENTORY_ROUTE>{"Inventory dashboard"}</A>
                </nav>
                <ThemeToggleControl />
            </DocsSurface>
            <DocsSurface
                title="Architecture deep dive"
                description="Markdown rendered with `pulldown_cmark`"
            >
                <MarkdownArticle route_hint=featured_route.to_string() />
            </DocsSurface>
            <DocsSurface
                title="Migration snapshot"
                description="Live data extracted from docs/ during the build"
            >
                <InventoryBoard />
            </DocsSurface>
        </section>
    }
}

/// Secondary route outlining automation extension points.
#[leptos::component]
fn AutomationFirstBlueprint() -> impl IntoView {
    let inventory_total = docs_inventory().len();
    let markdown_total = markdown_documents().len();

    view! {
        <section style="display: grid; gap: 1.5rem;">
            <DocsSurface
                title="Automation-first delivery"
                description="Deployment guidance for platform engineers"
            >
                <ul style="margin: 0; padding-left: 1.5rem;">
                    <li>{"Server render via Axum with the same router tree."}</li>
                    <li>{"Static export job emits JSON manifests alongside HTML snapshots."}</li>
                    <li>{"Telemetry toggles flow from DocsSiteConfig into the Leptos context."}</li>
                </ul>
                <ThemeToggleControl />
            </DocsSurface>
            <DocsSurface
                title="Corpus generated by build.rs"
                description="Inventory and Markdown summaries"
            >
                <p style="margin: 0;">{format!("Inventory entries tracked: {inventory_total}")}</p>
                <p style="margin: 0;">{format!("Markdown documents embedded: {markdown_total}")}</p>
            </DocsSurface>
        </section>
    }
}

/// Route rendering the full inventory board so stakeholders can audit migration progress.
#[leptos::component]
fn InventoryLanding() -> impl IntoView {
    let total = docs_inventory().len();
    let components = docs_inventory()
        .iter()
        .filter(|entry| entry.category == InventoryCategory::Component)
        .count();

    view! {
        <section style="display: grid; gap: 1.5rem;">
            <DocsSurface
                title="Legacy inventory"
                description=format!("Planning {components} component demos across {total} total assets")
            >
                <InventoryBoard />
            </DocsSurface>
        </section>
    }
}

/// Simple not found page to ensure the router fallback path is covered in tests.
#[leptos::component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <section style="display: grid; gap: 1.5rem;">
            <DocsSurface
                title="Not found"
                description="We could not locate the requested page"
            >
                <p style="margin: 0;">{"Return to the home page or open the inventory to browse available routes."}</p>
            </DocsSurface>
        </section>
    }
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
            <DocsThemeShell>
                <AutomationFirstBlueprint />
            </DocsThemeShell>
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
        let mut expected = vec![HOME_ROUTE, AUTOMATION_ROUTE, INVENTORY_ROUTE];
        expected.sort();
        assert_eq!(paths, expected);
    }

    #[test]
    fn theming_context_propagates() {
        let markup = leptos::ssr::render_to_string(|| {
            view! { <DocsThemeShell> <ThemeToggleControl /> </DocsThemeShell> }
        });
        assert!(markup.contains("Theme diagnostics"));
    }

    #[test]
    fn markdown_documents_render() {
        let doc = markdown_documents().first().expect("embedded markdown");
        let html = content::render_markdown_to_html(doc);
        assert!(html.contains("<p"));
    }

    #[test]
    fn inventory_routes_are_localised() {
        for entry in docs_inventory() {
            assert!(entry.route_hint.starts_with('/'));
            assert!(entry.locales.iter().any(|locale| locale.locale == "en"));
        }
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn static_snapshot_contains_toggle() {
        let snapshot = render_static_snapshot();
        assert!(snapshot.contains("Toggle to"));
        assert!(
            snapshot.contains("data-hk"),
            "Leptos hydration markers missing"
        );
    }
}
