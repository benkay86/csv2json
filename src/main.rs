extern crate clap;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod cli;
mod data;

use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let cli_matches = cli::get_matches();

    let csv_file = cli_matches
        .value_of("in")
        .expect("You must specify an input csv with --in");
    let ds = cli_matches.value_of("dimensional-separator");
    let file = File::open(csv_file).expect("Could not read csv file");
    let mut csv_reader = csv::Reader::from_reader(file);


    let data: Vec<HashMap<String, JsonValue>> = csv_reader
        .deserialize()
        .filter(|result| result.is_ok())
        .map(|result| {
            let row: HashMap<String, String> = result.unwrap();
            row
        })
        .map(|row| {
            let mut items: HashMap<String, JsonValue> = HashMap::new();

            row.iter().for_each(|(key, value)| {
                let (key, value) = data::dimensional_converter(key.clone(), value.clone(), &ds);
                let prepared_value = data::prepare_upsert(items.entry(key.clone()), value);
                items.insert(key, prepared_value);
            });

            items
        })
        .collect();

        println!("{}", serde_json::to_string_pretty(&data).unwrap());
}
