//! Joy UI component library.
//!
//! The crate exposes two complementary building blocks:
//!
//! * **Framework-neutral prop definitions** – [`joy_props!`](crate::joy_props) and
//!   [`joy_component_props!`](crate::joy_component_props) emit plain Rust
//!   structs that capture component configuration without binding to a specific
//!   renderer. Optional `#[cfg_attr]` hooks layer in `yew::Properties`,
//!   `dioxus::Props` and `sycamore::Props` derives when the corresponding Cargo
//!   feature is enabled. Leptos consumers can lean on the
//!   `LeptosPropsAdapter` (available when the `leptos` feature is enabled) to
//!   assert compatibility inside custom wrappers.
//! * **Yew-first component implementations** – the existing modules under
//!   `src/` currently target Yew and are gated behind the `yew` feature. This
//!   keeps the crate compiling for Leptos, Dioxus and Sycamore consumers that
//!   only need the shared prop structs while the remaining adapters are built
//!   out incrementally.
//!
//! ### Enabling framework integrations
//!
//! | Feature      | Purpose                                                                 |
//! |--------------|-------------------------------------------------------------------------|
//! | `yew`        | Activates the concrete Yew components and applies `yew::Properties`     |
//! | `leptos`     | Enables the Leptos marker trait so downstream crates can add adapters   |
//! | `dioxus`     | Derives `dioxus::Props` on every generated prop struct                  |
//! | `sycamore`   | Derives `sycamore::Props` on every generated prop struct                |
//!
//! All features activate the corresponding `rustic_ui_system` adapter to guarantee that themed
//! primitives behave consistently across frameworks. This design avoids manual repetitive
//! glue code and ensures future adapters reuse the exact same prop contracts.

#[cfg(feature = "yew")]
pub mod accordion;
#[cfg(feature = "yew")]
pub mod aspect_ratio;
#[cfg(feature = "yew")]
pub mod autocomplete;
#[cfg(feature = "yew")]
pub mod button;
#[cfg(feature = "yew")]
pub mod card;
#[cfg(feature = "yew")]
pub mod chip;
pub mod helpers;
pub mod macros;
#[cfg(feature = "yew")]
pub mod slider;
#[cfg(feature = "yew")]
pub mod snackbar;
#[cfg(feature = "yew")]
pub mod stepper;
#[cfg(feature = "yew")]
pub mod toggle_button_group;

#[cfg(feature = "yew")]
pub use accordion::{AccordionController, AccordionGroupState, AccordionItemChange};
#[cfg(feature = "yew")]
pub use aspect_ratio::{AspectRatio, AspectRatioProps};
#[cfg(feature = "yew")]
pub use autocomplete::{
    AutocompleteChange, AutocompleteConfig, AutocompleteControlStrategy, AutocompleteController,
    AutocompleteState,
};
#[cfg(feature = "yew")]
pub use button::{Button, ButtonProps};
#[cfg(feature = "yew")]
pub use card::{Card, CardProps};
#[cfg(feature = "yew")]
pub use chip::{Chip, ChipProps};
pub use macros::{Color, Variant};
#[cfg(feature = "yew")]
pub use slider::{SliderChange, SliderConfig, SliderController, SliderOrientation, SliderState};
#[cfg(feature = "yew")]
pub use snackbar::{
    SnackbarChange, SnackbarConfig, SnackbarController, SnackbarMessage, SnackbarState,
};
#[cfg(feature = "yew")]
pub use stepper::{StepStatus, StepperChange, StepperConfig, StepperController, StepperState};
#[cfg(feature = "yew")]
pub use toggle_button_group::{
    ToggleButtonGroupChange, ToggleButtonGroupConfig, ToggleButtonGroupController,
    ToggleButtonGroupState,
};

#[cfg(feature = "compat-mui")]
#[doc = "Deprecated compatibility shim exposing the crate under the legacy `mui_joy` name.\n\
Keep the `compat-mui` feature enabled only while migrating to `rustic_ui_joy`.\n\
The alias will be retired prior to the 1.0 release."]
#[deprecated(
    since = "0.1.0",
    note = "Update imports to `rustic_ui_joy`. The `mui_joy` alias is temporary and will be removed."
)]
pub use crate as mui_joy;
