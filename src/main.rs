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
use std::io::Read;
use strfmt::strfmt;

fn main() {
    let cli_matches = cli::get_matches();

    let csv_file = cli_matches.value_of(cli::IN);
    let out_dir = cli_matches.value_of(cli::OUT_DIR);
    let out_name = cli_matches.value_of(cli::OUT_NAME);
    let delimiter = cli_matches.value_of(cli::DELIMITER).unwrap(); // Has a default
    let delimiter_byte = *delimiter.as_bytes().first().expect("No delimiter provided");
    let ds = cli_matches.value_of(cli::DIMENSIONAL_SEPARATOR);
    let na = cli_matches.is_present(cli::ARRAYS);
    let res = cli_matches.is_present(cli::REMOVE_EMPTY_STRINGS);
    let reo = cli_matches.is_present(cli::REMOVE_EMPTY_OBJECTS);
    let boolean_columns = cli_matches
        .values_of_lossy(cli::BOOLEAN)
        .unwrap_or_else(|| vec![]);
    let numeric_columns = cli_matches
        .values_of_lossy(cli::NUMERIC)
        .unwrap_or_else(|| vec![]);
    let fold = cli_matches.is_present(cli::FOLD);
    let jsonl = cli_matches.is_present(cli::JSONL);
    let reader: Box<dyn Read> = match csv_file {
        Some(csv_file) => {
            let file = File::open(csv_file).expect("Could not read csv file");
            Box::new(file)
        },
        None => {
            eprintln!("Reading from standard input, press Ctrl+D or Ctrl+C to exit.");
            eprintln!("Use --in if you meant to specify a csv file.");
            eprintln!("Use --help for usage information.");
            Box::new(std::io::stdin())
        }
    };
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(delimiter_byte)
        .from_reader(reader);

    let raw_rows: Vec<HashMap<String, String>> = csv_reader
        .deserialize()
        .filter(|result| result.is_ok())
        .map(|result| -> HashMap<String, String> { result.unwrap() })
        .filter(|row| !row.is_empty())
        .collect();

    let typed_rows: Vec<HashMap<String, Value>> = raw_rows
        .iter()
        .map(data::row_to_values)
        .map(|map| data::columns_to_numbers(&numeric_columns, map))
        .map(|map| data::columns_to_booleans(&boolean_columns, map))
        .collect();

    let mut items: Value = typed_rows
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
    if fold {
        items = data::fold(items, csv_reader.headers().unwrap(), ds);
    }

    if let Some(out_dir) = out_dir {
        if let Some(out_name) = out_name {
            // If a template name was used.
            // With a template jsonl makes no sense.
            let raw_rows_iter = raw_rows.into_iter();
            let items_iter = items.as_array().unwrap().iter().cloned();

            raw_rows_iter.zip(items_iter).for_each(|(raw, data)| {
                let output = serde_json::to_string_pretty(&data).unwrap();
                let file_name = strfmt(&out_name, &raw).unwrap();
                sys::write_json_to_file(&out_dir, &file_name, &output)
                    .expect("Failed to write to file");
            })
        } else {
            // If no template name was provided
            let csv_file = match csv_file {
                Some(csv_file) => csv_file, // use the same name as the input file
                None => "output"            // otherwise default to output.json
            };

            let output = if jsonl {
                items.as_array().unwrap().iter()
                              .map(|item| serde_json::to_string(&item).unwrap())
                              .collect::<Vec<String>>()
                              .join("\n")
            } else {
                serde_json::to_string_pretty(&items).unwrap()
            };

            let file_name = sys::get_file_name(&csv_file);
            sys::write_json_to_file(&out_dir, &file_name, &output)
                .expect("Failed to write to file");
        }
    } else {
        // If no output was specified
        let output = if jsonl {
            items.as_array().unwrap().iter()
                          .map(|item| serde_json::to_string(&item).unwrap())
                          .collect::<Vec<String>>()
                          .join("\n")
        } else {
            serde_json::to_string_pretty(&items).unwrap()
        };

        println!("{}", output);
    }
}
