use rustic_ui_system::grid_span_to_percent;

/// Ensure the grid math matches expected percentages.
#[test]
fn grid_span_calculations() {
    assert!((grid_span_to_percent(1, 12) - 8.333).abs() < 0.01);
    assert!((grid_span_to_percent(6, 12) - 50.0).abs() < f32::EPSILON);
}
