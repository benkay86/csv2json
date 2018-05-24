use serde_json::Value as JsonValue;
use serde_json::map::Entry as JsonEntry;
use std::collections::hash_map::Entry;

pub fn dimensional_converter(key: String, value: String, ds: &Option<&str>) -> (String, JsonValue) {
    if let &Some(separator) = ds {
        if key.contains(separator) {
            let mut parts = key.split(separator);
            let this_key = parts.next().unwrap().to_owned();
            let key_chain = parts.collect::<Vec<&str>>().join(".").to_owned();
            let (next_key, data) = dimensional_converter(key_chain, value, &Some(separator));
            return (this_key, json!({ next_key: data }));
        }
    }
    (key, json!(value))
}

pub fn prepare_upsert(entry: Entry<String, JsonValue>, data: JsonValue) -> JsonValue {
    match entry {
        Entry::Vacant(_) => data,
        Entry::Occupied(e) => {
            let old_value = e.remove();
            merge_values(old_value, data)
        }
    }
}

fn merge_values(v1: JsonValue, v2: JsonValue) -> JsonValue {
    // If both values are objects combine on keys
    if v1.is_object() && v2.is_object() {
        if let JsonValue::Object(mut o1) = v1 {
            if let JsonValue::Object(mut o2) = v2 {
                o2.into_iter().for_each(|(key2, value2)| {
                    let replacement = match o1.entry(key2.to_owned()) {
                        JsonEntry::Vacant(_) => value2,
                        JsonEntry::Occupied(e) => {
                            let value1 = e.remove();
                            merge_values(value1, value2)
                        }
                    };
                    o1.insert(key2, replacement);
                });
                return json!(o1);
            }
            unreachable!();
        }
    }

    // If both values are arrays, add the other to it.
    if v1.is_array() && v2.is_array() {
        if let JsonValue::Array(mut a1) = v1 {
            if let JsonValue::Array(mut a2) = v2 {
                a1.append(&mut a2);
                return json!(a1);
            }
            unreachable!();
        }
    }

    // If either is an array add the other to it.
    if let JsonValue::Array(mut a1) = v1 {
        a1.push(v2);
        return json!(a1);
    }
    if let JsonValue::Array(mut a2) = v2 {
        a2.push(v1);
        return json!(a2);
    }

    // Otherwise create a new array with both items
    json!([v1, v2])
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dimensional_converter {
        #[test]
        fn it_does_simple_json_conversion_with_no_separator() {
            let key = String::from("first.second.third");
            let value = String::from("value");
            assert_eq!(
                super::dimensional_converter(key, value, &None),
                (String::from("first.second.third"), json!("value"))
            )
        }

        #[test]
        fn it_creates_objects_on_separation() {
            let key = String::from("first.second.third");
            let value = String::from("value");
            assert_eq!(
                super::dimensional_converter(key, value, &Some(".")),
                (String::from("first"), json!({"second":{"third":"value"}}))
            )
        }

        #[test]
        fn it_does_simple_json_conversion_when_seperator_not_found() {
            let key = String::from("first.second.third");
            let value = String::from("value");
            assert_eq!(
                super::dimensional_converter(key, value, &Some("-")),
                (String::from("first.second.third"), json!("value"))
            )
        }
    }

    mod merge_values {
        #[test]
        fn it_merges_scalars_correctly() {
            let v1 = json!("v1");
            let v2 = json!("v2");
            assert_eq!(super::merge_values(v1, v2), json!(["v1", "v2"]));
        }

        #[test]
        fn it_merges_a_scalars_with_an_array_correctly() {
            let v1 = json!(["v1"]);
            let v2 = json!("v2");
            assert_eq!(super::merge_values(v1, v2), json!(["v1", "v2"]));

            let v1 = json!("v1");
            let v2 = json!(["v2"]);
            assert_eq!(super::merge_values(v1, v2), json!(["v2", "v1"])); // v1 added to existing v2
        }

        #[test]
        fn it_merges_two_arrays_correctly() {
            let v1 = json!(["v1"]);
            let v2 = json!(["v2"]);
            assert_eq!(super::merge_values(v1, v2), json!(["v1", "v2"]));
        }

        #[test]
        fn it_merges_two_objects_correctly() {
            let v1 = json!({"k1":"v1"});
            let v2 = json!({"k2":"v2"});
            assert_eq!(super::merge_values(v1, v2), json!({"k1":"v1", "k2":"v2"}));
        }

        #[test]
        fn it_merges_values_inside_objects() {
            let v1 = json!({"k1":"v1"});
            let v2 = json!({"k1":"v2"});
            assert_eq!(super::merge_values(v1, v2), json!({"k1":["v1","v2"]}));
        }
    }
}
