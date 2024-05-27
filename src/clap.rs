use clap::*;

use crate::fonts::define_fonts;

fn get_fonts() -> Vec<&'static str> {
    let fonts = define_fonts();

    fonts.keys().cloned().collect()
}

pub fn clap_parse() -> ArgMatches {
    let cmd = clap::Command::new("cargo")
        .bin_name("glyphrs")
		.before_help("-----------------------------\n▄▀  █   ▀▄▀ █▀▄ █▄█   █▀▄ ▄▀▀\n▀▄█ █▄▄  █  █▀  █ █ ▄ █▀▄ ▄██ \n-----------------------------")
        .about("A text art generator written in Rust")
		.arg(
			Arg::new("input")
            .required_unless_present("version")
            .help("the string to be converted"),
		)
        .arg(
            Arg::new("font")
            .long("font")
            .short('f')
            .default_value("blocks_in_two_lines")
            .value_parser(get_fonts())
            .help("set the font"),

        )
        .arg(
            Arg::new("version")
            .long("version")
            .short('v')
            .action(ArgAction::SetTrue)
            .help("Print version information"),
        );

    cmd.get_matches()
}

pub fn handle_args() -> ArgMatches {
    let matches: ArgMatches = clap_parse();
    let version: bool = *matches.get_one("version").unwrap();
    if version {
        println!("glyphrs v{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    matches
}
