# Selection Controls with Leptos

The headless selection control states compose cleanly with Leptos. Generate
markup using the `leptos` adapters and mount it with `leptos::html::Div` or the
`leptos::view!` macro.

```rust
use leptos::*;
use rustic_ui_headless::checkbox::CheckboxState;
use rustic_ui_headless::switch::SwitchState;
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_material::checkbox::{self, CheckboxProps};
use rustic_ui_material::switch::{self, SwitchProps};
use rustic_ui_material::radio::{self, RadioGroupProps};

#[component]
pub fn SelectionControls() -> impl IntoView {
    let checkbox_state = CheckboxState::uncontrolled(false, true);
    let switch_state = SwitchState::uncontrolled(false, false);
    let radio_state = RadioGroupState::uncontrolled(
        vec!["Visa".into(), "Mastercard".into(), "Amex".into()],
        false,
        RadioOrientation::Vertical,
        Some(1),
    );

    view! {
        <div inner_html=checkbox::leptos::render(&CheckboxProps::new("Save card"), &checkbox_state)/>
        <div inner_html=switch::leptos::render(&SwitchProps::new("Enable auto-pay"), &switch_state)/>
        <div inner_html=radio::leptos::render(&RadioGroupProps::from_state(&radio_state), &radio_state)/>
    }
}
```
