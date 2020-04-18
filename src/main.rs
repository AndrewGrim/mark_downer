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
                            ' ' => {
                                if heading_count > 6 {
                                    tokens.push(Token::new(TokenType::Error, c.0, c.0 + heading_count));
                                } else {
                                    tokens.push(Token::new(TokenType::Heading, c.0, c.0 + heading_count));
                                }
                            },
                            _ => break,
                        }
                    }
                }
                _ => (),
            },
            None => break,
        }
    }

    println!("tokens: {:?}", tokens);
    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_count() {
        let t = lex(fs::read_to_string("test/heading.md").unwrap()).unwrap();
        let mut headings: usize = 0;
        for token in t.iter() {
            if token.id == TokenType::Heading {
                headings += 1;
            } 
        }
        assert!(headings == 6);
    }
    
    #[test]
    #[should_panic]
    fn heading_count_fail() {
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().len() == 1);
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().len() == 10);
    }

    #[test]
    fn heading_begin() {
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().get(0).unwrap().begin == 0);
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().get(2).unwrap().begin == 25);
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().get(5).unwrap().begin == 70);
    }
    
    #[test]
    fn heading_end() {
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().get(0).unwrap().end == 1);
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().get(2).unwrap().end == 28);
        assert!(lex(fs::read_to_string("test/heading.md").unwrap()).unwrap().get(5).unwrap().end == 76);
    }
}
