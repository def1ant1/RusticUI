use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn, Fields};

/// Derive macro generating an `into_theme` helper for custom theme structs.
///
/// The macro merges the user provided fields with [`mui_styled_engine::Theme`]'s
/// defaults. This enables ergonomic creation of theme overrides without
/// requiring manual plumbing for each field.
#[proc_macro_derive(Theme)]
pub fn derive_theme(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Only support structs with named fields for now
    let fields = match input.data {
        syn::Data::Struct(s) => match s.fields {
            Fields::Named(f) => f.named.into_iter().collect::<Vec<_>>(),
            _ => panic!("Theme derive only supports named structs"),
        },
        _ => panic!("Theme derive only supports structs"),
    };

    let assignments = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        quote! { #ident: self.#ident }
    });

    let expanded = quote! {
        impl #name {
            /// Converts the custom struct into the engine's [`Theme`],
            /// filling unspecified fields from `Theme::default`.
            pub fn into_theme(self) -> ::mui_styled_engine::Theme {
                ::mui_styled_engine::Theme { #( #assignments, )* ..::mui_styled_engine::Theme::default() }
            }
        }
    };

    expanded.into()
}

/// Function-like macro that converts a regular function into a Yew component
/// and automatically wires up theme retrieval. The macro expects a standard
/// function declaration and injects a `use_theme` call at the beginning.
///
/// ```ignore
/// styled_component! {
///     fn MyButton() -> yew::Html {
///         html! { <button>{"hi"}</button> }
///     }
/// }
/// ```
#[proc_macro]
pub fn styled_component(input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let vis = &func.vis;
    let sig = func.sig.clone();
    let name = sig.ident.clone();
    let block = func.block;

    let expanded = quote! {
        #[::yew::function_component(#name)]
        #vis #sig {
            let theme = ::mui_styled_engine::use_theme();
            #block
        }
    };

    expanded.into()
}
