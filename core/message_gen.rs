use crate::fonts::font_handling::define_fonts;
use wasm_bindgen::prelude::*;

// ▄▀▀ ▄▀▄ █▄ █ █ █ ██▀ █▀▄ ▀█▀   ▀█▀ ▄▀▄   █   ▄▀▄ █   █ ██▀ █▀▄ ▄▀▀ ▄▀▄ ▄▀▀ ██▀
// ▀▄▄ ▀▄▀ █ ▀█ ▀▄▀ █▄▄ █▀▄  █     █  ▀▄▀   █▄▄ ▀▄▀ ▀▄▀▄▀ █▄▄ █▀▄ ▀▄▄ █▀█ ▄██ █▄▄
fn sanitize_input(input: String) -> String {
    let mut output = input.clone();
    for characters in input.chars() {
        if characters.is_alphabetic() {
            output = output.to_lowercase();
        }
    }
    output
}

// ▄▀  ██▀ ▀█▀   ▄▀▀ █▄█ ▄▀▄ █▀▄ ▄▀▄ ▄▀▀ ▀█▀ ██▀ █▀▄   █▀ █▀▄ ▄▀▄ █▄ ▄█   █▄ ▄█ ▄▀▄ █▀▄
// ▀▄█ █▄▄  █    ▀▄▄ █ █ █▀█ █▀▄ █▀█ ▀▄▄  █  █▄▄ █▀▄   █▀ █▀▄ ▀▄▀ █ ▀ █   █ ▀ █ █▀█ █▀
fn map_search(key: String, font: &str) -> Vec<String> {
    let fonts = define_fonts();

    let map = fonts.get(font).unwrap().clone();

    let default: Vec<String> = vec!["".to_string(), "".to_string()];
    let value = map.get(&key).unwrap_or(&default).to_vec().clone();

    value
}

// ▄▀▀ ▄▀▄ █▄ ▄█ ██▄ █ █▄ █ ██▀   ▄▀▀ █▄█ ▄▀▄ █▀▄ ▄▀▄ ▄▀▀ ▀█▀ ██▀ █▀▄ ▄▀▀
// ▀▄▄ ▀▄▀ █ ▀ █ █▄█ █ █ ▀█ █▄▄   ▀▄▄ █ █ █▀█ █▀▄ █▀█ ▀▄▄  █  █▄▄ █▀▄ ▄██
fn string_composite(characters: Vec<Vec<String>>, prefix: String) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for _ in 0..characters[0].len() {
        output.push(String::new());
    }

    for line in output.iter_mut() {
        line.clone_from(&prefix);
    }

    for character in characters {
        for i in 0..character.len() {
            output[i] += &character[i];
        }
    }

    remove_blank_lines(output)
}

// ▄▀▀ ▄▀▄ █▄ █ █▀▄ ██▀ █▄ █ ▄▀▀ ██▀   ▄▀▄ █ █ ▀█▀ █▀▄ █ █ ▀█▀
// ▀▄▄ ▀▄▀ █ ▀█ █▄▀ █▄▄ █ ▀█ ▄██ █▄▄   ▀▄▀ ▀▄█  █  █▀  ▀▄█  █
fn remove_blank_lines(input: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for line in input {
        // if every character is a space, then remove the line
        if line.chars().all(|c| c == ' ') {
            continue;
        } else {
            output.push(line);
        }
    }

    output
}

// ▄▀▀ ▀█▀ █▀▄ █ █▄ █ ▄▀    ▀█▀ ▄▀▄ ▄▀  ██▀ ▀█▀ █▄█ ██▀ █▀▄   ▀█▀ █▄█ ██▀   ▄▀▄ ██▄ ▄▀▄ █ █ ██▀
// ▄██  █  █▀▄ █ █ ▀█ ▀▄█    █  ▀▄▀ ▀▄█ █▄▄  █  █ █ █▄▄ █▀▄    █  █ █ █▄▄   █▀█ █▄█ ▀▄▀ ▀▄▀ █▄▄
#[wasm_bindgen]
pub fn convert_input(mut input: String, font: String, prefix: String) -> Vec<String> {
    input = sanitize_input(input);

    let mut map_values: Vec<Vec<String>> = vec![];
    for c in input.chars() {
        map_values.push(map_search(c.to_string(), &font));
    }
    let output: Vec<String> = string_composite(map_values, prefix);

    output
}

