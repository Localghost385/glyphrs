use clap::{value_parser, Arg, ArgAction, ArgMatches, Command, ValueHint};
use clap_complete::{generate, Generator, Shell};
use glyphrs_core::{fonts::font_handling::get_fonts, message_gen::convert_input};
use std::io;

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
                .required_unless_present_any(vec!["version", "generator"])
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
        )
        .arg(
            Arg::new("generator")
                .long("generate")
                .value_parser(value_parser!(Shell)),
        );

    cmd
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, "glyphrs".to_string(), &mut io::stdout());
}

fn main() {
    // █▄█ ▄▀▄ █▄ █ █▀▄ █   ██▀   ▄▀▄ █▀▄ ▄▀  ▄▀▀
    // █ █ █▀█ █ ▀█ █▄▀ █▄▄ █▄▄   █▀█ █▀▄ ▀▄█ ▄██
    let matches: ArgMatches = clap_parse().get_matches();

    let version: bool = *matches.get_one("version").unwrap();
    if version {
        println!("glyphrs v{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    
    if let Some(generator) = matches.get_one::<Shell>("generator") {
        let mut cmd = clap_parse();
        eprintln!("Generating completion file for {generator}...");
        print_completions(*generator, &mut cmd);
        std::process::exit(0);
    }

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
