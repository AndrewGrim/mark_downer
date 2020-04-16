#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs;
use std::io;

mod token;

fn main() {
    let file: String = match read_markdown_file(String::from("test/heading.md")) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    lex(file)
}

fn read_markdown_file(file_path: String) -> Result<String, io::Error> {
    let file: String = fs::read_to_string(file_path)?;

    Ok(file)
}

fn lex(text: String) {
    for c in text.chars().enumerate().peekable() {
        println!("char: {:?}", c);
    }
}
