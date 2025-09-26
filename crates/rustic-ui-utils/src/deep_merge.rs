use serde_json::Value;

/// Merge two JSON-like values deeply.
///
/// The `source` value is merged into `target` recursively. When both sides
/// contain an object for the same key, their entries are merged. Arrays are
/// concatenated. For all other type combinations the `source` value replaces
/// the existing entry in `target`.
///
/// # Examples
/// Basic object merging:
/// ```
/// use rustic_ui_utils::deep_merge;
/// use serde_json::json;
///
/// let mut a = json!({"a": {"b": 1}});
/// let b = json!({"a": {"c": 2}});
/// deep_merge(&mut a, b);
/// assert_eq!(a, json!({"a": {"b": 1, "c": 2}}));
/// ```
///
/// Arrays are appended:
/// ```
/// use rustic_ui_utils::deep_merge;
/// use serde_json::json;
///
/// let mut a = json!({"nums": [1,2]});
/// let b = json!({"nums": [3]});
/// deep_merge(&mut a, b);
/// assert_eq!(a, json!({"nums": [1,2,3]}));
/// ```
///
/// Primitive values replace the previous entry:
/// ```
/// use rustic_ui_utils::deep_merge;
/// use serde_json::json;
///
/// let mut a = json!({"value": 1});
/// deep_merge(&mut a, json!({"value": "overwritten"}));
/// assert_eq!(a, json!({"value": "overwritten"}));
/// ```
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
        (Value::Array(target_arr), Value::Array(source_arr)) => {
            target_arr.extend(source_arr);
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
