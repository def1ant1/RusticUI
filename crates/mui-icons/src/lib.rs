//! Material UI icon bindings.
//!
//! Future versions of this crate will expose a procedural macro that turns
//! the upstream SVG icon set into type-safe Rust structures for use with
//! front-end frameworks. For now it's an empty crate with a compilation
//! test to keep the project wiring intact.

/// Placeholder function to prove that the crate compiles.
pub fn placeholder() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_works() {
        placeholder();
    }
}
