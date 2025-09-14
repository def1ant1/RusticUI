//! Material Icons for Rust front-end frameworks.
//!
//! This crate is intentionally lean – at compile time a build script parses the
//! SVGs located in the accompanying `material-icons/` directory and converts them
//! into memoized Rust functions. Each SVG is exposed both as a function and via
//! the [`material_icon!`] macro for ergonomic access:
//!
//! ```no_run
//! let svg = mui_icons_material::material_icon!("10k_24px");
//! println!("{}", svg);
//! ```
//!
//! ### Custom icon sets
//! Additional SVGs can be placed into `material-icons/`. The build script will
//! detect them automatically and regenerate the Rust bindings. This keeps manual
//! wiring to a minimum and encourages centralized automation.

// Include the generated icon functions and the `material_icon!` macro.
include!(concat!(env!("OUT_DIR"), "/icons.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_renders_valid_svg() {
        // Use the generated macro to fetch an SVG and ensure it parses.
        let svg = material_icon!("10k_24px");
        assert!(svg.starts_with("<svg"));
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(svg, &opt).expect("valid svg");
        assert!(tree.root().has_children());
    }
}
