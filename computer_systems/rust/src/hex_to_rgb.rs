use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub fn hex_to_rgb(mut file: File) -> Vec<Vec<u32>> {
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})").unwrap();

    // grab only matched group in regex

    let mut hex_values = Vec::new();

    for hex in re.captures_iter(&contents) {
        hex_values.push(hex.get(1).unwrap().as_str());
    }

    let mut rgb_values = Vec::new();

    for hex in hex_values {
        let mut rgb = Vec::new();
        let hex_length = hex.len();

        let mut hex_char = hex.chars();

        if hex_length == 4 {
            hex_char.next();
        }

        // go through each "pair" of hex values and push them to rgb vector
        for _ in 0..3 {
            let first = hex_char.next().unwrap();
            let second = hex_char.next().unwrap();

            let mut first = match first {
                'a' => 10,
                'b' => 11,
                'c' => 12,
                'd' => 13,
                'e' => 14,
                'f' => 15,
                _ => first.to_digit(16).unwrap(),
            };

            let second = match second {
                'a' => 10,
                'b' => 11,
                'c' => 12,
                'd' => 13,
                'e' => 14,
                'f' => 15,
                _ => second.to_digit(16).unwrap(),
            };

            // shifting by 4 to move it to the higher order bits
            first = first << 4;

            rgb.push(first + second);
        }

        rgb_values.push(rgb);
    }
    
    return rgb_values;
}
