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

// Centralized enums used by many Material components.  Defining them here
// keeps the public API uniform and avoids repeating the common color, variant
// and size options across every widget.  Components are free to re-export
// these or declare their own more specific enums if required.
material_enum!(Color { Primary, Secondary });
material_enum!(Variant {
    Text,
    Contained,
    Outlined
});
material_enum!(Size {
    Small,
    Medium,
    Large
});

/// Convenience macro generating a `Props` struct that already includes the
/// ubiquitous `color`, `variant` and `size` fields.  This drastically reduces
/// boilerplate when adding new components by centralizing prop definitions in
/// a single location.  Custom fields can be appended after the defaults:
///
/// ```ignore
/// material_component_props!(AppBarProps { title: String });
/// ```
///
/// The above expands to a struct `AppBarProps` with fields `title`, `color`,
/// `variant` and `size` – each one optional thanks to `#[prop_or_default]`.
#[macro_export]
macro_rules! material_component_props {
    ($name:ident { $( $(#[$meta:meta])* $field:ident : $ty:ty ),* $(,)? }) => {
        $crate::material_props!($name {
            $( $(#[$meta])* $field: $ty, )*
            /// Visual color scheme applied from the active [`Theme`].
            color: $crate::macros::Color,
            /// Stylistic variant such as `Text` or `Contained`.
            variant: $crate::macros::Variant,
            /// Overall component size.
            size: $crate::macros::Size,
        });
    };
}
