use serde_json::Value;

/// Merge two JSON-like values deeply.
///
/// The `source` value is merged into `target` recursively. When both sides
/// contain an object for the same key, their entries are merged. Otherwise the
/// `source` value replaces the existing entry in `target`.
///
/// # Performance
/// This function operates in-place and moves values from `source` into
/// `target` to avoid unnecessary cloning. Heap allocations only occur when new
/// keys are introduced.
pub fn deep_merge(target: &mut Value, source: Value) {
    match (target, source) {
        (Value::Object(target_map), Value::Object(source_map)) => {
            for (key, value) in source_map {
                match target_map.get_mut(&key) {
                    Some(existing) => deep_merge(existing, value),
                    None => {
                        target_map.insert(key, value);
                    }
                }
            }
        }
        (t, s) => {
            *t = s;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merges_recursively() {
        let mut a = json!({"a": {"b": 1}, "d": 2});
        let b = json!({"a": {"c": 2}, "d": 4});
        deep_merge(&mut a, b);
        assert_eq!(a, json!({"a": {"b": 1, "c": 2}, "d": 4}));
    }
}
