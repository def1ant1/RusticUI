//! Minimal SSR harness demonstrating the headless + Material rating primitives.
//!
//! The binary prints a deterministic HTML fragment that downstream frameworks can
//! reuse as their hydration baseline.  Integration scripts can pipe the output to
//! Playwright snapshots or static site generators without duplicating renderer
//! logic in multiple languages.

use rustic_ui_headless::rating::{RatingConfig, RatingState};
use rustic_ui_material::rating::{render_rating_html, RatingAdapterProps, RatingControlMode};

fn main() {
    let mut config = RatingConfig::enterprise_defaults();
    config.analytics_channel = Some("examples.feedback.rating".into());
    let mut state = RatingState::uncontrolled(config);
    state.select(3.5);
    let html = render_rating_html(RatingAdapterProps {
        state: &state,
        id: Some("feedback-rating"),
        label: Some("Rate this release"),
        automation_id: Some("demo"),
        control: RatingControlMode::Uncontrolled,
    });
    println!("{html}");
}
