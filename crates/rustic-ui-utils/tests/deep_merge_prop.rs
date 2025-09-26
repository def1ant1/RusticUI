use rustic_ui_utils::deep_merge;
use proptest::prelude::*;
use serde_json::{Map, Value};

fn arb_value() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        any::<i64>().prop_map(|v| Value::Number(v.into())),
        any::<bool>().prop_map(Value::Bool),
        Just(Value::Null),
    ];
    leaf.prop_recursive(3, 8, 3, |inner| {
        prop::collection::btree_map(
            proptest::string::string_regex("[a-z]{1,3}").unwrap(),
            inner.clone(),
            0..3,
        )
        .prop_map(|m| {
            let map: Map<String, Value> = m.into_iter().collect();
            Value::Object(map)
        })
    })
}

fn arb_object() -> impl Strategy<Value = Value> {
    prop::collection::btree_map(
        proptest::string::string_regex("[a-z]{1,3}").unwrap(),
        arb_value(),
        0..3,
    )
    .prop_map(|m| {
        let map: Map<String, Value> = m.into_iter().collect();
        Value::Object(map)
    })
}

proptest! {
    #[test]
    fn associative(a in arb_object(), b in arb_object(), c in arb_object()) {
        let mut ab = a.clone();
        deep_merge(&mut ab, b.clone());
        deep_merge(&mut ab, c.clone());

        let mut bc = b.clone();
        deep_merge(&mut bc, c);
        let mut result = a.clone();
        deep_merge(&mut result, bc);
        prop_assert_eq!(ab, result);
    }

    #[test]
    fn identity(a in arb_object()) {
        let mut value = a.clone();
        deep_merge(&mut value, Value::Object(Map::new()));
        prop_assert_eq!(value, a);
    }
}
