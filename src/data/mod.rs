use serde_json::{map::Entry, Map, Number, Value};
use std::collections::{hash_map::Entry as HashMapEntry, HashMap};

pub fn group_numeric_arrays(value: Value) -> Value {
    match value {
        Value::Object(object) => group_numeric_arrays_in_object(object),
        Value::Array(arr) => group_numeric_arrays_in_array(arr),
        _ => value,
    }
}

fn group_numeric_arrays_in_object(object: Map<String, Value>) -> Value {
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
}

fn group_numeric_arrays_in_array(arr: Vec<Value>) -> Value {
    arr.into_iter().map(group_numeric_arrays).collect()
}

pub fn dimensional_converter(key: &str, value: &Value, ds: Option<&str>) -> (String, Value) {
    if let Some(separator) = ds {
        if key.contains(separator) {
            let mut parts = key.split(separator);
            let this_key = parts.next().unwrap().to_owned();
            let key_chain = parts.collect::<Vec<&str>>().join(".").to_owned();
            let (next_key, data) = dimensional_converter(&key_chain, value, Some(separator));
            return (this_key, json!({ next_key: data }));
        }
    }
    (key.to_owned(), json!(value))
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

pub fn remove_empty_objects(value: Value) -> Value {
    match value {
        Value::Object(object) => remove_empty_objects_from_object(object),
        Value::Array(arr) => remove_empty_objects_from_array(arr),
        _ => value,
    }
}

fn remove_empty_objects_from_object(object: Map<String, Value>) -> Value {
    let new_object: Map<String, Value> = object
        .into_iter()
        .map(|(key, value)| (key, remove_empty_objects(value)))
        .filter(|(_key, value)| !(value.is_object() && value.as_object().unwrap().is_empty()))
        .collect();
    json!(new_object)
}

fn remove_empty_objects_from_array(arr: Vec<Value>) -> Value {
    let new_arr: Vec<Value> = arr
        .into_iter()
        .map(remove_empty_objects)
        .filter(|value| !(value.is_object() && value.as_object().unwrap().is_empty()))
        .collect();
    json!(new_arr)
}

pub fn remove_empty_strings(value: Value) -> Value {
    match value {
        Value::Object(object) => remove_empty_strings_from_object(object),
        Value::Array(arr) => remove_empty_strings_from_array(arr),
        _ => value,
    }
}

fn remove_empty_strings_from_object(object: Map<String, Value>) -> Value {
    let new_object: Map<String, Value> = object
        .into_iter()
        .map(|(key, value)| (key, remove_empty_strings(value)))
        .filter(|(_key, value)| !(value.is_string() && value.as_str().unwrap().is_empty()))
        .collect();
    json!(new_object)
}

fn remove_empty_strings_from_array(arr: Vec<Value>) -> Value {
    let new_arr: Vec<Value> = arr
        .into_iter()
        .map(remove_empty_strings)
        .filter(|value| !(value.is_string() && value.as_str().unwrap().is_empty()))
        .collect();
    json!(new_arr)
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

fn string_to_bool(string: &str) -> bool {
    match string.to_lowercase().as_str() {
        "" => false,
        "0" => false,
        "false" => false,
        _ => true,
    }
}

fn number_to_bool(number: &Number) -> bool {
    if number.is_u64() {
        number.as_u64().unwrap() != 0
    } else if number.is_i64() {
        number.as_i64().unwrap() != 0
    } else if number.is_f64() {
        number.as_f64().unwrap() != 0.0
    } else {
        panic!("serde_json have changed their api, it is no longer possible to determine the type of numbers")
    }
}

pub fn value_to_bool(value: &Value) -> bool {
    match value {
        &Value::Null => false,
        Value::Bool(boolean) => *boolean,
        Value::Number(number) => number_to_bool(&number),
        Value::String(string) => string_to_bool(&string),
        Value::Array(array) => !array.is_empty(),
        Value::Object(object) => !object.is_empty(),
    }
}

fn boolean_to_number(boolean: bool) -> Number {
    if boolean {
        1.into()
    } else {
        0.into()
    }
}

fn string_to_number(string: &str) -> Result<Number, &str> {
    if string.is_empty() {
        let zero = json!(0);
        if let Value::Number(zero) = zero {
            return Ok(zero);
        }
    }
    if let Ok(unsigned) = string.parse::<u64>() {
        let unsigned = json!(unsigned);
        if let Value::Number(unsigned) = unsigned {
            return Ok(unsigned);
        }
    }
    if let Ok(signed) = string.parse::<i64>() {
        let signed = json!(signed);
        if let Value::Number(signed) = signed {
            return Ok(signed);
        }
    }
    if let Ok(float) = string.parse::<f64>() {
        let float = json!(float);
        if let Value::Number(float) = float {
            return Ok(float);
        }
    }
    Err(string)
}

pub fn value_to_number(value: &Value) -> Number {
    match value {
        &Value::Null => 0.into(),
        Value::Bool(boolean) => boolean_to_number(*boolean),
        Value::Number(number) => number.clone(),
        Value::String(string) => {
            string_to_number(&string).expect("Could not calculate numeric value of column")
        }
        Value::Array(array) => boolean_to_number(!array.is_empty()),
        Value::Object(object) => boolean_to_number(!object.is_empty()),
    }
}

pub fn row_to_values(row: &HashMap<String, String>) -> HashMap<String, Value> {
    row.iter()
        .map(|(key, value)| (key.clone(), Value::String(value.clone())))
        .collect()
}

pub fn columns_to_booleans(
    columns: &[String],
    mut row: HashMap<String, Value>,
) -> HashMap<String, Value> {
    columns.iter().for_each(|column| {
        if let HashMapEntry::Occupied(entry) = row.entry(column.to_string()) {
            *entry.into_mut() = Value::Bool(value_to_bool(entry.get()));
        }
    });
    row
}

pub fn columns_to_numbers(
    columns: &[String],
    mut row: HashMap<String, Value>,
) -> HashMap<String, Value> {
    columns.iter().for_each(|column| {
        if let HashMapEntry::Occupied(entry) = row.entry(column.to_string()) {
            *entry.into_mut() = Value::Number(value_to_number(entry.get()));
        }
    });
    row
}

// Fold array of json objects (one object for each row) into one root object containing an
// an array for each column, where the length of each array is equal to the # of rows.
pub fn fold(mut items: Value, headers: &csv::StringRecord, ds: Option<&str>) -> Value {
    // Clean up headers to account for dimensional separators.
    let headers = distill_headers(headers, ds);
    
    // Initialize root object with an empty array for each column. 
    let mut root_object = Value::Object(Map::new());
    headers.into_iter().for_each( |header| {
        root_object.as_object_mut().unwrap().insert(header.into(), Value::Array(Vec::new()));
    });
    
    // Move each row into the arrays under the root object.
    items.as_array_mut().unwrap().into_iter().for_each( |row| {
        // Put empty values back into row.
        root_object
            .as_object_mut()
            .unwrap()
            .iter_mut()
            .for_each( |(k,a)| {
                match row.as_object_mut().unwrap().remove(k) {
                    Some(v) => {
                        a.as_array_mut().unwrap().push(v);
                    },
                    None => {
                        a.as_array_mut().unwrap().push(Value::Null);
                    }
                }
            });
    });
    
    // All done.
    return root_object;
}

// If we were called with a separator, remove separated items from header.
// E.g., `-D .` then header `foo.bar` becomes just `foo`.
fn distill_headers(headers: &csv::StringRecord, ds: Option<&str>) -> Vec<String> {
    match ds {
        Some(ds) => {
            // For each header, trim off the bits after the dimensional separator.
            let mut headers: Vec<String> = headers.into_iter().map(
                |s| s.split(ds).next().unwrap().into()
            ).collect();
            
            // Then remove duplicates.
            headers.sort();
            headers.dedup();
            headers
        },
        None => {
            // No dimensional separator, just return headers without changing anything.
            headers.into_iter().map(|s| s.into()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dimensional_converter {
        #[test]
        fn it_does_simple_json_conversion_with_no_separator() {
            let key = String::from("first.second.third");
            let value = json!("value");
            assert_eq!(
                super::dimensional_converter(&key, &value, None),
                (String::from("first.second.third"), value)
            )
        }

        #[test]
        fn it_creates_objects_on_separation() {
            let key = String::from("first.second.third");
            let value = json!("value");
            assert_eq!(
                super::dimensional_converter(&key, &value, Some(".")),
                (String::from("first"), json!({"second":{"third":&value}}))
            )
        }

        #[test]
        fn it_does_simple_json_conversion_when_seperator_not_found() {
            let key = String::from("first.second.third");
            let value = json!("value");
            assert_eq!(
                super::dimensional_converter(&key, &value, Some("-")),
                (String::from("first.second.third"), value)
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

    mod number_to_bool {
        #[test]
        fn it_converts_u64s() {
            let zero: u64 = 0;
            let one: u64 = 1;

            let zero = match json!(zero) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };
            let one = match json!(one) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };

            assert!(!super::number_to_bool(&zero));
            assert!(super::number_to_bool(&one));
        }

        #[test]
        fn it_converts_i64s() {
            let zero: i64 = 0;
            let one: i64 = 1;
            let minus_one: i64 = 1;

            let zero = match json!(zero) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };
            let one = match json!(one) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };
            let minus_one = match json!(minus_one) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };

            assert!(!super::number_to_bool(&zero));
            assert!(super::number_to_bool(&one));
            assert!(super::number_to_bool(&minus_one));
        }

        #[test]
        fn it_converts_f64s() {
            let zero: f64 = 0.0;
            let one: f64 = 1.0;
            let huge: f64 = 1.0e+40;
            let tiny: f64 = 1.0e-40;

            let zero = match json!(zero) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };
            let one = match json!(one) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };
            let huge = match json!(huge) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };
            let tiny = match json!(tiny) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };

            assert!(!super::number_to_bool(&zero));
            assert!(super::number_to_bool(&one));
            assert!(super::number_to_bool(&huge));
            assert!(super::number_to_bool(&tiny));
        }
    }

    mod boolean_to_number {
        #[test]
        fn it_converts_true_to_one() {
            let b = true;
            let one = match json!(1) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };
            assert_eq!(super::boolean_to_number(b), one)
        }

        #[test]
        fn it_converts_false_to_zero() {
            let b = false;
            let zero = match json!(0) {
                super::Value::Number(num) => num,
                _ => panic!("Number not created correctly"),
            };
            assert_eq!(super::boolean_to_number(b), zero)
        }
    }

    mod string_to_number {
        #[test]
        fn it_converts_positive_numbers() {
            let n = "1";
            assert_eq!(json!(super::string_to_number(n).unwrap()), json!(1))
        }

        #[test]
        fn it_converts_negative_numbers() {
            let n = "-1";
            assert_eq!(json!(super::string_to_number(n).unwrap()), json!(-1))
        }

        #[test]
        fn it_converts_floats() {
            let n = "1.0e3";
            assert_eq!(json!(super::string_to_number(n).unwrap()), json!(1000.0))
        }

        #[test]
        fn it_converts_zero() {
            let n = "0";
            assert_eq!(json!(super::string_to_number(n).unwrap()), json!(0))
        }

        #[test]
        fn it_converts_an_empty_string() {
            let n = "";
            assert_eq!(json!(super::string_to_number(n).unwrap()), json!(0))
        }
    }
    
    mod fold {
        #[test]
        fn it_distills_plain_headers() {
            let mut headers = vec!["foo", "bar"];
            headers.sort();
            let csv_headers = csv::StringRecord::from(headers.clone());
            let csv_headers = super::distill_headers(&csv_headers, None);
            let matching = csv_headers.iter().zip(headers).filter(|&(a,b)| a == b).count(); 
            assert_eq!(matching, 2);
        }
        
        #[test]
        fn it_distills_dimensional_headers() {
            let headers = vec!["foo.1", "foo.2", "bar"];
            let csv_headers = csv::StringRecord::from(headers);
            let csv_headers = super::distill_headers(&csv_headers, Some("."));
            let mut headers = vec!["foo", "bar"];
            headers.sort();
            let matching = csv_headers.iter().zip(headers).filter(|&(a,b)| a == b).count(); 
            assert_eq!(matching, 2);
        }
        
        #[test]
        fn it_folds_plain() {
            let mut headers = vec!["bar", "foo"];
            headers.sort();
            let items = super::fold(
                json!([
                    {
                        "bar": "a",
                        "foo": 1
                    },
                    {
                        "bar": "b",
                        "foo": 2
                    },
                    {
                        "bar": "c",
                        "foo": 3
                    },
                ]),
                &csv::StringRecord::from(headers),
                None
            );
            assert_eq!(
                items,
                json!({
                    "bar": ["a", "b", "c"],
                    "foo": [1, 2, 3]
                })
            );
        }
        
        #[test]
        fn it_folds_dimensional() {
            let mut headers = vec!["bar", "foo.1", "foo.2"];
            headers.sort();
            let items = super::fold(
                json!([
                    {
                        "bar": "a",
                        "foo": [1, 11]
                    },
                    {
                        "bar": "b",
                        "foo": [2, 22]
                    },
                    {
                        "bar": "c",
                        "foo": [3, 33]
                    },
                ]),
                &csv::StringRecord::from(headers),
                Some(".")
            );
            assert_eq!(
                items,
                json!({
                    "bar": ["a", "b", "c"],
                    "foo": [
                        [1, 11],
                        [2, 22],
                        [3, 33]
                    ]
                })
            );
        }
    }
}
