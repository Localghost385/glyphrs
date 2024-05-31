use glyphrs::{clap, message_gen::*};

fn main() {
    // █▄█ ▄▀▄ █▄ █ █▀▄ █   ██▀   ▄▀▄ █▀▄ ▄▀  ▄▀▀
    // █ █ █▀█ █ ▀█ █▄▀ █▄▄ █▄▄   █▀█ █▀▄ ▀▄█ ▄██
    let matches = clap::handle_args();
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
