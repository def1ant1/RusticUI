use mui_lab::adapters::{AdapterChrono, DateAdapter};
use mui_lab::date_picker::{DatePicker, Key};
use mui_lab::localization::{init_default_locales, register_locale, LocalizationProvider, LocalePack};

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
        format!("test:{}", adapter.format(&date))
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

