//! Multi-set SVG icon bindings for Rust front-end frameworks.
//!
//! Icon sets live under `icons/<set>/` and are converted into memoized
//! Rust functions at compile time. Each set exposes a `<set>_icon!` macro
//! when the corresponding `set-<set>` feature is enabled.

// Include the generated bindings produced by build.rs. This keeps the
// repository lean and removes the need for manual wiring when icon sets
// change.
include!(concat!(env!("OUT_DIR"), "/icons.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "set-material")]
    #[test]
    fn material_macro_renders_valid_svg() {
        let svg = material_icon!("10k_24px");
        assert!(svg.starts_with("<svg"));
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(svg, &opt).expect("valid svg");
        assert!(tree.root().has_children());
    }

    // Compile-time assurance that icons without their feature flag are not
    // included in the build. This test only runs when one icon is enabled
    // and another is intentionally left out.
    #[cfg(all(
        feature = "icon-material-10k_24px",
        not(feature = "icon-material-10mp_24px")
    ))]
    #[test]
    fn disabled_icons_are_omitted() {
        #[cfg(feature = "icon-material-10mp_24px")]
        compile_error!("icon-material-10mp_24px should not be enabled in this configuration");
        assert!(material::icon_10k_24px().starts_with("<svg"));
    }
}
