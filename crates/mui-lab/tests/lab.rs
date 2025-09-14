use mui_lab::adapters::{AdapterChrono, AdapterTime, DateAdapter, TimeAdapter};
use mui_lab::date_picker::{DatePicker, Key};
use mui_lab::localization::{
    init_default_locales, register_locale, LocalePack, LocalizationProvider,
};
use mui_lab::masonry::Masonry;
use mui_lab::time_picker::TimePicker;

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