// ▀█▀ ██▀ ▄▀▀ ▀█▀ ▄▀▀
//  █  █▄▄ ▄██  █  ▄██
#[cfg(test)]
pub mod tests {
    use crate::{fonts::font_handling::get_fonts, message_gen::*};
    use std::collections::HashMap;

    #[test]
    fn test_remove_blank_lines() {
        let input: Vec<String> = vec![
            " ╔╗          ╔╗ ".to_string(),
            "╔╝╚╗        ╔╝╚╗".to_string(),
            "╚╗╔╝╔══╗╔══╗╚╗╔╝".to_string(),
            " ║║ ║╔╗║║══╣ ║║ ".to_string(),
            " ║╚╗║║═╣╠══║ ║╚╗".to_string(),
            " ╚═╝╚══╝╚══╝ ╚═╝".to_string(),
            "                ".to_string(),
            "                ".to_string(),
        ];
        let expected: Vec<String> = vec![
            " ╔╗          ╔╗ ".to_string(),
            "╔╝╚╗        ╔╝╚╗".to_string(),
            "╚╗╔╝╔══╗╔══╗╚╗╔╝".to_string(),
            " ║║ ║╔╗║║══╣ ║║ ".to_string(),
            " ║╚╗║║═╣╠══║ ║╚╗".to_string(),
            " ╚═╝╚══╝╚══╝ ╚═╝".to_string(),
        ];
        let output = remove_blank_lines(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_prefix() {
        let input = "abc";
        let mut map_values: Vec<Vec<String>> = vec![];
        for c in input.chars() {
            map_values.push(map_search(c.to_string(), "blocks_in_two_lines"));
        }

        assert_eq!(
            string_composite(map_values.clone(), "".to_string()),
            vec!["▄▀▄ ██▄ ▄▀▀ ".to_string(), "█▀█ █▄█ ▀▄▄ ".to_string()]
        );
        assert_eq!(
            string_composite(map_values.clone(), "# ".to_string()),
            vec!["# ▄▀▄ ██▄ ▄▀▀ ".to_string(), "# █▀█ █▄█ ▀▄▄ ".to_string()]
        );
    }

    #[test]
    fn test_message_gen() {
        let expected_map: HashMap<String, Vec<String>> = HashMap::from([
            (
                "blocks_in_two_lines".to_string(),
                vec![
                    "▄▀▄ ██▄ ▄▀▀ █▀▄ ██▀ █▀ ▄▀  █▄█ █   █ █▄▀ █   █▄ ▄█ █▄ █ ▄▀▄ █▀▄ ▄▀▄ █▀▄ ▄▀▀ ▀█▀ █ █ █ █ █   █ ▀▄▀ ▀▄▀ ▀█▀ ▄█ ▀█ ▀██ █▄ █▄ █▀ ▀█ █▄█ ██ █▀█   █ ▀ ▀ ▀ ▄▀ ▀▄ ".to_string(),
                    "█▀█ █▄█ ▀▄▄ █▄▀ █▄▄ █▀ ▀▄█ █ █ █ ▀▄█ █ █ █▄▄ █ ▀ █ █ ▀█ ▀▄▀ █▀  ▀▄█ █▀▄ ▄██  █  ▀▄█ ▀▄▀ ▀▄▀▄▀ █ █  █  █▄▄  █ █▄ ▄▄█  █ ▄█ ██  █ █▄█ ▄█ █▄█ ▄ ▄       ▀▄ ▄▀ ".to_string(),
                ],
            ),
            (
                "pipes".to_string(),
                vec![
                    "     ╔╗        ╔╗     ╔═╗    ╔╗       ╔╗  ╔╗                             ╔╗                              ╔╗ ╔═══╗╔═══╗╔╗ ╔╗╔═══╗╔═══╗╔═══╗╔═══╗╔═══╗╔═══╗  ╔╗╔╗╔╗╔╗  ╔═╗╔═╗  ".to_string(),
                    "     ║║        ║║     ║╔╝    ║║     ╔╗║║  ║║                            ╔╝╚╗                            ╔╝║ ║╔═╗║║╔═╗║║║ ║║║╔══╝║╔══╝║╔═╗║║╔═╗║║╔═╗║║╔═╗║  ║║║║║║║║ ╔╝╔╝╚╗╚╗ ".to_string(),
                    "╔══╗ ║╚═╗╔══╗╔═╝║╔══╗╔╝╚╗╔══╗║╚═╗╔╗ ╚╝║║╔╗║║ ╔╗╔╗╔═╗ ╔══╗╔══╗╔══╗╔═╗╔══╗╚╗╔╝╔╗╔╗╔╗╔╗╔╗╔╗╔╗╔╗╔╗╔╗ ╔╗╔═══╗╚╗║ ╚╝╔╝║╚╝╔╝║║╚═╝║║╚══╗║╚══╗╚╝╔╝║║╚═╝║║╚═╝║║║ ║║  ║║╚╝╚╝╚╝╔╝╔╝  ╚╗╚╗".to_string(),
                    "╚ ╗║ ║╔╗║║╔═╝║╔╗║║╔╗║╚╗╔╝║╔╗║║╔╗║╠╣ ╔╗║╚╝╝║║ ║╚╝║║╔╗╗║╔╗║║╔╗║║╔╗║║╔╝║══╣ ║║ ║║║║║╚╝║║╚╝╚╝║╚╬╬╝║║ ║║╠══║║ ║║ ╔═╝╔╝╔╗╚╗║╚══╗║╚══╗║║╔═╗║  ║╔╝║╔═╗║╚══╗║║║ ║║  ╚╝      ║║║    ║║║".to_string(),
                    "║╚╝╚╗║╚╝║║╚═╗║╚╝║║║═╣ ║║ ║╚╝║║║║║║║ ║║║╔╗╗║╚╗║║║║║║║║║╚╝║║╚╝║║╚╝║║║ ╠══║ ║╚╗║╚╝║╚╗╔╝╚╗╔╗╔╝╔╬╬╗║╚═╝║║║══╣╔╝╚╗║║╚═╗║╚═╝║   ║║╔══╝║║╚═╝║  ║║ ║╚═╝║╔══╝║║╚═╝║╔╗╔╗      ║║║    ║║║".to_string(),
                    "╚═══╝╚══╝╚══╝╚══╝╚══╝ ╚╝ ╚═╗║╚╝╚╝╚╝ ║║╚╝╚╝╚═╝╚╩╩╝╚╝╚╝╚══╝║╔═╝╚═╗║╚╝ ╚══╝ ╚═╝╚══╝ ╚╝  ╚╝╚╝ ╚╝╚╝╚═╗╔╝╚═══╝╚══╝╚═══╝╚═══╝   ╚╝╚═══╝╚═══╝  ╚╝ ╚═══╝╚═══╝╚═══╝╚╝╚╝      ╚╗╚╗  ╔╝╔╝".to_string(),
                    "                         ╔═╝║      ╔╝║                   ║║    ║║                             ╔═╝║                                                                  ╚╗╚╗╔╝╔╝ ".to_string(),
                    "                         ╚══╝      ╚═╝                   ╚╝    ╚╝                             ╚══╝                                                                   ╚═╝╚═╝  ".to_string(),
                ],
            ),
        ]);

        for font in get_fonts() {
            let input = "abcdefghijklmnopqrstuvwxyz1234567890.!\"\'()";
            let mut map_values: Vec<Vec<String>> = vec![];
            for c in input.chars() {
                map_values.push(map_search(c.to_string(), &font.to_string()));
            }
            let output: Vec<String> = string_composite(map_values, "".to_string());
            let expected: Vec<String> = expected_map.get(&font).unwrap().clone();

            assert_eq!(output, expected);
        }
    }
}
