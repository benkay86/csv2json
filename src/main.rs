extern crate clap;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod cli;
mod data;

use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let cli_matches = cli::get_matches();

    let csv_file = cli_matches
        .value_of("in")
        .expect("You must specify an input csv with --in");
    let ds = cli_matches.value_of("dimensional-separator");
    let na = cli_matches.is_present("numeric-arrays");
    let file = File::open(csv_file).expect("Could not read csv file");
    let mut csv_reader = csv::Reader::from_reader(file);

    let data: Vec<Value> = csv_reader
        .deserialize()
        .filter(|result| result.is_ok())
        .map(|result| -> HashMap<String, String> { result.unwrap() })
        .map(|row| -> Value {
            let mut items = Map::new();

            row.into_iter().for_each(|(key, value)| {
                let (key, value) = data::dimensional_converter(key, value, ds);
                let prepared_value = data::prepare_upsert(items.entry(key.clone()), value);
                items.insert(key, prepared_value);
            });

            let items = json!(items);

            if na {
                return data::group_numeric_arrays(items);
            }

            items
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&data).unwrap());
}
