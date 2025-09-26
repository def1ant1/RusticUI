// build.rs - converts SVG files into memoized Rust functions and macros.
// The script scans the `material-icons/` directory and writes a generated
// `icons.rs` file into Cargo's OUT_DIR. This keeps the repo lean while
// guaranteeing repeatable builds. Contributors can drop new SVGs into the
// directory and functions will be produced automatically.

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Directory containing the raw SVG icon files.
    let icons_dir = Path::new("material-icons");
    // Where the generated Rust code and processed SVGs will live.
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    fs::create_dir_all(&out_dir)?;

    let mut functions: Vec<TokenStream> = Vec::new();
    let mut macro_arms: Vec<TokenStream> = Vec::new();

    for entry in fs::read_dir(icons_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("svg") {
            continue;
        }
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let name_no_ext = file_name.trim_end_matches(".svg");

        // Sanitize the identifier: replace dashes with underscores and prefix with 'icon_'.
        let ident_str = name_no_ext.replace('-', "_");
        let ident = format_ident!("icon_{}", ident_str);
        // Each icon gets its own Cargo feature so downstream crates can select
        // only the symbols they need. The CLI keeps this list up to date.
        let feature_name = format!("icon-{}", name_no_ext);
        let feature_lit = Literal::string(&feature_name);

        // Parse and minify the SVG using usvg to ensure valid syntax.
        let raw_svg = fs::read_to_string(&path)?;
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(&raw_svg, &opt)?;
        let wopt = usvg::WriteOptions::default();
        let minified = tree.to_string(&wopt);
        fs::write(out_dir.join(&file_name), minified)?;

        let file_lit = Literal::string(&file_name);
        let name_lit = Literal::string(name_no_ext);

        // Generate a function returning a memoized &'static str of the SVG data.
        functions.push(quote! {
            #[cfg(feature = #feature_lit)]
            #[doc = concat!("Returns the `", #name_lit, "` icon as an SVG string.")]
            pub fn #ident() -> &'static str {
                static SVG: once_cell::sync::Lazy<&'static str> =
                    once_cell::sync::Lazy::new(|| include_str!(concat!(env!("OUT_DIR"), "/", #file_lit)));
                *SVG
            }
        });

        // Add a macro arm mapping the original file name to the function.
        macro_arms.push(quote! {
            (#name_lit) => {{
                #[cfg(feature = #feature_lit)]
                { $crate::#ident() }
                #[cfg(not(feature = #feature_lit))]
                { compile_error!(concat!("feature `", #feature_lit, "` is not enabled")); }
            }};
        });
    }

    let output = quote! {
        #(#functions)*

        #[macro_export]
        macro_rules! material_icon {
            #(#macro_arms)*
        }
    };

    fs::write(out_dir.join("icons.rs"), output.to_string())?;
    // Re-run build script if icons change.
    println!("cargo:rerun-if-changed=material-icons");
    Ok(())
}
