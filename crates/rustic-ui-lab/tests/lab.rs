use mui_lab::adapters::{AdapterChrono, AdapterTime, DateAdapter, TimeAdapter};
use mui_lab::autocomplete::Autocomplete;
use mui_lab::data_grid::DataGrid;
use mui_lab::date_picker::{DatePicker, Key};
use mui_lab::localization::{
    init_default_locales, register_locale, LocalePack, LocalizationProvider,
};
use mui_lab::masonry::Masonry;
use mui_lab::time_picker::TimePicker;
use mui_lab::timeline::{Timeline, TimelineEvent};
use mui_lab::tree_view::TreeNode;

/// Custom locale used to prove that the provider can be extended at
/// runtime by the community.
struct CustomLocale;

impl LocalePack for CustomLocale {
    fn code(&self) -> &'static str {
        "test"
    }

    fn format_date(&self, iso: &str) -> String {
        format!("test:{iso}")
    }
}

#[test]
fn localization_supports_custom_locale() {
    init_default_locales();
    register_locale(CustomLocale);

    let adapter = AdapterChrono;
    let provider = LocalizationProvider::new("test").expect("locale registered");
    let date = adapter.today();
    assert_eq!(
        provider.format_date(&date, &adapter),
        format!("test:{}", DateAdapter::format(&adapter, &date))
    );
}

#[test]
fn keyboard_navigation_moves_selection() {
    init_default_locales();
    let adapter = AdapterChrono;
    let mut picker = DatePicker::new(adapter);
    let start = picker.selected.clone();
    picker.handle_key(Key::Left);
    let expected = picker.adapter.add_days(&start, -1);
    assert_eq!(picker.selected, expected);
}

#[test]
fn time_picker_increments_selection() {
    init_default_locales();
    let adapter = AdapterChrono;
    let mut picker = TimePicker::new(adapter);
    let start = picker.selected.clone();
    picker.increment(60);
    let expected = picker.adapter.add_minutes(&start, 60);
    assert_eq!(picker.selected, expected);
}

#[test]
fn masonry_distributes_items_round_robin() {
    let mut masonry = Masonry::new(2);
    masonry.push(1);
    masonry.push(2);
    masonry.push(3);
    let layout = masonry.layout();
    assert_eq!(layout[0], vec![1, 3]);
    assert_eq!(layout[1], vec![2]);
}

#[test]
fn locale_pack_serializes_to_json() {
    let en = mui_lab::localization::EnUs::default();
    let json = serde_json::to_string(&en).expect("serialize locale");
    assert!(json.contains("date_format"));
}

#[test]
fn time_adapter_from_time_crate_roundtrip() {
    let adapter = AdapterTime;
    let now = adapter.now();
    let later = adapter.add_minutes(&now, 30);
    assert_eq!(adapter.add_minutes(&later, -30), now);
}

#[test]
fn autocomplete_returns_matching_options() {
    let ac = Autocomplete::new(vec!["apple".into(), "banana".into()]);
    assert_eq!(ac.suggestions("ba"), vec!["banana"]);
}

#[test]
fn data_grid_sorts_rows_ascending() {
    let mut grid = DataGrid::new(vec![3, 1, 2]);
    grid.sort_by(|a, b| a.cmp(b));
    assert_eq!(grid.rows, vec![1, 2, 3]);
}

#[test]
fn tree_node_toggle_expands() {
    let mut node = TreeNode::new("root");
    assert!(!node.expanded);
    node.toggle();
    assert!(node.expanded);
}

#[test]
fn timeline_orders_pushed_events() {
    let mut tl = Timeline::new();
    tl.push(TimelineEvent { at: 2, data: "b" });
    tl.push(TimelineEvent { at: 1, data: "a" });
    let events: Vec<_> = tl.events().iter().map(|e| e.data).collect();
    assert_eq!(events, vec!["a", "b"]);
}
