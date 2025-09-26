//! Runtime locale management inspired by MUI's `LocalizationProvider`.
//!
//! **Unstable:** Localization APIs may change as more real world
//! requirements surface.  This module is gated behind the `localization`
//! feature flag to highlight its experimental nature.
//!
//! Locales are registered at runtime so that large applications can share
//! translations without forcing every binary to rebuild when new languages
//! are added.  Locale packs are regular Rust structs that implement the
//! [`LocalePack`] trait and can optionally be serialized with `serde` for
//! distribution via configuration files or network requests.

use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::adapters::DateAdapter;

/// Trait representing a bundle of localized strings and formatting rules.
///
/// Keeping the interface tiny reduces boilerplate for community
/// translations and makes it easy to adapt the API as requirements evolve.
pub trait LocalePack: Send + Sync {
    /// Returns the BCP-47 locale code (e.g. `en-US`).
    fn code(&self) -> &'static str;

    /// Formats a preformatted ISO date string.  The adapter is responsible
    /// for generating the base representation.
    fn format_date(&self, iso: &str) -> String;
}

/// Global registry of locale packs.  Community crates can register their
/// translations at runtime.  `RwLock` is used so reads (the common case)
/// don't block each other.
static LOCALES: Lazy<RwLock<HashMap<String, Box<dyn LocalePack>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Registers a locale pack.  Existing registrations are overwritten so
/// version bumps can replace outdated translations automatically.
pub fn register_locale<L: LocalePack + 'static>(locale: L) {
    LOCALES
        .write()
        .expect("locale registry poisoned")
        .insert(locale.code().to_string(), Box::new(locale));
}

/// Provider used by widgets to access locale data.  A provider is typically
/// created once per request and cloned cheaply throughout the widget tree.
pub struct LocalizationProvider {
    locale: String,
}

impl LocalizationProvider {
    /// Creates a provider for the given locale if it was registered.
    pub fn new(locale: &str) -> Option<Self> {
        if LOCALES
            .read()
            .expect("locale registry poisoned")
            .contains_key(locale)
        {
            Some(Self {
                locale: locale.to_string(),
            })
        } else {
            None
        }
    }

    /// Formats a date using the locale pack and adapter.
    pub fn format_date<A: DateAdapter>(&self, date: &A::Date, adapter: &A) -> String {
        let iso = adapter.format(date);
        LOCALES
            .read()
            .expect("locale registry poisoned")
            .get(&self.locale)
            .expect("locale not registered")
            .format_date(&iso)
    }
}

pub mod en_us;
pub use en_us::EnUs;

/// Initializes the registry with the default English locale.  Tests call
/// this to ensure a baseline environment but production applications may
/// choose to register their own locales instead.
pub fn init_default_locales() {
    register_locale(EnUs::default());
}
