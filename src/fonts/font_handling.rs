use std::collections::HashMap;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Outer {
    fonts: Vec<Font>,
}

#[derive(Debug, Deserialize)]
struct Font {
    name: String,
    character: Vec<char>,
    key_values: Vec<Vec<String>>,
}




pub fn get_fonts() -> Vec<String> {
    let fonts = define_fonts();
    let mut font_list: Vec<String> = vec![];

    for keys in fonts.keys().cloned() {
        font_list.push(keys);
    }

    font_list
}

fn get_font_file() -> &'static str {
    let file = include_bytes!("./fonts.toml");
    let contents = std::str::from_utf8(file).expect("Failed to read config.toml");

    contents
}

pub fn define_fonts() -> HashMap<String, HashMap<char, Vec<String>>> {
    let mut fonts: HashMap<String, HashMap<char, Vec<String>>> = HashMap::new();
    let contents = get_font_file();
    let toml_value: Outer =toml::from_str(&contents).expect("Failed to parse config.toml");

    for font in toml_value.fonts {
        let mut font_map: HashMap<char, Vec<String>> = HashMap::new();
        for i in 0..font.character.len() {
            let key = font.character[i];
            let value = &font.key_values[i];
            font_map.insert(key, value.to_vec());
        }

        fonts.insert(font.name, font_map);
    }
    fonts
}
