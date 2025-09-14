//! Experimental string-based autocomplete.
//!
//! This tiny component is intentionally minimal. It focuses on the core
//! suggestion logic so that downstream crates can wrap it with any
//! rendering technology they prefer. The module is gate kept behind the
//! `autocomplete` Cargo feature to signal its unstable status and keep
//! compile times low for consumers who do not need it.
//!
//! The implementation favors pure functions and small data structures to
//! keep the API easy to reason about and friendly for automated testing
//! and future code generation.

/// Simple autocomplete that matches the beginning of options.
#[derive(Debug, Clone)]
pub struct Autocomplete {
    /// List of candidate options provided by the application.
    options: Vec<String>,
}

impl Autocomplete {
    /// Creates a new autocomplete with the given options.
    pub fn new(options: Vec<String>) -> Self {
        Self { options }
    }

    /// Returns all options that start with the provided input.
    pub fn suggestions(&self, input: &str) -> Vec<String> {
        self.options
            .iter()
            .filter(|opt| opt.to_lowercase().starts_with(&input.to_lowercase()))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggestions_filter_by_prefix() {
        let ac = Autocomplete::new(vec!["alpha".into(), "beta".into()]);
        assert_eq!(ac.suggestions("a"), vec!["alpha"]);
    }
}
