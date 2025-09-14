//! Helper macros for defining Material UI component props and enums.

/// Generates a `Properties` struct for the target framework with a
/// `Default` implementation. Each field automatically receives the
/// appropriate attribute so callers may omit them when instantiating the
/// component.
#[macro_export]
macro_rules! material_props {
    ($name:ident { $( $(#[$meta:meta])* $field:ident : $ty:ty ),* $(,)? }) => {
        #[cfg(feature = "yew")]
        #[derive(yew::Properties, Clone, PartialEq, Default)]
        pub struct $name {
            $( $(#[$meta])* #[prop_or_default] pub $field: $ty, )*
        }

        #[cfg(feature = "leptos")]
        #[derive(leptos::Props, Clone, PartialEq)]
        pub struct $name {
            $( $(#[$meta])* #[prop(optional, into)] pub $field: $ty, )*
        }

        #[cfg(feature = "leptos")]
        impl Default for $name {
            fn default() -> Self {
                Self {
                    $( $field: Default::default(), )*
                }
            }
        }
    };
}

/// Declares a simple enum and implements `Default` for the first variant.
#[macro_export]
macro_rules! material_enum {
    (
        $name:ident {
            $(#[$meta_first:meta])* $first:ident
            $(,
                $(#[$meta:meta])* $rest:ident
            )*
            $(,)?
        }
    ) => {
        #[derive(Clone, PartialEq)]
        pub enum $name {
            $(#[$meta_first])* $first,
            $( $(#[$meta])* $rest, )*
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
material_enum!(Color {
    /// Use the primary theme color.
    Primary,
    /// Use the secondary theme color.
    Secondary
});
material_enum!(Variant {
    /// Minimal text-only style.
    Text,
    /// Filled background emphasizing the component.
    Contained,
    /// Outlined style with transparent background.
    Outlined
});
material_enum!(Size {
    /// Smallest padding and font size.
    Small,
    /// Default medium size.
    Medium,
    /// Largest spacing for prominent components.
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
