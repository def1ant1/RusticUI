//! Shared primitives for the select menu examples.
//!
//! The packages under `examples/select-menu-*` intentionally reuse this crate to
//! avoid copy/pasting the same data loading logic, automation identifiers and
//! theme overrides in multiple frameworks. Centralising the helpers keeps the
//! examples focused on framework specific wiring while still demonstrating how
//! enterprises can share core behaviour across SSR and CSR entry points.

use rustic_ui_material::select::{SelectOption, SelectProps};
use rustic_ui_system::theme::{ColorScheme, Theme};

/// Stable automation identifier applied to every DOM node we render.
///
/// Downstream automation suites (Playwright, Cypress, Selenium, etc.) can key
/// off this value to drive the select without relying on CSS class names that
/// may change during refactors.
pub const AUTOMATION_ID: &str = "rusticui-select-menu";

/// Component prefix shared with the Material select implementation.
const COMPONENT_PREFIX: &str = "rustic-select";

/// Domain model representing an option returned from the mock async API.
#[derive(Clone, Debug)]
pub struct Region {
    /// Short code used when persisting the value to backends.
    pub code: &'static str,
    /// Human readable label shown inside the select popover.
    pub name: &'static str,
}

/// Fetch a list of regions with simulated network latency.
///
/// The helper yields to the current runtime (wasm timers in the browser and
/// `tokio` on the server) so examples can exercise async loading spinners and
/// controlled state updates once data arrives.
pub async fn fetch_regions() -> Vec<Region> {
    wait_for_data().await;
    REGIONS.iter().cloned().collect()
}

/// Convert the domain records into `SelectOption`s understood by
/// `rustic_ui_material`.
pub fn to_select_options(regions: &[Region]) -> Vec<SelectOption> {
    regions
        .iter()
        .map(|region| SelectOption::new(region.name, region.code))
        .collect()
}

/// Build select props with a consistent automation identifier.
pub fn props_from_options(label: &str, automation_id: &str, options: &[SelectOption]) -> SelectProps {
    let mut props = SelectProps::new(label, options.to_vec());
    props.automation_id = Some(automation_id.to_string());
    props
}

/// Produce a high contrast enterprise theme used across the demos.
pub fn enterprise_theme() -> Theme {
    let mut theme = Theme::default();
    for scheme in [ColorScheme::Light, ColorScheme::Dark] {
        let palette = theme.palette.scheme_mut(scheme);
        palette.primary = "#003366".into();
        palette.secondary = "#f97316".into();
        palette.background_default = "#0b1120".into();
        palette.background_paper = "#111c3a".into();
        palette.text_primary = "#f8fafc".into();
        palette.text_secondary = "#cbd5f5".into();
    }
    theme.palette.initial_color_scheme = ColorScheme::Dark;
    theme.typography.font_family = "'IBM Plex Sans', 'Segoe UI', sans-serif".into();
    theme.joy.radius = 8;
    theme
}

/// Render Material inspired markup for the select trigger and option list.
///
/// The helper keeps the HTML consistent across SSR and CSR entry points without
/// pulling in the private `ControlStrategy` types from `rustic_ui_headless`.
pub fn render_select_markup(props: &SelectProps, open: bool, selected: Option<usize>) -> String {
    let user_id = props.automation_id.as_deref();
    let base = automation_value(user_id, []);
    let trigger_id = automation_value(user_id, ["trigger"]);
    let list_id = automation_value(user_id, ["list"]);
    let open_flag = open.then_some("true").unwrap_or("false");

    let automation_root = format!(" data-rustic-select-id=\"{base}\"");
    let automation_root_marker = format!(
        " data-rustic-select-root=\"{}\"",
        automation_value(user_id, ["root"])
    );
    let automation_trigger = format!(
        " data-rustic-select-trigger=\"{}\"",
        automation_value(user_id, ["trigger"])
    );
    let automation_list = format!(
        " data-rustic-select-list=\"{}\"",
        automation_value(user_id, ["list"])
    );

    let mut options_markup = String::new();
    for (index, option) in props.options.iter().enumerate() {
        let is_selected = selected == Some(index);
        let selected_flag = is_selected.then_some("true").unwrap_or("false");
        let option_marker = automation_value(user_id, [format!("option-{index}")]);
        let option_id = option_marker.clone();
        let automation_option = format!(" data-rustic-select-option=\"{option_marker}\"");
        options_markup.push_str(&format!(
            "<li id=\"{option_id}\" role=\"option\" aria-selected=\"{selected_flag}\" data-selected=\"{selected_flag}\" data-index=\"{index}\" data-value=\"{}\"{automation_option}>{}</li>",
            option.value,
            option.label
        ));
    }

    format!(
        "<div class=\"rustic_ui_select_root\" data-component=\"rustic-select\" data-open=\"{open_flag}\"{automation_root}{automation_root_marker}><button id=\"{trigger_id}\" role=\"button\" aria-haspopup=\"listbox\" aria-expanded=\"{open_flag}\" aria-controls=\"{list_id}\" data-open=\"{open_flag}\"{automation_trigger}>{}</button><ul id=\"{list_id}\" role=\"listbox\" aria-hidden=\"{}\" data-open=\"{open_flag}\"{automation_list}>{options_markup}</ul></div>",
        props.label,
        (!open).then_some("true").unwrap_or("false")
    )
}

