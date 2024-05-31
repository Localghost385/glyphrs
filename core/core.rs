use glyphrs::message_gen::convert_input;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn glyphrs_wrapper(content: String, font: String, prefix: String) -> String {
    let raw = convert_input(content, font, prefix);
    let mut processed = "".to_string();
    for line in raw {
        processed += &(line + "\n");
    }

    processed
}