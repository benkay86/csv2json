use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, ArgMatches};

pub const DELIMITER: &str = "delimiter";
pub const DIMENSIONAL_SEPARATOR: &str = "dimensional-separator";
pub const ARRAYS: &str = "arrays";
pub const REMOVE_EMPTY_STRINGS: &str = "remove-empty-strings";
pub const REMOVE_EMPTY_OBJECTS: &str = "remove-empty-objects";
pub const IN: &str = "in";
pub const OUT_DIR: &str = "out-dir";
pub const OUT_NAME: &str = "out-name";
pub const BOOLEAN: &str = "boolean";
pub const NUMERIC: &str = "numeric";
pub const FOLD: &str = "fold";

pub fn get_matches<'a>() -> ArgMatches<'a> {
    configure_app().get_matches()
}

fn configure_app<'a, 'b>() -> App<'a, 'b> {
    app_from_crate!("\n")
        .arg(
            Arg::with_name(IN)
                .short("i")
                .long(IN)
                .value_name("FILE")
                .help("The csv file to read, otherwise reads stdin")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name(OUT_DIR)
                .short("o")
                .long(OUT_DIR)
                .value_name("DIR")
                .help("Where to save the json file(s)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(OUT_NAME)
                .short("f")
                .long(OUT_NAME)
                .value_name("TEMPLATE")
                .help("The template to use for naming multiple output files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(DELIMITER)
                .short("d")
                .long(DELIMITER)
                .value_name("DELIMITER")
                .help("What delimiter does your csv use")
                .takes_value(true)
                .default_value(","),
        )
        .arg(
            Arg::with_name(DIMENSIONAL_SEPARATOR)
                .short("D")
                .long(DIMENSIONAL_SEPARATOR)
                .value_name("SEPARATOR")
                .help("A separator to break header names allowing you to create deeper objects")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARRAYS)
                .short("a")
                .long(ARRAYS)
                .help("Indicates the csv contains arrays represented by numeric keys. Use with -D")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(BOOLEAN)
                .short("b")
                .long(BOOLEAN)
                .value_name("COLUMN")
                .help("Indicate that COLUMN is a boolean")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name(NUMERIC)
                .short("n")
                .long(NUMERIC)
                .value_name("COLUMN")
                .help("Indicate that COLUMN is numeric")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name(REMOVE_EMPTY_STRINGS)
                .long(REMOVE_EMPTY_STRINGS)
                .value_name(REMOVE_EMPTY_STRINGS)
                .help("Removes keys that contain empty strings")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(REMOVE_EMPTY_OBJECTS)
                .long(REMOVE_EMPTY_OBJECTS)
                .value_name(REMOVE_EMPTY_OBJECTS)
                .help("Removes keys that contain empty objects")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(FOLD)
                .long(FOLD)
                .short("F")
                .value_name(FOLD)
                .help("Fold array of json objects into one object with an array for each column")
                .takes_value(false),
        )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_has_the_correct_name() {
        let app = super::configure_app();
        assert_eq!(app.get_name(), "csv2json");
    }
}
