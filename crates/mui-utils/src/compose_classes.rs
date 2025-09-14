use std::collections::HashMap;

/// Compose CSS class names for each slot from multiple sources.
///
/// Slots map to arrays of optional class keys. For every defined key the
/// `get_utility_class` function resolves the base utility class. If `classes`
/// contains an entry for the same key its value is appended as well.
///
/// # Performance
/// Each slot is processed in a single pass and output strings are built with
/// pre-allocated buffers, minimizing temporary allocations.
pub fn compose_classes<F>(
    slots: &HashMap<String, Vec<Option<String>>>,
    get_utility_class: F,
    classes: Option<&HashMap<String, String>>,
) -> HashMap<String, String>
where
    F: Fn(&str) -> String,
{
    let mut out = HashMap::with_capacity(slots.len());
    for (slot_name, slot_values) in slots {
        let mut buf = String::new();
        let mut first = true;
        for opt in slot_values {
            if let Some(ref value) = opt {
                if !first {
                    buf.push(' ');
                } else {
                    first = false;
                }
                buf.push_str(&get_utility_class(value));
                if let Some(class_map) = classes {
                    if let Some(extra) = class_map.get(value) {
                        buf.push(' ');
                        buf.push_str(extra);
                    }
                }
            }
        }
        out.insert(slot_name.clone(), buf);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_classes() {
        let mut slots = HashMap::new();
        slots.insert(
            "root".to_string(),
            vec![Some("root".to_string()), Some("primary".to_string())],
        );
        let get = |s: &str| format!("MuiButton-{s}");
        let mut classes = HashMap::new();
        classes.insert("root".to_string(), "my-root-class".to_string());
        let out = compose_classes(&slots, get, Some(&classes));
        let mut expected = HashMap::new();
        expected.insert(
            "root".to_string(),
            "MuiButton-root my-root-class MuiButton-primary".to_string(),
        );
        assert_eq!(out, expected);
    }
}
