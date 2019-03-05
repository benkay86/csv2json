use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, ArgMatches};

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
            Arg::with_name("dimensional-separator")
                .short("d")
                .long("dimensional-separator")
                .value_name("dimensional-separator")
                .help("A separator to break header names allowing you to create deeper objects")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("numeric-arrays")
                .short("n")
                .long("numeric-arrays")
                .value_name("numeric-arrays")
                .help("Indicates the csv contains arrays represented by numeric keys. Use with -d")
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
