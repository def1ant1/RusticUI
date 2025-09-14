//! Built-in US English locale pack.
//!
//! **Unstable:** The structure of locale packs may change before a stable
//! release.  The pack is kept intentionally small to encourage community
//! contributions for real world translations.

use serde::{Deserialize, Serialize};

use super::LocalePack;

/// Minimal locale pack demonstrating how translations can be modeled as a
/// serializable data structure.  Real applications are expected to embed
/// many more fields covering all UI strings.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnUs {
    /// Date format string used by the `time` crate's formatting routines.
    pub date_format: &'static str,
}

impl Default for EnUs {
    fn default() -> Self {
        Self {
            date_format: "%Y-%m-%d",
        }
    }
}

impl LocalePack for EnUs {
    fn code(&self) -> &'static str {
        "en-US"
    }

    fn format_date(&self, iso: &str) -> String {
        // For the default locale we simply forward the ISO string.  More
        // advanced packs could parse the string and reformat according to the
        // `date_format` field.
        iso.to_string()
    }
}
