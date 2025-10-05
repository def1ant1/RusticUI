//! Generated inventory describing every legacy documentation surface.
//!
//! The build script produces the `DOCS_INVENTORY` constant so the runtime can
//! inspect which Markdown files, page templates and demo sources existed in the
//! historical Next.js site.  Each entry also carries suggested Leptos/Yew
//! component names to help teams mirror the layout in Rust without spelunking
//! through TypeScript.

use serde::Serialize;

/// High level classification of a documentation asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum InventoryCategory {
    /// Legacy `docs/src/modules/components` entries containing demo logic.
    Component,
    /// React pages that will be reimplemented using Leptos/Yew routers.
    Page,
    /// Data files (YAML/JSON/Markdown) backing the docs experience.
    Data,
}

/// Suggested component wiring for a framework adapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct FrameworkPlan {
    /// Target framework ("leptos" or "yew").
    pub framework: &'static str,
    /// Proposed component name for the reimplementation.
    pub component: &'static str,
    /// Module housing shared helpers.
    pub module_path: &'static str,
    /// Notes describing which RusticUI primitives to compose.
    pub notes: &'static str,
}

/// Locale aware routing hint for the documentation shell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct LocalizedRoute {
    /// BCP-47 locale identifier.
    pub locale: &'static str,
    /// Route segment served to the browser.
    pub path: &'static str,
}

/// Metadata describing a single documentation asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct InventoryEntry {
    /// Relative path to the legacy asset under the `docs/` directory.
    pub source_path: &'static str,
    /// High level grouping for the asset.
    pub category: InventoryCategory,
    /// Route hint derived from the original Next.js setup.
    pub route_hint: &'static str,
    /// Recommended Leptos/Yew hooks for rebuilding the surface.
    pub frameworks: &'static [FrameworkPlan],
    /// Locale specific route hints.
    pub locales: &'static [LocalizedRoute],
    /// RusticUI primitives that should be composed when porting the asset.
    pub recommended_primitives: &'static [&'static str],
    /// Commentary explaining automation hooks or extension points.
    pub notes: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/docs_inventory.rs"));

/// Returns the generated documentation inventory.
#[must_use]
pub fn docs_inventory() -> &'static [InventoryEntry] {
    DOCS_INVENTORY
}
