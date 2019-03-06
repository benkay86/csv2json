use serde_json::{map::Entry, Map, Value};

pub fn group_numeric_arrays(value: Value) -> Value {
    // If there are no non-numeric keys we can turn this group into an array.
    if let Value::Object(object) = value {
        // Recurse over each element in the object
        let object: Map<String, Value> = object
            .into_iter()
            .map(|(key, value)| {
                let replacement = group_numeric_arrays(value);
                (key, replacement)
            })
            .collect();

        // Test if this object should be an array
        let remaining_keys: Vec<bool> = object
            .keys()
            .filter(|k| k.parse::<u64>().is_err()) // Find anything the doesn't parse to u64
            .map(|_| true)
            .collect();

        if remaining_keys.is_empty() {
            let values: Vec<Value> = object.values().map(|i| i.to_owned()).collect();
            json!(values)
        } else {
            json!(object)
        }
    } else {
        value
    }
}

pub fn dimensional_converter(key: String, value: String, ds: Option<&str>) -> (String, Value) {
    if let Some(separator) = ds {
        if key.contains(separator) {
            let mut parts = key.split(separator);
            let this_key = parts.next().unwrap().to_owned();
            let key_chain = parts.collect::<Vec<&str>>().join(".").to_owned();
            let (next_key, data) = dimensional_converter(key_chain, value, Some(separator));
            return (this_key, json!({ next_key: data }));
        }
    }
    (key, json!(value))
}

pub fn prepare_upsert(entry: Entry, data: Value) -> Value {
    match entry {
        Entry::Vacant(_) => data,
        Entry::Occupied(e) => {
            let old_value = e.remove();
            merge_values(old_value, data)
        }
    }
}

pub fn remove_empty_objects(value: &mut Value) {
    match value {
        Value::Object(object) => remove_empty_objects_from_object(object),
        Value::Array(arr) => remove_empty_objects_from_array(arr),
        _ => {}
    }
}

fn remove_empty_objects_from_object(object: &mut Map<String, Value>) {
    let mut keys_to_remove: Vec<String> = vec![];
    object.iter_mut().for_each(|(key, mut value)| {
        if value.is_object() && value.as_object().unwrap().is_empty() {
            keys_to_remove.push(key.to_owned());
        } else {
            remove_empty_objects(&mut value)
        }
    });
    keys_to_remove.iter().for_each(|key| {
        object.remove(key);
    })
}

fn remove_empty_objects_from_array(arr: &mut Vec<Value>) {
    arr.retain(|value| !(value.is_object() && value.as_object().unwrap().is_empty()));
    arr.iter_mut().for_each(|value| remove_empty_objects(value));
}

pub fn remove_empty_strings(value: &mut Value) {
    match value {
        Value::Object(object) => remove_empty_strings_from_object(object),
        Value::Array(arr) => remove_empty_strings_from_array(arr),
        _ => {}
    }
}

fn remove_empty_strings_from_object(object: &mut Map<String, Value>) {
    let mut keys_to_remove: Vec<String> = vec![];
    object.iter_mut().for_each(|(key, mut value)| {
        if value.is_string() && value.as_str().unwrap().is_empty() {
            keys_to_remove.push(key.to_owned());
        } else {
            remove_empty_strings(&mut value)
        }
    });
    keys_to_remove.iter().for_each(|key| {
        object.remove(key);
    })
}

fn remove_empty_strings_from_array(arr: &mut Vec<Value>) {
    arr.retain(|value| !(value.is_string() && value.as_str().unwrap().is_empty()));
    arr.iter_mut().for_each(|value| remove_empty_strings(value))
}

fn merge_values(v1: Value, v2: Value) -> Value {
    // If both values are objects combine on keys
    if v1.is_object() && v2.is_object() {
        if let Value::Object(mut o1) = v1 {
            if let Value::Object(mut o2) = v2 {
                o2.into_iter().for_each(|(key2, value2)| {
                    let replacement = match o1.entry(key2.to_owned()) {
                        Entry::Vacant(_) => value2,
                        Entry::Occupied(e) => {
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
        if let Value::Array(mut a1) = v1 {
            if let Value::Array(mut a2) = v2 {
                a1.append(&mut a2);
                return json!(a1);
            }
            unreachable!();
        }
    }

    // If either is an array add the other to it.
    if let Value::Array(mut a1) = v1 {
        a1.push(v2);
        return json!(a1);
    }
    if let Value::Array(mut a2) = v2 {
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
                super::dimensional_converter(key, value, None),
                (String::from("first.second.third"), json!("value"))
            )
        }

        #[test]
        fn it_creates_objects_on_separation() {
            let key = String::from("first.second.third");
            let value = String::from("value");
            assert_eq!(
                super::dimensional_converter(key, value, Some(".")),
                (String::from("first"), json!({"second":{"third":"value"}}))
            )
        }

        #[test]
        fn it_does_simple_json_conversion_when_seperator_not_found() {
            let key = String::from("first.second.third");
            let value = String::from("value");
            assert_eq!(
                super::dimensional_converter(key, value, Some("-")),
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
