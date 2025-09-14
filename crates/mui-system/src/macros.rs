//! Declarative helpers for generating CSS style strings.
//!
//! A large portion of component boilerplate in styling libraries is simply
//! mapping Rust struct fields to CSS strings.  The macros in this module keep
//! that mapping centralized and maintainable so new style props can be added
//! with a single line of code rather than repetitive manual functions.  Each
//! macro includes usage examples to encourage copy‑pasting directly into new
//! components.

/// Generates a style string from `key: value` pairs.
///
/// ```rust
/// use mui_system::style_props;
/// let style = style_props! { width: "100%", margin_top: "8px" };
/// assert_eq!(style, "width:100%;margin-top:8px;");
/// ```
#[macro_export]
macro_rules! style_props {
    ( $( $name:ident : $value:expr ),* $(,)? ) => {{
        let mut s = String::new();
        $(
            // Convert snake_case identifiers to kebab-case CSS property names.
            let prop = stringify!($name).replace('_', "-");
            s.push_str(&prop);
            s.push(':');
            s.push_str(&$value);
            s.push(';');
        )*
        s
    }};
}

/// Declares a helper function for a single CSS property.
///
/// ```rust
/// use mui_system::define_style_prop;
/// define_style_prop!(margin_top, "margin-top");
/// let style = margin_top("8px");
/// assert_eq!(style, "margin-top:8px;");
/// ```
#[macro_export]
macro_rules! define_style_prop {
    ($func:ident, $prop:expr) => {
        /// Macro generated style helper.
        pub fn $func<V: Into<String>>(value: V) -> String {
            format!("{}:{};", $prop, value.into())
        }
    };
}

/// Batch declare multiple style helper functions at once.
///
/// This macro is ideal when a component exposes many style props.  It keeps
/// the source focused on *what* props exist rather than *how* the individual
/// strings are assembled.
///
/// ```rust
/// use mui_system::define_style_props;
/// define_style_props! {
///     margin_top => "margin-top",
///     margin_bottom => "margin-bottom",
/// }
/// assert_eq!(margin_top("8px"), "margin-top:8px;");
/// assert_eq!(margin_bottom("4px"), "margin-bottom:4px;");
/// ```
#[macro_export]
macro_rules! define_style_props {
    ( $( $func:ident => $prop:expr ),* $(,)? ) => {
        $(
            pub fn $func<V: Into<String>>(value: V) -> String {
                format!("{}:{};", $prop, value.into())
            }
        )*
    };
}

/// Quickly generate a style string for a single property.
///
/// ```rust
/// use mui_system::style_prop;
/// let style = style_prop!(width = "100px");
/// assert_eq!(style, "width:100px;");
/// ```
#[macro_export]
macro_rules! style_prop {
    ($name:ident = $value:expr) => {{
        let prop = stringify!($name).replace('_', "-");
        format!("{}:{};", prop, $value)
    }};
}
