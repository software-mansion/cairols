use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::project::model::configs_registry::package_config::merge_serde_json_value;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
struct Structured {
    a: bool,
    b: Nested,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
struct Nested {
    c: u8,
}

#[test]
fn merge_json_values_structured() {
    let mut a = serde_json::to_value(Structured::default()).unwrap();
    let b = serde_json::to_value(Structured { a: false, b: Nested { c: 10 } }).unwrap();

    merge_serde_json_value(&mut a, &b);

    let merged: Structured = serde_json::from_value(a).unwrap();
    assert_eq!(merged, Structured { a: false, b: Nested { c: 10 } })
}

type Unstructured = HashMap<u8, u8>;

#[test]
fn merge_json_values_unstructured() {
    let mut a = serde_json::to_value(Unstructured::from([(1, 1), (3, 2)])).unwrap();
    let b = serde_json::to_value(Unstructured::from([(2, 2), (3, 3)])).unwrap();

    merge_serde_json_value(&mut a, &b);

    let merged: Unstructured = serde_json::from_value(a).unwrap();
    assert_eq!(merged, Unstructured::from([(1, 1), (2, 2), (3, 3)]));
}