/// Format a human readable summary of the current selection.
pub fn selection_summary(props: &SelectProps, selected: Option<usize>) -> String {
    selected
        .and_then(|index| props.options.get(index))
        .map(|option| format!("Region: {} ({})", option.label, option.value))
        .unwrap_or_else(|| "Select a region to pin traffic".into())
}

/// Wrap raw select markup in a minimal HTML shell for SSR snapshots.
///
/// The wrapper injects basic typography and background colours so the
/// pre-rendered output mirrors the client experience even before hydration.
pub fn ssr_shell(select_markup: &str, theme: &Theme) -> String {
    let palette = theme.palette.active();
    format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"/><title>RusticUI Select Menu</title></head><body style=\"margin:0;background:{};color:{};font-family:{};min-height:100vh;display:flex;align-items:center;justify-content:center;\"><main data-rustic-select-shell=\"{}\" style=\"padding:32px;max-width:720px;\"><h1 style=\"margin-top:0;font-size:1.75rem;\">RusticUI Select Menu</h1>{}</main></body></html>",
        palette.background_default,
        palette.text_primary,
        theme.typography.font_family,
        automation_value(Some(AUTOMATION_ID), ["shell"]),
        select_markup
    )
}

pub fn example_automation_value<I, S>(segments: I) -> String
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    automation_value(Some(AUTOMATION_ID), segments)
}

fn automation_value<I, S>(automation_id: Option<&str>, segments: I) -> String
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut parts = Vec::new();
    parts.push(sanitise(
        automation_id
            .filter(|id| !id.trim().is_empty())
            .unwrap_or(AUTOMATION_ID),
    ));

    for segment in segments {
        let value: String = segment.into();
        let sanitised = sanitise(&value);
        if !sanitised.is_empty() {
            parts.push(sanitised);
        }
    }

    format!("{COMPONENT_PREFIX}-{}", parts.join("-"))
}

fn sanitise(input: &str) -> String {
    let mut output = String::new();
    let mut prev_dash = false;

    for ch in input.chars() {
        let mapped = match ch {
            'A'..='Z' => Some(ch.to_ascii_lowercase()),
            'a'..='z' | '0'..='9' => Some(ch),
            '-' | '_' | ' ' | ':' | '.' | '/' => None,
            _ => None,
        };

        if let Some(valid) = mapped {
            output.push(valid);
            prev_dash = false;
        } else if !prev_dash {
            output.push('-');
            prev_dash = true;
        }
    }

    let trimmed = output.trim_matches('-').to_string();

    if trimmed.is_empty() {
        String::from("select")
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_material::select::{SelectOption, SelectProps};

    #[test]
    fn automation_value_prefixes_component() {
        let value = example_automation_value(["shell", "yew"]);
        assert_eq!(value, "rustic-select-rusticui-select-menu-shell-yew");
    }

    #[test]
    fn render_markup_embeds_prefixed_attributes() {
        let options = vec![SelectOption::new("Sydney", "ap-southeast-2")];
        let mut props = SelectProps::new("Region", options.clone());
        props.automation_id = Some("custom id".into());

        let html = render_select_markup(&props, false, Some(0));
        assert!(html.contains("data-rustic-select-id=\"rustic-select-custom-id\""));
        assert!(html.contains("data-rustic-select-trigger=\"rustic-select-custom-id-trigger\""));
        assert!(html.contains("data-rustic-select-list=\"rustic-select-custom-id-list\""));
        assert!(html.contains("data-rustic-select-option=\"rustic-select-custom-id-option-0\""));
    }
}

#[cfg(feature = "csr")]
async fn wait_for_data() {
    use gloo_timers::future::TimeoutFuture;
    TimeoutFuture::new(120).await;
}

#[cfg(all(feature = "ssr", not(feature = "csr")))]
async fn wait_for_data() {
    use tokio::time::{sleep, Duration};
    sleep(Duration::from_millis(120)).await;
}

#[cfg(not(any(feature = "csr", feature = "ssr")))]
async fn wait_for_data() {}

const REGIONS: [Region; 6] = [
    Region {
        code: "us-east-1",
        name: "US East (N. Virginia)",
    },
    Region {
        code: "us-west-2",
        name: "US West (Oregon)",
    },
    Region {
        code: "eu-central-1",
        name: "EU Central (Frankfurt)",
    },
    Region {
        code: "ap-southeast-2",
        name: "AP Southeast (Sydney)",
    },
    Region {
        code: "sa-east-1",
        name: "South America (São Paulo)",
    },
    Region {
        code: "me-central-1",
        name: "Middle East (UAE)",
    },
];
