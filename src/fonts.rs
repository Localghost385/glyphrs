use std::{collections::HashMap, vec};

pub fn get_fonts() -> Vec<&'static str> {
    let fonts = define_fonts();

    fonts.keys().cloned().collect()
}

pub fn define_fonts() -> HashMap<&'static str, HashMap<char, Vec<&'static str>>> {
    //██▄ █   ▄▀▄ ▄▀▀ █▄▀ ▄▀▀   █ █▄ █   ▀█▀ █   █ ▄▀▄   █   █ █▄ █ ██▀ ▄▀▀
    //█▄█ █▄▄ ▀▄▀ ▀▄▄ █ █ ▄██   █ █ ▀█    █  ▀▄▀▄▀ ▀▄▀   █▄▄ █ █ ▀█ █▄▄ ▄██
    let blocks_in_two_lines: HashMap<char, Vec<&str>> = vec![
        (' ', vec![" ", " "]),
        ('a', vec!["▄▀▄", "█▀█"]),
        ('b', vec!["██▄", "█▄█"]),
        ('c', vec!["▄▀▀", "▀▄▄"]),
        ('d', vec!["█▀▄", "█▄▀"]),
        ('e', vec!["██▀", "█▄▄"]),
        ('f', vec!["█▀", "█▀"]),
        ('g', vec!["▄▀ ", "▀▄█"]),
        ('h', vec!["█▄█", "█ █"]),
        ('i', vec!["█", "█"]),
        ('j', vec!["  █", "▀▄█"]),
        ('k', vec!["█▄▀", "█ █"]),
        ('l', vec!["█  ", "█▄▄"]),
        ('m', vec!["█▄ ▄█", "█ ▀ █"]),
        ('n', vec!["█▄ █", "█ ▀█"]),
        ('o', vec!["▄▀▄", "▀▄▀"]),
        ('p', vec!["█▀▄", "█▀ "]),
        ('q', vec!["▄▀▄", "▀▄█"]),
        ('r', vec!["█▀▄", "█▀▄"]),
        ('s', vec!["▄▀▀", "▄██"]),
        ('t', vec!["▀█▀", " █ "]),
        ('u', vec!["█ █", "▀▄█"]),
        ('v', vec!["█ █", "▀▄▀"]),
        ('w', vec!["█   █", "▀▄▀▄▀"]),
        ('x', vec!["▀▄▀", "█ █"]),
        ('y', vec!["▀▄▀", " █ "]),
        ('z', vec!["▀█▀", "█▄▄"]),
        ('1', vec!["▄█", " █"]),
        ('2', vec!["▀█", "█▄"]),
        ('3', vec!["▀██", "▄▄█"]),
        ('4', vec!["█▄", " █"]),
        ('5', vec!["█▄", "▄█"]),
        ('6', vec!["█▀", "██"]),
        ('7', vec!["▀█", " █"]),
        ('8', vec!["█▄█", "█▄█"]),
        ('9', vec!["██", "▄█"]),
        ('0', vec!["█▀█", "█▄█"]),
        ('.', vec![" ", "▄"]),
        ('!', vec!["█", "▄"]),
        ('\'', vec!["▀", " "]),
        ('\"', vec!["▀ ▀", "   "]),
        ('(', vec!["▄▀", "▀▄"]),
        (')', vec!["▀▄", "▄▀"]),
    ]
    .into_iter()
    .collect();

    let fonts: HashMap<&str, HashMap<char, Vec<&str>>> =
        vec![("blocks_in_two_lines", blocks_in_two_lines)]
            .into_iter()
            .collect();

    fonts
}
