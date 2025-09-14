use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Root directory containing icon sets. Each subdirectory represents a set
    // such as `material`, `material-outlined`, etc.
    let icons_root = Path::new("icons");
    if !icons_root.exists() {
        // Nothing to do if the repo was cloned without icons.
        return Ok(());
    }

    // Where the generated Rust source and processed SVGs will be written.
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    fs::create_dir_all(&out_dir)?;

    let mut modules: Vec<TokenStream> = Vec::new();
    let mut macros: Vec<TokenStream> = Vec::new();

    for set in fs::read_dir(icons_root)? {
        let set = set?;
        if !set.file_type()?.is_dir() {
            continue;
        }
        let set_name = set.file_name().to_string_lossy().into_owned();
        let set_dir = set.path();
        let module_ident = format_ident!("{}", set_name.replace('-', "_"));
        let set_feature = format!("set-{}", set_name);
        let set_feature_lit = Literal::string(&set_feature);

        fs::create_dir_all(out_dir.join(&set_name))?;

        let mut functions: Vec<TokenStream> = Vec::new();
        let mut macro_arms: Vec<TokenStream> = Vec::new();

        for icon in fs::read_dir(&set_dir)? {
            let icon = icon?;
            let path = icon.path();
            if path.extension().and_then(|s| s.to_str()) != Some("svg") {
                continue;
            }
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            let name_no_ext = file_name.trim_end_matches(".svg");
            let ident = format_ident!("icon_{}", name_no_ext.replace('-', "_"));
            let feature_name = format!("icon-{}-{}", set_name, name_no_ext);
            let feature_lit = Literal::string(&feature_name);

            // Validate and minify SVG for deterministic builds.
            let raw_svg = fs::read_to_string(&path)?;
            let opt = usvg::Options::default();
            let tree = usvg::Tree::from_str(&raw_svg, &opt)?;
            let minified = tree.to_string(&usvg::WriteOptions::default());
            fs::write(out_dir.join(&set_name).join(&file_name), &minified)?;

            let file_lit = Literal::string(&format!("{}/{}", set_name, file_name));
            let icon_name_lit = Literal::string(name_no_ext);

            // Memoized function for this icon.
            functions.push(quote! {
                #[cfg(feature = #feature_lit)]
                #[doc = concat!("Returns the `", #icon_name_lit, "` icon from the `", #set_name, "` set as an SVG string.")]
                pub fn #ident() -> &'static str {
                    static SVG: once_cell::sync::Lazy<&'static str> =
                        once_cell::sync::Lazy::new(|| include_str!(concat!(env!("OUT_DIR"), "/", #file_lit)));
                    *SVG
                }
            });

            // Macro arm mapping string name to the function.
            macro_arms.push(quote! {
                (#icon_name_lit) => {{
                    #[cfg(feature = #feature_lit)]
                    { $crate::#module_ident::#ident() }
                    #[cfg(not(feature = #feature_lit))]
                    { compile_error!(concat!("feature `", #feature_lit, "` is not enabled")); }
                }};
            });
        }

        modules.push(quote! {
            /// Auto-generated bindings for the `#set_name` icon set.
            pub mod #module_ident {
                #(#functions)*
            }
        });

        let macro_ident = format_ident!("{}_icon", set_name.replace('-', "_"));
        macros.push(quote! {
            #[cfg(feature = #set_feature_lit)]
            #[macro_export]
            macro_rules! #macro_ident {
                #(#macro_arms)*
            }
        });

        println!("cargo:rerun-if-changed=icons/{}", set_name);
    }

    let output = quote! {
        #(#modules)*

        #(#macros)*
    };

    fs::write(out_dir.join("icons.rs"), output.to_string())?;
    println!("cargo:rerun-if-changed=icons");
    Ok(())
}
