//! Helper macros for defining Material UI component props and enums.

/// Generates a `yew::Properties` struct with `Default` implementation.
/// Each field automatically receives `#[prop_or_default]` so callers can
/// omit them.
#[macro_export]
macro_rules! material_props {
    ($name:ident { $( $(#[$meta:meta])* $field:ident : $ty:ty ),* $(,)? }) => {
        #[cfg(feature = "yew")]
        #[derive(yew::Properties, Clone, PartialEq, Default)]
        pub struct $name {
            $( $(#[$meta])* #[prop_or_default] pub $field: $ty, )*
        }
    };
}

/// Declares a simple enum and implements `Default` for the first variant.
#[macro_export]
macro_rules! material_enum {
    ($name:ident { $first:ident $(, $rest:ident)* $(,)? }) => {
        #[derive(Clone, PartialEq)]
        pub enum $name {
            $first,
            $( $rest, )*
        }
        impl Default for $name {
            fn default() -> Self { Self::$first }
        }
    };
}
