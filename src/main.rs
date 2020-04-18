#![allow(dead_code, unused_imports, unused_variables)]

use std::fs;
use std::io;

mod position;
mod token;
use token::Token;
use token::TokenType;

fn main() {
    let file: String = match read_markdown_file(String::from("test/heading.md")) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    lex(file).unwrap();
}

fn read_markdown_file(file_path: String) -> Result<String, io::Error> {
    let file: String = fs::read_to_string(file_path)?;

    Ok(file)
}

fn lex(text: String) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::with_capacity(text.len());
    let mut iter = text.chars().enumerate().peekable();
    loop {
        match iter.next() {
            Some(c) => match c.1 {
                '#' => {
                    let mut heading_count: usize = 1;
                    while let Some(v) = iter.next() {
                        match v.1 {
                            '#' => {
                                heading_count += 1;
                            }
                            ' ' => tokens.push(Token::new(TokenType::Heading, c.0, c.0 + heading_count)),
                            _ => break,
                        }
                    }
                }
                _ => (),
            },
            None => break,
        }
    }

    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_count() {
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().len() == 6);
    }

    #[test]
    #[should_panic]
    fn heading_count_fail() {
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().len() == 5);
    }

}
