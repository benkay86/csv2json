extern crate clap;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate strfmt;

mod cli;
mod data;
mod sys;

use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use strfmt::strfmt;

fn main() {
    let cli_matches = cli::get_matches();

    let csv_file = cli_matches
        .value_of(cli::IN)
        .expect("You must specify an input csv with --in");
    let out_dir = cli_matches.value_of(cli::OUT_DIR);
    let out_name = cli_matches.value_of(cli::OUT_NAME);
    let ds = cli_matches.value_of(cli::DIMENSIONAL_SEPARATOR);
    let na = cli_matches.is_present(cli::ARRAYS);
    let res = cli_matches.is_present(cli::REMOVE_EMPTY_STRINGS);
    let reo = cli_matches.is_present(cli::REMOVE_EMPTY_OBJECTS);
    let file = File::open(csv_file).expect("Could not read csv file");
    let boolean_columns = cli_matches.values_of_lossy(cli::BOOLEAN);
    let boolean_columns = boolean_columns.unwrap_or_else(|| vec![]);
    let mut csv_reader = csv::Reader::from_reader(file);

    let raw_rows: Vec<HashMap<String, Value>> = csv_reader
        .deserialize()
        .filter(|result| result.is_ok())
        .map(|result| -> HashMap<String, String> { result.unwrap() })
        .filter(|row| !row.is_empty())
        .map(data::row_to_values)
        .map(|map| -> HashMap<String, Value> {
            map.into_iter()
                .map(|(key, value)| {
                    if boolean_columns.contains(&key) {
                        (key, Value::Bool(data::value_to_bool(&value)))
                    } else {
                        (key, value)
                    }
                })
                .collect()
        })
        .collect();

    let mut items: Value = raw_rows
        .iter()
        .map(|row| -> Value {
            let mut items = Map::new();

            row.iter().for_each(|(key, value)| {
                let (key, value) = data::dimensional_converter(key, value, ds);
                let prepared_value = data::prepare_upsert(items.entry(key.clone()), value);
                items.insert(key, prepared_value);
            });

            json!(items)
        })
        .collect();

    if na {
        items = data::group_numeric_arrays(items);
    }
    if res {
        items = data::remove_empty_strings(items);
    }
    if reo {
        items = data::remove_empty_objects(items);
    }

    if let Some(out_dir) = out_dir {
        if let Some(out_name) = out_name {
            // If a template name was used.
            let raw_rows_iter = raw_rows.into_iter();
            let items_iter = items.as_array().unwrap().iter().cloned();
            let paired_data: Vec<(HashMap<String, Value>, Value)> =
                raw_rows_iter.zip(items_iter).collect();

            paired_data.iter().for_each(|(raw, data)| {
                let output = serde_json::to_string_pretty(&data).unwrap();
                let file_name = strfmt(&out_name, raw).unwrap();
                sys::write_json_to_file(&out_dir, &file_name, &output)
                    .expect("Failed to write to file");
            })
        } else {
            // If no template name was provided
            let output = serde_json::to_string_pretty(&items).unwrap();
            let file_name = sys::get_file_name(&csv_file);
            sys::write_json_to_file(&out_dir, &file_name, &output)
                .expect("Failed to write to file");
        }
    } else {
        // If no output was specified
        let output = serde_json::to_string_pretty(&items).unwrap();
        println!("{}", output);
    }
}
