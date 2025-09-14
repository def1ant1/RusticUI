//! Server side style collection utilities.
//!
//! The functions in this module execute a render closure within a temporary
//! [`stylist`] style manager that records all generated CSS. The collected styles
//! can then be embedded into HTML responses produced by frameworks like Axum or
//! Actix, ensuring the initial paint matches the client side.

use stylist::manager::{render_static, StyleManager};

/// Result of server side rendering with collected styles.
///
/// * `html` - Markup returned by the render closure.
/// * `styles` - `<style>` tags that should be injected into the document `<head>`.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SsrOutput {
    pub html: String,
    pub styles: String,
}

/// Renders HTML while capturing all styles produced inside the closure.
///
/// The closure receives a [`StyleManager`] which must be supplied to calls that
/// create styles (e.g. [`stylist::Style::new_with_manager`]). This explicit
/// manager ensures styles remain isolated per request and avoids leaking state
/// between concurrent renders.
pub fn render_with_style<F>(render: F) -> SsrOutput
where
    F: FnOnce(StyleManager) -> String,
{
    // Create a writer/reader pair. The writer is passed to the manager so it can
    // record CSS rules; the reader is used afterwards to turn the rules into
    // style tags.
    let (writer, reader) = render_static();
    let manager = StyleManager::builder()
        .writer(writer)
        .build()
        .expect("create style manager");

    let html = render(manager);

    let mut styles = String::new();
    reader
        .read_style_data()
        .write_static_markup(&mut styles)
        .expect("write styles");

    SsrOutput { html, styles }
}

/// Convenience helper that wraps [`render_with_style`] and returns a complete
/// HTML document containing both the rendered markup and collected style tags.
/// This allows Axum/Actix handlers to simply return the resulting string.
pub fn render_to_string<F>(render: F) -> String
where
    F: FnOnce(StyleManager) -> String,
{
    let out = render_with_style(render);
    format!(
        "<!DOCTYPE html><html><head>{}</head><body>{}</body></html>",
        out.styles, out.html
    )
}
