///! Utility macros for generating CSS style strings.
///
/// The macros favor code generation over manual string concatenation which
/// keeps component implementations terse and less error prone.

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
