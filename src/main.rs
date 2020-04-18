#![allow(dead_code, unused_imports, unused_variables)]

use std::fs;
use std::io;
use std::io::Write;
use std::iter;
use std::str;

mod position;
use position::Position;

mod token;
use token::Token;
use token::TokenType;

fn main() {
    let test_files = [
        "heading",
        "checkbutton",
        "image",
    ];
    for test in test_files.iter() {
        let text: String = fs::read_to_string(format!("test/{}.md", test)).unwrap();
        let tokens = lex(&text).unwrap();
        let mut log = fs::File::create(format!("log/{}.log", test)).unwrap();
        log.write(format!("{:#?}", tokens).as_bytes()).unwrap();
        let html = parse(&text, &tokens);
        generate_html(format!("generated_html/{}.html", test), html);
    }
}

fn lex(text: &String) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::with_capacity(text.len());
    let mut iter = text.chars().enumerate().peekable();
    let mut pos: Position = Position::new(0, 0, 0);
    loop {
        match iter.next() {
            Some(c) => {
                pos.increment();
                match c.1 {
                    '#' => {
                        match_heading(&mut tokens, &mut iter, &mut pos, c);
                    },
                    '-' => {
                        match_checkbutton(text, &mut tokens, &mut iter, &mut pos, c);
                    },
                    '!' => {
                        match_image(text, &mut tokens, &mut iter, &mut pos, c);
                    },
                    '\n' => tokens.push(Token::new_single(TokenType::Newline, c.0)),
                    '\t' => tokens.push(Token::new_single(TokenType::Tab, c.0)),
                    ' ' => tokens.push(Token::new_single(TokenType::Space, c.0)),
                    _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
                }
            },
            None => break,
        }
    }

    Some(tokens)
}

fn match_heading(tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
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

fn match_checkbutton(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
    match text.get(c.0 + 1..c.0 + 6) {
        Some(v) => {
            if v == " [ ] " {
                tokens.push(Token::new(TokenType::Checkbutton(false), c.0, c.0 + 5));
                pos.index += 3;
                iter.nth(3);
            } else if v == " [x] " {
                tokens.push(Token::new(TokenType::Checkbutton(true), c.0, c.0 + 5));
                pos.index += 3;
                iter.nth(3);
            } else {
                tokens.push(Token::new_single(TokenType::Text, c.0));
            }
        },
        None => tokens.push(Token::new_single(TokenType::Text, c.0)),
    }
}

fn match_image(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
    match iter.peek() {
        Some(v) => {
            match v.1 {
                '[' => {
                    let alt_begin: usize = v.0 + 1;
                    iter.next();
                    pos.increment();
                    while let Some(v) = iter.next() {
                        pos.increment();
                        match v.1 {
                            ']' =>  {
                                let alt_end: usize = v.0;
                                match iter.peek() {
                                    Some(v) => {
                                        match v.1 {
                                            '(' => {
                                                let src_begin: usize = v.0 + 1;
                                                while let Some(v) = iter.next() {
                                                    pos.increment();
                                                    match v.1 {
                                                        ')' =>  {
                                                            tokens.push(Token::new(TokenType::ImageAlt, alt_begin, alt_end));
                                                            tokens.push(Token::new(TokenType::ImageSrc, src_begin, v.0));
                                                            break;
                                                        },
                                                        '\n' => {
                                                            tokens.push(Token::new(TokenType::Error, c.0, v.0));
                                                            break;
                                                        },
                                                        _ => (),
                                                    }
                                                }
                                            },
                                            _ => (),
                                        }
                                    },
                                    None => (),
                                }
                                break;
                            },
                            '\n' => {
                                tokens.push(Token::new(TokenType::Text, c.0, v.0));
                                tokens.push(Token::new_single(TokenType::Newline, v.0));
                                break;
                            },
                            _ => (),
                        }
                    }
                },
                _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
            }
        },
        None => tokens.push(Token::new_single(TokenType::Text, c.0)),
    }
}

fn parse(text: &String, tokens: &Vec<Token>) -> Vec<String> {
    let mut html: Vec<String> = Vec::with_capacity(text.len());
    let mut iter = tokens.iter().peekable();
    while let Some(t) = iter.next() {
        match t.id {
            TokenType::Heading => {
                let begin: usize = iter.next().unwrap().end;
                let mut end: usize = begin;
                while let Some(tok) = iter.peek() {
                    match tok.id {
                        TokenType::Text|TokenType::Space => {
                            end = tok.end;
                            iter.next();
                        },
                        _ => break,
                    }
                }
                html.push(format!("<h{}>{}</h{}>\n", t.end - t.begin, text[begin..end].to_string(), t.end - t.begin));
                match iter.peek() {
                    Some(n) => {
                        if n.id == TokenType::Newline {
                            iter.next();
                        }
                    },
                    None => (),
                }
            },
            TokenType::Checkbutton(bool) => {
                if t.id == TokenType::Checkbutton(true) {
                    html.push(format!("<input type=\"checkbox\" checked>"));
                } else {
                    html.push(format!("<input type=\"checkbox\">"));
                }
            },
            TokenType::ImageAlt => {
                html.push(format!("<img alt=\"{}\"", text[t.begin..t.end].to_string()));
                let t = iter.next().unwrap();
                html.push(format!(" src=\"{}\">", text[t.begin..t.end].to_string()));
            },
            TokenType::Error => html.push(format!("<div class=\"error\">ERROR: {}</div>\n", text[t.begin..t.end].to_string())),
            TokenType::Newline => html.push("<br>\n".to_string()),
            TokenType::Text => html.push(text[t.begin..t.end].to_string()),
            TokenType::Space => html.push(text[t.begin..t.end].to_string()),
            TokenType::Tab => html.push(text[t.begin..t.end].to_string()),
            TokenType::Whitespace(char) => html.push(text[t.begin..t.end].to_string()),
            _ => (),
        }
    }

    html
}

fn generate_html(output_file: String, html: Vec<String>) {
    let mut file = fs::File::create(output_file).unwrap();
    file.write("<link rel=\"stylesheet\" href=\"default.css\">\n".as_bytes()).unwrap();
    file.write("<div class=\"markdown-body\">\n".as_bytes()).unwrap();
    for tag in html.iter() {
        file.write(tag.as_bytes()).unwrap();
    }
    file.write("\n</div>".as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading() {
        let t = lex(&fs::read_to_string("test/heading.md").unwrap()).unwrap();
        let mut headings: usize = 0;
        let mut errors: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::Heading => {
                    headings += 1;
                },
                TokenType::Error => {
                    errors += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(headings == 6);
        assert!(errors == 1);
    }

    #[test]
    fn checkbutton() {
        let t = lex(&fs::read_to_string("test/checkbutton.md").unwrap()).unwrap();
        let mut checkbuttons: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::Checkbutton(bool) => {
                    checkbuttons += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(checkbuttons == 2);
    }

    #[test]
    fn image() {
        let t = lex(&fs::read_to_string("test/image.md").unwrap()).unwrap();
        let mut image_alt: usize = 0;
        let mut image_src: usize = 0;
        let mut errors: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::ImageAlt => {
                    image_alt += 1;
                },
                TokenType::ImageSrc => {
                    image_src += 1;
                },
                TokenType::Error => {
                    errors += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(image_alt == 2);
        assert!(image_src == 2);
        assert!(errors == 1);
    }
}
