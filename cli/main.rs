use clap::{Arg, ArgAction, ArgMatches, Command, ValueHint};
use glyphrs_core::{fonts::font_handling::get_fonts, message_gen::convert_input};

// █▄█ ██▀ █   █▀▄ ██▀ █▀▄   █▀ █ █ █▄ █ ▄▀▀ ▀█▀ █ ▄▀▄ █▄ █ ▄▀▀
// █ █ █▄▄ █▄▄ █▀  █▄▄ █▀▄   █▀ ▀▄█ █ ▀█ ▀▄▄  █  █ ▀▄▀ █ ▀█ ▄██
fn string_to_str(input: Vec<String>) -> Vec<&'static str> {
    let mut output: Vec<&'static str> = Vec::with_capacity(input.len());
    for s in input {
        output.push(Box::leak(s.into_boxed_str()));
    }
    output
}

// █▀▄ ▄▀▄ █▀▄ ▄▀▀ ██▀   █ █▄ █ █▀▄ █ █ ▀█▀
// █▀  █▀█ █▀▄ ▄██ █▄▄   █ █ ▀█ █▀  ▀▄█  █
pub fn clap_parse() -> Command {
    let cmd = clap::Command::new("cargo")
        .bin_name("glyphrs")
        .before_help(
            "".to_owned()
                + "-----------------------------\n"
                + "▄▀  █   ▀▄▀ █▀▄ █▄█   █▀▄ ▄▀▀\n"
                + "▀▄█ █▄▄  █  █▀  █ █ ▄ █▀▄ ▄██\n"
                + "-----------------------------",
        )
        .about("A text art generator written in Rust")
        .arg(
            Arg::new("input")
                .required_unless_present("version")
                .help("the string to be converted")
                .value_hint(ValueHint::CommandString),
        )
        // ▄▀▄ █▀▄ ▄▀  ▄▀▀
        // █▀█ █▀▄ ▀▄█ ▄██
        .arg(
            Arg::new("font")
                .long("font")
                .short('f')
                .default_value("blocks_in_two_lines")
                .value_parser(string_to_str(get_fonts()))
                .help_heading("Output")
                .help("set the font")
                .value_hint(ValueHint::CommandString),
        )
        .arg(
            Arg::new("prefix")
                .long("prefix")
                .short('p')
                .default_value("")
                .help_heading("Output")
                .help("set the characters preceding each line, for example \"# \"")
                .value_hint(ValueHint::CommandString),
        )
        .arg(
            Arg::new("version")
                .long("version")
                .short('v')
                .action(ArgAction::SetTrue)
                .help("Print version information"),
        );

    cmd
}

// ▄▀▄ █▀▄ ▄▀    █▄█ ▄▀▄ █▄ █ █▀▄ █   █ █▄ █ ▄▀
// █▀█ █▀▄ ▀▄█   █ █ █▀█ █ ▀█ █▄▀ █▄▄ █ █ ▀█ ▀▄█
pub fn handle_args() -> ArgMatches {
    let matches: ArgMatches = clap_parse().get_matches();

    let version: bool = *matches.get_one("version").unwrap();
    if version {
        println!("glyphrs v{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    matches
}

fn main() {
    // █▄█ ▄▀▄ █▄ █ █▀▄ █   ██▀   ▄▀▄ █▀▄ ▄▀  ▄▀▀
    // █ █ █▀█ █ ▀█ █▄▀ █▄▄ █▄▄   █▀█ █▀▄ ▀▄█ ▄██
    let matches = handle_args();
    let input: String = matches.get_one::<String>("input").unwrap().to_string();
    let font: String = matches.get_one::<String>("font").unwrap().to_string();
    let prefix: String = matches.get_one::<String>("prefix").unwrap().to_string();

    // ▄▀  ██▀ █▄ █ ██▀ █▀▄ ▄▀▄ ▀█▀ ██▀   █▄ ▄█ ██▀ ▄▀▀ ▄▀▀ ▄▀▄ ▄▀  ██▀
    // ▀▄█ █▄▄ █ ▀█ █▄▄ █▀▄ █▀█  █  █▄▄   █ ▀ █ █▄▄ ▄██ ▄██ █▀█ ▀▄█ █▄▄
    let output = convert_input(input, font, prefix);

    // █▀▄ █▀▄ █ █▄ █ ▀█▀   ▄▀▄ █ █ ▀█▀ █▀▄ █ █ ▀█▀
    // █▀  █▀▄ █ █ ▀█  █    ▀▄▀ ▀▄█  █  █▀  ▀▄█  █
    for line in output {
        println!("{}", line);
    }
}
