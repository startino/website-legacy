use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};

fn hex_to_rgb(hex: &str) -> String {
    let mut hex = hex.to_string();
    hex.remove(0);
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    format!("{} {} {}", r, g, b)
}

fn replace_hex_rgb(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let re = Regex::new(r"#(?:[a-fA-F\d]{3}){1,2}").unwrap();

    let mut output = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let replaced = re.replace_all(&line, |caps: &regex::Captures| {
            hex_to_rgb(caps.get(0).unwrap().as_str())
        });
        output.push_str(&replaced);
        output.push('\n');
    }
    let mut file = BufWriter::new(File::create(file_path).unwrap());
    write!(file, "{}", output).unwrap();
}

fn main() {
    let file_path = "styles/tailwind.css.txt";
    replace_hex_rgb(file_path);
}
