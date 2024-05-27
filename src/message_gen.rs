use crate::fonts::font_handling::define_fonts;

pub fn sanitize_input(input: String) -> String {
    let mut output = input.clone();
    for characters in input.chars() {
        if characters.is_alphabetic() {
            output = output.to_lowercase();
        }
    }
    output
}

pub fn map_search(key: char, font: &str) -> Vec<String> {
    let fonts = define_fonts();

    let map = fonts.get(font).unwrap().clone();

    let default: Vec<String> = vec!["".to_string(), "".to_string()];
    let value = map.get(&key).unwrap_or(&default).to_vec().clone();

    value
}

pub fn string_composite(characters: Vec<Vec<String>>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for _ in 0..characters[0].len() {
        output.push(String::new());
    }

    for character in characters {
        for i in 0..character.len() {
            output[i] += &character[i];
            output[i] += " ";
        }
    }

    output
}

pub fn convert_input(mut input: String, font: String) -> Vec<String> {
    input = sanitize_input(input);

    let mut map_values: Vec<Vec<String>> = vec![];
    for c in input.chars() {
        map_values.push(map_search(c, &font));
    }
    let output: Vec<String> = string_composite(map_values);

    output
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;
    use crate::{fonts::font_handling::get_fonts, message_gen::*};

    #[test]
    fn test_string_composite() {
        let expected_map: HashMap<String, Vec<String>> = HashMap::from([(
            "blocks_in_two_lines".to_string(),
            vec![
                "▀█▀ ██▀ ▄▀▀ ▀█▀   ▄▀▀ ▀█▀ █▀▄ █ █▄ █ ▄▀    ▄█ ▀█ ▀██ ".to_string(),
                " █  █▄▄ ▄██  █    ▄██  █  █▀▄ █ █ ▀█ ▀▄█    █ █▄ ▄▄█ ".to_string(),
            ],
        )]);

        for font in get_fonts() {
            let input = "test string 123";
            let mut map_values: Vec<Vec<String>> = vec![];
            for c in input.chars() {
                map_values.push(map_search(c, &font.to_string()));
            }
            let output: Vec<String> = string_composite(map_values);
            let expected: Vec<String> = expected_map.get(&font).unwrap().clone();

            assert_eq!(output, expected);
        }
    }
}
