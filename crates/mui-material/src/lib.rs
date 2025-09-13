//! Higher level Material Design components.
//!
//! This crate builds on top of `mui-system` to provide widgets like buttons,
//! dialogs, and layout utilities. The initial version simply re-exports
//! `mui-system` to show the intended layering.

pub use mui_system as system;

/// Confirms that the crate links to `mui-system` and compiles.
pub fn placeholder() {
    // Call into the system crate to prevent dead code warnings
    system::placeholder();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_works() {
        placeholder();
    }
}
