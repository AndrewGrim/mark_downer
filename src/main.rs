#![allow(dead_code, unused_imports, unused_variables)]

use std::fs;
use std::io;
use std::iter;
use std::str;

mod position;
use position::Position;

mod token;
use token::Token;
use token::TokenType;

fn main() {
    let file: String = match read_markdown_file(String::from("test/heading.md")) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let tokens = lex(&file).unwrap();
    parse(&file, &tokens);
}

fn read_markdown_file(file_path: String) -> Result<String, io::Error> {
    let file: String = fs::read_to_string(file_path)?;

    Ok(file)
}

fn lex(text: &String) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::with_capacity(text.len());
    let mut iter = text.chars().enumerate();
    let mut pos: Position = Position::new(0, 0, 0);
    loop {
        match iter.next() {
            Some(c) => {
                pos.increment();
                match c.1 {
                    '#' => {
                        match_heading(&mut tokens, &mut iter, &mut pos, c);
                    }
                    _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
                }
            },
            None => break,
        }
    }

    // for t in tokens.iter() {
    //     println!("{:?}", t);
    // }
    Some(tokens)
}

fn match_heading(tokens: &mut Vec<Token>, iter: &mut iter::Enumerate<str::Chars>, pos: &mut Position, c: (usize, char)) {
    let mut heading_count: usize = 1;
    while let Some(v) = iter.next() {
        pos.increment();
        match v.1 {
            '#' => {
                heading_count += 1;
            },
            ' ' => {
                if heading_count > 6 {
                    tokens.push(Token::new(TokenType::Error, c.0, c.0 + heading_count));
                } else {
                    tokens.push(Token::new(TokenType::Heading, c.0, c.0 + heading_count));
                }
                tokens.push(Token::new_single(TokenType::Space, c.0 + heading_count));
                break;
            },
            _ =>  {
                loop  {
                    match iter.next() {
                        Some(v) => {
                            pos.increment();
                            match v.1 {
                                ' '|'\t'|'\n' => {
                                    tokens.push(Token::new(TokenType::Text, c.0, v.0));
                                    tokens.push(Token::new_single(TokenType::Whitespace(v.1), v.0));
                                    break;
                                },
                                _ => (),
                            }
                        },
                        None => {
                            tokens.push(Token::new(TokenType::Text, c.0, pos.index));
                            break;
                        },
                    }
                }
                break;
            },
        }
    }
}

fn parse(file: &String, tokens: &Vec<Token>) {
    let mut iter = tokens.iter().peekable();
    while let Some(t) = iter.next() {
        match t.id {
            TokenType::Heading => {
                let begin: usize = iter.next().unwrap().end;
                let mut end: usize = begin;
                while let Some(tok) = iter.peek() {
                    match tok.id {
                        TokenType::Text => {
                            end = tok.end;
                            iter.next();
                        },
                        _ => break,
                    }
                }
                println!("<h{}>{}</h{}>", t.end - t.begin, file[begin..end - 1].to_string(), t.end - t.begin);
            },
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_count() {
        let t = lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap();
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
        assert!(lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap().len() == 1);
        assert!(lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap().len() == 10);
    }

    #[test]
    fn heading_begin() {
        assert!(lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap().get(0).unwrap().begin == 0);
        assert!(lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap().get(2).unwrap().begin == 25);
        assert!(lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap().get(5).unwrap().begin == 70);
    }
    
    #[test]
    fn heading_end() {
        assert!(lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap().get(0).unwrap().end == 1);
        assert!(lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap().get(2).unwrap().end == 28);
        assert!(lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap().get(5).unwrap().end == 76);
    }
}
