//! Content orchestration for the documentation experience.
//!
//! This module stitches together Markdown documents, component inventories and
//! framework-specific renderers.  The build script generates static metadata so
//! the runtime can remain lean while still reflecting the file-system structure
//! of the legacy `docs/` Next.js implementation.

pub mod inventory;
pub mod leptos_components;
pub mod markdown;
#[cfg(feature = "yew-docs")]
pub mod yew_components;

pub use inventory::{
    docs_inventory, FrameworkPlan, InventoryCategory, InventoryEntry, LocalizedRoute,
};
pub use markdown::{
    find_document_by_route, markdown_documents, render_markdown_to_html, MarkdownArticle,
    MarkdownDocument,
};
