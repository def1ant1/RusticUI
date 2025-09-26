//! Shared style registry used by Yew providers and SSR helpers.
//!
//! The registry owns both a [`Theme`] and an isolated [`StyleManager`]. The
//! manager is backed by a writer/reader pair so that CSS rules generated during
//! rendering can be flushed into `<style>` blocks.  Cloning the registry
//! produces a shallow copy that shares the underlying manager allowing
//! independent components to contribute styles while still aggregating them in a
//! single place.
//!
//! The internal reader is protected by a [`Mutex`] to ensure thread-safety when
//! style flushing happens concurrently (e.g. across async boundaries during
//! server side rendering).  Each request should instantiate its own registry to
//! avoid cross-request style leakage.  Once [`flush_styles`](StyleRegistry::flush_styles)
//! is called, the accumulated style data is drained, making the type safe to
//! reuse for subsequent renders.

use std::sync::{Arc, Mutex};

use stylist::manager::{render_static, StaticReader, StyleManager};

use crate::Theme;

/// Central style registry shared via Yew context.
#[derive(Clone)]
pub struct StyleRegistry {
    /// Theme associated with this registry.  The theme is cloned for each
    /// component render to prevent mutation across threads.
    theme: Theme,
    /// Manager that records all style rules.  [`StyleManager`] is internally
    /// reference counted so cloning is cheap and thread-safe.
    manager: StyleManager,
    /// Reader side of the style channel.  Wrapped in a [`Mutex`] so multiple
    /// threads can request style flushing concurrently without data races.
    reader: Arc<Mutex<Option<StaticReader>>>, // when flushed styles are drained
}

impl PartialEq for StyleRegistry {
    fn eq(&self, other: &Self) -> bool {
        self.theme == other.theme
    }
}

impl StyleRegistry {
    /// Constructs a new registry with an isolated [`StyleManager`].
    pub fn new(theme: Theme) -> Self {
        let (writer, reader) = render_static();
        let manager = StyleManager::builder()
            .writer(writer)
            .build()
            .expect("create style manager");
        Self {
            theme,
            manager,
            reader: Arc::new(Mutex::new(Some(reader))),
        }
    }

    /// Returns the [`Theme`] tied to this registry.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Provides a [`StyleManager`] clone for creating styles.
    pub fn style_manager(&self) -> StyleManager {
        self.manager.clone()
    }

    /// Drains all collected styles and returns them as `<style>` blocks suitable
    /// for embedding into server rendered documents.
    pub fn flush_styles(&self) -> String {
        let mut out = String::new();
        if let Ok(mut reader) = self.reader.lock() {
            if let Some(r) = reader.take() {
                r.read_style_data()
                    .write_static_markup(&mut out)
                    .expect("write styles");
            }
        }
        out
    }
}
