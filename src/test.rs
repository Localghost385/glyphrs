#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{fonts::get_fonts, message_gen::*};

    #[test]
    fn test_string_composite() {
        let expected_map: HashMap<&str, Vec<String>> = HashMap::from([(
            "blocks_in_two_lines",
            vec![
                "▀█▀ ██▀ ▄▀▀ ▀█▀   ▄▀▀ ▀█▀ █▀▄ █ █▄ █ ▄▀    ▄█ ▀█ ▀██ ".to_string(),
                " █  █▄▄ ▄██  █    ▄██  █  █▀▄ █ █ ▀█ ▀▄█    █ █▄ ▄▄█ ".to_string(),
            ],
        )]);

        for font in get_fonts() {
            let input = "test string 123";
            let mut map_values: Vec<Vec<&str>> = vec![];
            for c in input.chars() {
                map_values.push(map_search(c, &font));
            }
            let output: Vec<String> = string_composite(map_values);
            let expected: Vec<String> = expected_map.get(font).unwrap().clone();

            assert_eq!(output, expected);
        }
    }
}
