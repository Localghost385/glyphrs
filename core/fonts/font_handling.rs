use serde::Deserialize;
use std::collections::HashMap;

// ▄▀▀ ▀█▀ █▀▄ █ █ ▄▀▀ ▀█▀ ▄▀▀   █▀ ▄▀▄ █▀▄   ▀█▀ ▄▀▄ █▄ ▄█ █     █▄█ ▄▀▄ █▄ █ █▀▄ █   █ █▄ █ ▄▀  
// ▄██  █  █▀▄ ▀▄█ ▀▄▄  █  ▄██   █▀ ▀▄▀ █▀▄    █  ▀▄▀ █ ▀ █ █▄▄   █ █ █▀█ █ ▀█ █▄▀ █▄▄ █ █ ▀█ ▀▄█ 
#[derive(Debug, Deserialize)]
struct Outer {
    fonts: Vec<Font>,
}

#[derive(Debug, Deserialize)]
struct Font {
    name: String,
    character: Vec<String>,
    key_values: Vec<Vec<String>>,
}

// ▄▀  ██▀ ▀█▀   ▄▀▄ █   █     █▀ ▄▀▄ █▄ █ ▀█▀ ▄▀▀ 
// ▀▄█ █▄▄  █    █▀█ █▄▄ █▄▄   █▀ ▀▄▀ █ ▀█  █  ▄██ 
pub fn get_fonts() -> Vec<String> {
    let fonts = define_fonts();
    let mut font_list: Vec<String> = vec![];

    for keys in fonts.keys().cloned() {
        font_list.push(keys);
    }

    // make sure the fonts are in alphabetical order
    font_list.sort();

    font_list
}

// ▄▀▀ ▄▀▄ █ █ █▀▄ ▄▀▀ ██▀   █▀ ▄▀▄ █▄ █ ▀█▀   █▀ █ █   ██▀ 
// ▄██ ▀▄▀ ▀▄█ █▀▄ ▀▄▄ █▄▄   █▀ ▀▄▀ █ ▀█  █    █▀ █ █▄▄ █▄▄ 
fn get_font_file() -> &'static str {
    let file = include_bytes!("./fonts.toml");
    let contents = std::str::from_utf8(file).expect("Failed to read config.toml");

    contents
}

// ▄▀  ██▀ ▀█▀   █▀ ▄▀▄ █▄ █ ▀█▀ ▄▀▀   █▀ █▀▄ ▄▀▄ █▄ ▄█   █▀ █ █   ██▀ 
// ▀▄█ █▄▄  █    █▀ ▀▄▀ █ ▀█  █  ▄██   █▀ █▀▄ ▀▄▀ █ ▀ █   █▀ █ █▄▄ █▄▄ 
pub fn define_fonts() -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut fonts: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let contents = get_font_file();
    let toml_value: Outer = toml::from_str(contents).expect("Failed to parse config.toml");

    for font in toml_value.fonts {
        let mut font_map: HashMap<String, Vec<String>> = HashMap::new();
        for i in 0..font.character.len() {
            let key = &font.character[i];
            let value = &font.key_values[i];
            font_map.insert(key.to_string(), value.to_vec());
        }

        fonts.insert(font.name, font_map);
    }
    fonts
}

// ▀█▀ ██▀ ▄▀▀ ▀█▀ ▄▀▀
//  █  █▄▄ ▄██  █  ▄██
#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::fonts::font_handling::{define_fonts, get_font_file, get_fonts, Outer};

    #[test]
    fn test_get_fonts() {
        let expected = vec!["blocks_in_two_lines".to_string(), "pipes".to_string()];
        assert_eq!(get_fonts(), expected);
    }

    #[test]
    fn test_define_fonts() {
        let contents = get_font_file();
        let toml_value: Outer = toml::from_str(contents).expect("Failed to parse config.toml");

        let mut expected: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
        for font in toml_value.fonts {
            let mut font_map: HashMap<String, Vec<String>> = HashMap::new();
            for i in 0..font.character.len() {
                let key = &font.character[i];
                let value = &font.key_values[i];
                font_map.insert(key.to_string(), value.to_vec());
            }

            expected.insert(font.name, font_map);
        }

        assert_eq!(define_fonts(), expected);
    }
}
