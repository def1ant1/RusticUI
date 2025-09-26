use std::collections::{HashMap, HashSet};

/// Compose CSS class names for each slot from multiple sources.
///
/// Slots map to arrays of optional class keys. For every defined key the
/// `get_utility_class` function resolves the base utility class. If `classes`
/// contains an entry for the same key its value is appended as well. Empty
/// strings are ignored and duplicate keys are de-duplicated to avoid bloated
/// `class` attributes.
///
/// # Examples
/// Basic composition with user provided overrides:
/// ```
/// use std::collections::HashMap;
/// use mui_utils::compose_classes;
///
/// let slots = HashMap::from([
///     ("root".to_string(), vec![Some("root".into()), Some("primary".into())])
/// ]);
/// let classes = HashMap::from([("root".to_string(), "custom".to_string())]);
/// let get = |s: &str| format!("MuiButton-{s}");
/// let out = compose_classes(&slots, get, Some(&classes));
/// assert_eq!(out.get("root").unwrap(), "MuiButton-root custom MuiButton-primary");
/// ```
///
/// Duplicate and empty entries are skipped automatically:
/// ```
/// use std::collections::HashMap;
/// use mui_utils::compose_classes;
///
/// let slots = HashMap::from([
///     ("root".to_string(), vec![Some("root".into()), Some("root".into()), Some("".into())])
/// ]);
/// // Identity resolver – returns the key verbatim
/// let out = compose_classes(&slots, |s| s.to_string(), None);
/// assert_eq!(out.get("root").unwrap(), "root");
/// ```
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
        let mut seen = HashSet::new();
        for value in slot_values.iter().flatten() {
            // Avoid repeating the same utility key
            if seen.insert(value.clone()) {
                let util = get_utility_class(value);
                if !util.is_empty() {
                    if !buf.is_empty() {
                        buf.push(' ');
                    }
                    buf.push_str(&util);
                }
                if let Some(class_map) = classes {
                    if let Some(extra) = class_map.get(value) {
                        if !extra.is_empty() {
                            if !buf.is_empty() {
                                buf.push(' ');
                            }
                            buf.push_str(extra);
                        }
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
