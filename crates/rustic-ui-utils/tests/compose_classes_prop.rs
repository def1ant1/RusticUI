use rustic_ui_utils::compose_classes;
use proptest::prelude::*;
use std::collections::HashMap;

proptest! {
    #[test]
    fn matches_reference(
        slots in proptest::collection::hash_map(
            proptest::string::string_regex("[a-z]{1,3}").unwrap(),
            proptest::collection::vec(
                proptest::option::of(proptest::string::string_regex("[a-z]{1,3}").unwrap()),
                0..3
            ),
            0..3
        ),
        classes in proptest::collection::hash_map(
            proptest::string::string_regex("[a-z]{1,3}").unwrap(),
            proptest::string::string_regex("[a-z]{1,3}").unwrap(),
            0..5
        )
    ) {
        let get = |s: &str| format!("u-{s}");
        let reference = {
            let mut out = HashMap::new();
            for (slot, values) in &slots {
                let mut buf = String::new();
                let mut seen = std::collections::HashSet::new();
                for opt in values {
                    if let Some(v) = opt {
                        if seen.insert(v.clone()) {
                            let util = get(v);
                            if !util.is_empty() {
                                if !buf.is_empty() { buf.push(' '); }
                                buf.push_str(&util);
                            }
                            if let Some(extra) = classes.get(v) {
                                if !extra.is_empty() {
                                    if !buf.is_empty() { buf.push(' '); }
                                    buf.push_str(extra);
                                }
                            }
                        }
                    }
                }
                out.insert(slot.clone(), buf);
            }
            out
        };
        let output = compose_classes(&slots, get, Some(&classes));
        prop_assert_eq!(output, reference);
    }
}
