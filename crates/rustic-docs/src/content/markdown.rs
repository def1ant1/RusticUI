//! Markdown ingestion for the documentation shell.
//!
//! Static files from the legacy Next.js site are embedded at compile time using
//! `include_str!`.  Rendering is handled by `pulldown_cmark`, keeping the
//! pipeline entirely Rust driven and eliminating the need for Node-based
//! preprocessing.

use leptos::{view, IntoView};
use pulldown_cmark::{html, Options, Parser};
use serde::Serialize;

/// Metadata attached to each embedded Markdown file.
#[derive(Clone, Debug, Serialize)]
pub struct MarkdownDocument {
    /// Relative path to the source asset under `docs/`.
    pub source_path: &'static str,
    /// Route hint derived from the file-system layout.
    pub route_hint: &'static str,
    /// Human friendly title inferred from the file name.
    pub title: &'static str,
    /// Raw Markdown content embedded via `include_str!`.
    pub body: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/docs_markdown.rs"));

/// Returns the embedded Markdown documents ordered by path.
#[must_use]
pub fn markdown_documents() -> &'static [MarkdownDocument] {
    MARKDOWN_DOCUMENTS
}

/// Locate a Markdown document by its route hint.
#[must_use]
pub fn find_document_by_route(route: &str) -> Option<&'static MarkdownDocument> {
    markdown_documents()
        .iter()
        .find(|doc| doc.route_hint == route)
}

/// Render the Markdown body into HTML using `pulldown_cmark`.
#[must_use]
pub fn render_markdown_to_html(doc: &MarkdownDocument) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(doc.body, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}

/// Leptos component rendering a Markdown document identified by `route_hint`.
#[allow(
    missing_docs,
    reason = "Leptos macro emits generated prop structs without doc attributes."
)]
#[leptos::component]
pub fn MarkdownArticle(
    #[doc = "Route hint used to locate the embedded Markdown document during hydration."]
    #[prop(into)]
    route_hint: String,
) -> impl IntoView {
    let rendered = find_document_by_route(route_hint.as_str())
        .map(render_markdown_to_html)
        .unwrap_or_else(|| format!("<p>We do not have a Markdown document for {route_hint}.</p>"));

    view! { <article class="docs-markdown" inner_html=rendered></article> }
}
