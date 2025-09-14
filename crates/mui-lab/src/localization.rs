//! Runtime locale management inspired by MUI's `LocalizationProvider`.
//! Consumers can register locale packs at startup and widgets will pull
//! the appropriate translations dynamically.  This keeps translation
//! responsibilities decoupled from rendering logic and scales to large
//! applications maintained by many teams.

use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::adapters::DateAdapter;

/// Trait representing a bundle of localized strings and formatting rules.
/// The trait is object safe so locale packs can be stored behind `dyn`.
pub trait LocalePack: Send + Sync {
    /// Returns the BCP-47 locale code (e.g. `en-US`).
    fn code(&self) -> &'static str;

    /// Formats a preformatted ISO date string.  The adapter is responsible
    /// for generating the base representation.
    fn format_date(&self, iso: &str) -> String;
}

/// Global registry of locale packs.  Community crates can register their
/// translations at runtime.
static LOCALES: Lazy<RwLock<HashMap<String, Box<dyn LocalePack>>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

/// Registers a locale pack.  Existing registrations are overwritten so
/// version bumps can replace outdated translations automatically.
pub fn register_locale<L: LocalePack + 'static>(locale: L) {
    LOCALES
        .write()
        .expect("locale registry poisoned")
        .insert(locale.code().to_string(), Box::new(locale));
}

/// Provider used by widgets to access locale data.
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

/// Basic English locale pack used as a fallback.
pub struct EnUs;

impl LocalePack for EnUs {
    fn code(&self) -> &'static str {
        "en-US"
    }

    fn format_date(&self, iso: &str) -> String {
        iso.to_string()
    }
}

/// Initializes the registry with the default English locale.  Tests call
/// this to ensure a baseline environment.
pub fn init_default_locales() {
    register_locale(EnUs);
}

