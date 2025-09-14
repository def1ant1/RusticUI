use mui_utils::compose_classes;
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
                let mut first = true;
                for opt in values {
                    if let Some(v) = opt {
                        if !first { buf.push(' '); } else { first = false; }
                        buf.push_str(&get(v));
                        if let Some(extra) = classes.get(v) {
                            buf.push(' ');
                            buf.push_str(extra);
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
