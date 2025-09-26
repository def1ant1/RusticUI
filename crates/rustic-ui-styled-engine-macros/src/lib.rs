use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, ItemFn, Type};

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
        let ty = &f.ty;
        if is_option(ty) {
            quote! {
                #ident: self.#ident
                    .map(::core::convert::Into::into)
                    .unwrap_or_else(|| ::mui_styled_engine::Theme::default().#ident)
            }
        } else {
            quote! { #ident: ::core::convert::Into::into(self.#ident) }
        }
    });

    let expanded = quote! {
        impl #name {
            /// Converts the custom struct into the engine's [`Theme`],
            /// filling unspecified fields from `Theme::default`.
            pub fn into_theme(self) -> ::mui_styled_engine::Theme {
                ::mui_styled_engine::Theme { #( #assignments, )* ..::mui_styled_engine::Theme::default() }
            }
        }

        impl ::core::convert::From<#name> for ::mui_styled_engine::Theme {
            fn from(value: #name) -> Self {
                value.into_theme()
            }
        }
    };

    expanded.into()
}

fn is_option(ty: &Type) -> bool {
    matches!(ty, Type::Path(tp) if tp.path.segments.first().map(|s| s.ident == "Option").unwrap_or(false))
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

/// Function-like macro that wraps [`stylist::css!`] while automatically injecting
/// a `use_theme` call. The macro works across supported front-end frameworks by
/// delegating theme retrieval to `mui_styled_engine::use_theme`.
///
/// ```ignore
/// let style = css_with_theme!(r#"color: ${theme.palette.primary};"#);
/// assert!(style.get_class_name().starts_with("css-"));
/// ```
#[proc_macro]
pub fn css_with_theme(input: TokenStream) -> TokenStream {
    let tokens = proc_macro2::TokenStream::from(input);
    let expanded = quote! {{
        let theme = ::mui_styled_engine::use_theme();
        ::mui_styled_engine::Style::new(::mui_styled_engine::css!(#tokens))
            .expect("valid css")
    }};
    TokenStream::from(expanded)
}
