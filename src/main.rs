use glyphrs::{clap, fonts::define_fonts};

fn sanitize_input(input: String) -> String {
    let mut output = input.clone();
    for characters in input.chars() {
        if characters.is_alphabetic() {
            output = output.to_lowercase();
        }
    }
    output
}

fn map_search(key: char, font: &str) -> Vec<&'static str> {
    let fonts = define_fonts();

    let map = fonts.get(font).unwrap().clone();

    let default: Vec<&str> = vec!["", ""];
    let value = map.get(&key).unwrap_or(&default).to_vec().clone();

    value
}

fn string_composite(characters: Vec<Vec<&str>>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for _ in 0..characters[0].len() {
        output.push(String::new());
    }

    for character in characters {
        for i in 0..character.len() {
            output[i] += character[i];
            output[i] += " ";
        }
    }

    output
}

fn main() {
    let matches = clap::handle_args();
    let input: String = matches.get_one::<String>("input").unwrap().to_string();
    let font: String = matches.get_one::<String>("font").unwrap().to_string();

    let input = sanitize_input(input);

    let mut map_values: Vec<Vec<&str>> = vec![];
    for c in input.chars() {
        map_values.push(map_search(c, &font));
    }
    let output: Vec<String> = string_composite(map_values);

    println!("{}", output[0]);
    println!("{}", output[1]);
}
