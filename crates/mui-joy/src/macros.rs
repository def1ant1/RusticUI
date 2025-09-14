//! Helper macros for defining Joy UI component props and enums.

/// Generates a `yew::Properties` struct with `Default` implementation and
/// automatic `#[prop_or_default]` markers for ergonomics.
#[macro_export]
macro_rules! joy_props {
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
macro_rules! joy_enum {
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

// Reusable enums shared across Joy components. Keeping them here ensures
// a single source of truth so every component follows the same patterns.
joy_enum!(Color {
    Primary,
    Neutral,
    Danger
});

joy_enum!(Variant {
    Solid,
    Soft,
    Outlined,
    Plain
});

/// Helper macro building on [`joy_props!`] that pre-defines common `color`
/// and `variant` fields shared by most Joy components. Additional fields can
/// be supplied after these defaults.
#[macro_export]
macro_rules! joy_component_props {
    ($name:ident { $( $(#[$meta:meta])* $field:ident : $ty:ty ),* $(,)? }) => {
        crate::joy_props!($name {
            /// Visual color scheme of the component.
            color: Color,
            /// Variant controlling background and border styles.
            variant: Variant,
            $( $(#[$meta])* $field : $ty, )*
        });
    };
}
