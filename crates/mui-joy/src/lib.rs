//! Joy UI component library.
//!
//! This crate mirrors the structure of `mui-material` but implements
//! components and tokens from the Joy design system. The goal is to provide
//! a fully typed Rust API that can scale with additional components.

pub mod accordion;
pub mod aspect_ratio;
pub mod autocomplete;
pub mod button;
pub mod card;
pub mod chip;
pub mod slider;
pub mod snackbar;
pub mod stepper;
pub mod toggle_button_group;
pub mod macros;

pub use accordion::{AccordionController, AccordionGroupState, AccordionItemChange};
pub use aspect_ratio::{AspectRatio, AspectRatioProps};
pub use autocomplete::{
    AutocompleteChange, AutocompleteConfig, AutocompleteController, AutocompleteControlStrategy,
    AutocompleteState,
};
pub use button::{Button, ButtonProps};
pub use card::{Card, CardProps};
pub use chip::{Chip, ChipProps};
pub use macros::{Color, Variant};
pub use slider::{
    SliderChange, SliderConfig, SliderController, SliderOrientation, SliderState,
};
pub use snackbar::{SnackbarChange, SnackbarConfig, SnackbarController, SnackbarMessage, SnackbarState};
pub use stepper::{StepStatus, StepperChange, StepperController, StepperConfig, StepperState};
pub use toggle_button_group::{
    ToggleButtonGroupChange, ToggleButtonGroupConfig, ToggleButtonGroupController,
    ToggleButtonGroupState,
};
