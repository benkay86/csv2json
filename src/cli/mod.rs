use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, ArgMatches};

pub const DIMENSIONAL_SEPARATOR: &str = "dimensional-separator";
pub const NUMERIC_ARRAYS: &str = "numeric-arrays";
pub const REMOVE_EMPTY_STRINGS: &str = "remove-empty-strings";
pub const REMOVE_EMPTY_OBJECTS: &str = "remove-empty-objects";

pub fn get_matches<'a>() -> ArgMatches<'a> {
    configure_app().get_matches()
}

fn configure_app<'a, 'b>() -> App<'a, 'b> {
    app_from_crate!("\n")
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .value_name("in")
                .help("The csv file to read")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name(DIMENSIONAL_SEPARATOR)
                .short("d")
                .long(DIMENSIONAL_SEPARATOR)
                .value_name(DIMENSIONAL_SEPARATOR)
                .help("A separator to break header names allowing you to create deeper objects")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(NUMERIC_ARRAYS)
                .short("n")
                .long(NUMERIC_ARRAYS)
                .value_name(NUMERIC_ARRAYS)
                .help("Indicates the csv contains arrays represented by numeric keys. Use with -d")
                .takes_value(false),
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_has_the_correct_name() {
        let app = super::configure_app();
        assert_eq!(app.get_name(), "csv2json");
    }
}
