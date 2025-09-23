//! Helper macros for defining Joy UI component props and enums.
//!
//! Historically the props macros were tightly coupled to Yew which meant the
//! generated structs could not be reused by other front-end adapters without
//! copy/pasting definitions. The updated implementation below intentionally
//! emits framework-neutral structs and layers optional integration points for
//! each supported renderer. This keeps enterprise teams from maintaining
//! divergent prop models while still allowing framework specific derive macros
//! (for example `yew::Properties` or `dioxus::Props`) to hook into the shared
//! definition when the corresponding Cargo feature is enabled.

/// Marker trait implemented for every props struct whenever the `leptos`
/// feature is active.
///
/// Leptos primarily relies on function parameters for component props rather
/// than derive macros. The trait provides a lightweight hook that downstream
/// adapters can use to enforce that a props struct is compatible with Leptos
/// specific builders without forcing additional trait bounds on every field.
#[cfg(feature = "leptos")]
pub trait LeptosPropsAdapter: Clone + Default {}

/// Generates a framework-neutral struct capturing Joy component props.
///
/// * Always derives `Clone`, `Default` and `PartialEq` so the props can be
///   cloned across async boundaries and merged with defaults in controller
///   style patterns without locking the implementation into a particular
///   rendering strategy.
/// * Applies optional derives for each supported front-end framework via
///   `#[cfg_attr]`. When the associated Cargo feature is active the struct will
///   automatically implement that framework's props trait without duplicating
///   the field list.
/// * Annotates every field with optional framework specific attributes (for
///   example Yew's `#[prop_or_default]`) using `cfg_attr` so the metadata is
///   only emitted when the framework is compiled in.
#[macro_export]
macro_rules! joy_props {
    ($name:ident { $( $(#[$meta:meta])* $field:ident : $ty:ty ),* $(,)? }) => {
        #[derive(Clone, Default, PartialEq)]
        #[cfg_attr(feature = "yew", derive(yew::Properties))]
        #[cfg_attr(feature = "dioxus", derive(dioxus::Props))]
        #[cfg_attr(feature = "sycamore", derive(sycamore::Props))]
        pub struct $name {
            $(
                $(#[$meta])*
                #[cfg_attr(feature = "yew", prop_or_default)]
                pub $field: $ty,
            )*
        }

        #[cfg(feature = "leptos")]
        impl $crate::macros::LeptosPropsAdapter for $name {}
    };
}

/// Declares a simple enum and implements `Default` for the first variant.
///
/// The macro accepts optional attributes (including documentation comments) per
/// variant so call sites can describe the semantics of each option without
/// having to hand-roll the enum definition.  This keeps the `Color` palette and
/// other Joy enums heavily documented, aligning with the repository goal of
/// enabling enterprise teams to self-serve details directly from the code.
#[macro_export]
macro_rules! joy_enum {
    (
        $name:ident {
            $(#[$first_meta:meta])* $first:ident
            $(,
                $(#[$rest_meta:meta])* $rest:ident
            )*
            $(,)?
        }
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $(#[$first_meta])* $first,
            $(
                $(#[$rest_meta])* $rest,
            )*
        }
        impl Default for $name {
            fn default() -> Self {
                Self::$first
            }
        }
    };
}

// Reusable enums shared across Joy components. Keeping them here ensures
// a single source of truth so every component follows the same patterns.
joy_enum!(Color {
    /// Primary brand tint used for high-emphasis actions and the default Joy accent.
    Primary,
    /// Neutral tone that keeps surfaces understated while still harmonising with the theme.
    Neutral,
    /// Danger color reserved for destructive flows and critical alerts.
    Danger,
    /// Success feedback color reinforcing positive outcomes across dashboards and forms.
    Success,
    /// Warning hue signalling cautionary states that require user attention without implying failure.
    Warning,
    /// Informational accent balancing the palette for notification banners and secondary emphasis.
    Info,
});

impl Color {
    /// Stable list of every Joy palette color.  Framework adapters iterate this
    /// constant to generate colour pickers, documentation tables, or exhaustive
    /// tests without re-stating the palette in multiple locations.
    pub const ALL: [Self; 6] = [
        Self::Primary,
        Self::Neutral,
        Self::Danger,
        Self::Success,
        Self::Warning,
        Self::Info,
    ];

    /// Lowercase identifier mirroring the class name suffixes used by the
    /// upstream MUI libraries.  Keeping the mapping centralised guarantees that
    /// future adapters (Leptos, Dioxus, Sycamore) can emit consistent `data-*`
    /// hooks or CSS module keys without bespoke lookup tables.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Neutral => "neutral",
            Self::Danger => "danger",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

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
        $crate::joy_props!($name {
            /// Visual color scheme of the component.
            ///
            /// Supported palette entries: [`Color::Primary`], [`Color::Neutral`],
            /// [`Color::Danger`], [`Color::Success`], [`Color::Warning`], and
            /// [`Color::Info`].  The enum derives [`Copy`] so adapters can store
            /// the selected value in signals or contexts without additional
            /// allocations.
            color: Color,
            /// Variant controlling background and border styles.
            variant: Variant,
            $( $(#[$meta])* $field : $ty, )*
        });
    };
}
