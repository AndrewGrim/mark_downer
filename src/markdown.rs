use std::iter;
use std::str;

use crate::position::Position;
use crate::token::Token;
use crate::token::TokenType;
use crate::emphasis::Tag;
use crate::emphasis::State;

pub fn match_heading(tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
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
                // TODO The loop below is likely unnecessary
                // instead if we hit anything else then break and push a text token
                // and when detecting headings just check if the previous character 
                // is a whitespace.
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

pub fn match_checkbutton(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
    match text.get(c.0 + 2..c.0 + 6) {
        Some(v) => {
            if v == "[ ] " {
                tokens.push(Token::new(TokenType::Checkbutton(false), c.0, c.0 + 5));
                pos.index += 3;
                iter.nth(3);
            } else if v == "[x] " {
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

pub fn match_image(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
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
                                            _ => tokens.push(Token::new(TokenType::Text, c.0, v.0)),
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

pub fn match_link(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
    let text_begin: usize = c.0 + 1;
    loop {
        match iter.next() {
            Some(v) => {
                pos.increment();
                match v.1 {
                    ']' =>  {
                        let text_end: usize = v.0;
                        match iter.peek() {
                            Some(v) => {
                                match v.1 {
                                    '(' => {
                                        let href_begin: usize = v.0 + 1;
                                        while let Some(v) = iter.next() {
                                            pos.increment();
                                            match v.1 {
                                                ')' =>  {
                                                    tokens.push(Token::new(TokenType::LinkHref, href_begin, v.0));
                                                    tokens.push(Token::new(TokenType::LinkText, text_begin, text_end));
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
                                    _ => tokens.push(Token::new(TokenType::Text, c.0, v.0)),
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
            },
            None => {
                tokens.push(Token::new_single(TokenType::Text, c.0));
                break;
            }
        }
    }
}

pub fn match_horizontalrule(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
    iter.next();
    pos.increment();
    match iter.next() {
        Some(v) => {
            pos.increment();
            match v.1 {
                '-' => {
                    match iter.peek() {
                        Some(v) => {
                            match v.1 {
                                '\n' => {
                                    if c.0 == 0 || &text[c.0 - 1..c.0] == "\n" {
                                        tokens.push(Token::new(TokenType::HorizontalRule, c.0, v.0 + 1));
                                    } else {
                                        tokens.push(Token::new(TokenType::Text, c.0, v.0));
                                        tokens.push(Token::new_single(TokenType::Newline, v.0));
                                    }
                                },
                                _ => tokens.push(Token::new(TokenType::Text, c.0, v.0 + 1)),
                            }
                            iter.next();
                            pos.increment();
                        },
                        None => tokens.push(Token::new(TokenType::Text, c.0, v.0)),
                    }
                },
                _ => tokens.push(Token::new(TokenType::Text, c.0, v.0 + 1)),
            }
        },
        None => tokens.push(Token::new_double(TokenType::Text, c.0)),
    }
}

pub fn match_blockquote(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
    if c.0 == 0 || &text[c.0 - 1..c.0] == "\n" {
        tokens.push(Token::new_single(TokenType::BlockquoteBegin, c.0));
        loop {
            match iter.next() {
                Some(v) => {
                    pos.increment();
                    match v.1 {
                        '\n' => {
                            match iter.peek() {
                                Some(v) => {
                                    match v.1 {
                                        '\n' => {
                                            tokens.push(Token::new(TokenType::BlockquoteEnd, c.0, pos.index));
                                            tokens.push(Token::new_single(TokenType::Newline, v.0));
                                            break;
                                        },
                                        _ => tokens.push(Token::new_single(TokenType::Text, v.0)),
                                    }
                                    iter.next();
                                    pos.increment();
                                },
                                None => tokens.push(Token::new(TokenType::BlockquoteEnd, c.0, pos.index)),
                            }
                        },
                        _ => tokens.push(Token::new_single(TokenType::Text, v.0)),
                    }
                },
                None => {
                    tokens.push(Token::new(TokenType::BlockquoteEnd, c.0, pos.index));
                    break;
                },
            }
        }
    } else  {
        tokens.push(Token::new_single(TokenType::Text, c.0));
    }
}

pub fn match_code(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
    if c.0 == 0 || &text[c.0 - 1..c.0] != "`" {
        loop {
            match iter.next() {
                Some(v) => {
                    pos.increment();
                    match v.1 {
                        '`' => {
                            tokens.push(Token::new(TokenType::Code, c.0 + 1, v.0));
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
    } else {
        tokens.push(Token::new_single(TokenType::Text, c.0));
    }
}

pub fn match_codeblock(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
    // TODO Perhaps when looking for closing backticks also check if the following characters is a newline.
    // And only then push a closing token.
    if c.0 == 0 || &text[c.0 - 1..c.0] == "\n" {
        iter.next();
        pos.increment();
        match iter.peek() {
            Some(v) => {
                let v = iter.next().unwrap();
                pos.increment();
                match v.1 {
                    '`' =>{
                        tokens.push(Token::new(TokenType::CodeBlockBegin, c.0, pos.index));
                        let lang_begin: usize = pos.index;
                        loop {
                            match iter.next() {
                                Some(v) => {
                                    pos.increment();
                                    match v.1 {
                                        '\n' => {
                                            tokens.push(Token::new(TokenType::CodeBlockLanguage, lang_begin, pos.index));
                                            break;
                                        },
                                        _ => (),
                                    }
                                },
                                None => break,
                            }
                        }
                        let lang_end: usize = pos.index;
                        loop {
                            match iter.next() {
                                Some(v) => {
                                    pos.increment();
                                    match v.1 {
                                        '`' => {
                                            match iter.next() {
                                                Some(v) => {
                                                    pos.increment();
                                                    match v.1 {
                                                        '`' => {
                                                            match iter.next() {
                                                                Some(v) => {
                                                                    pos.increment();
                                                                    match v.1 {
                                                                        '`' => {
                                                                            tokens.push(Token::new(TokenType::CodeBlockEnd, lang_end, v.0));
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
                                                        },
                                                        _ => (),
                                                    }
                                                },
                                                None => {
                                                    tokens.push(Token::new(TokenType::Text, c.0, pos.index));
                                                    break;
                                                },
                                            }
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
                    },
                    _ => tokens.push(Token::new(TokenType::Text, c.0, v.0)),
                }
            },
            None => tokens.push(Token::new(TokenType::Text, c.0, pos.index)),
        }
    } else {
        tokens.push(Token::new_single(TokenType::Text, c.0));
    }
}

pub fn match_indentblock(text: &String, mut tokens: &mut Vec<Token>, mut iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, mut pos: &mut Position, c: (usize, char)) {
    iter.next();
    pos.increment();
    if match_string(String::from("  "), text, &mut tokens, &mut iter, &mut pos, c) {
        loop {
            match iter.next() {
                Some(v) => {
                    pos.increment();
                    match v.1 {
                        '\n' => {
                            if !match_string(String::from("    "), text, &mut tokens, &mut iter, &mut pos, c) {
                                tokens.push(Token::new(TokenType::IndentBlock, c.0, pos.index - 1));
                                tokens.push(Token::new_single(TokenType::Text, pos.index - 1));
                                break;
                            } 
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
    }
}

pub fn match_emphasis(mut state: &mut State, text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
    match c.1 {
        '*' => {
            match iter.peek() {
                Some(v) => {
                    match v.1 {
                        '*' => {
                            iter.next();
                            pos.increment();
                            if state.bold == Tag::Bold(false) {
                                tokens.push(Token::new_double(TokenType::BoldBegin, c.0));
                                state.bold = Tag::Bold(true);
                            } else {
                                tokens.push(Token::new_double(TokenType::BoldEnd, c.0));
                                state.bold = Tag::Bold(false);
                            }
                        },
                        _ => {
                            if state.italic == Tag::Italic(false) {
                                tokens.push(Token::new_single(TokenType::ItalicBegin, c.0));
                                state.italic = Tag::Italic(true);
                            } else {
                                tokens.push(Token::new_single(TokenType::ItalicEnd, c.0));
                                state.italic = Tag::Italic(false);
                            }
                        },
                    }
                },
                None => tokens.push(Token::new_single(TokenType::Text, c.0)),
            }
        },
        '~' => {
            match iter.peek() {
                Some(v) => {
                    match v.1 {
                        '~' => {
                            iter.next();
                            pos.increment();
                            if state.strike == Tag::Strike(false) {
                                tokens.push(Token::new_double(TokenType::StrikeBegin, c.0));
                                state.strike = Tag::Strike(true);
                            } else {
                                tokens.push(Token::new_double(TokenType::StrikeEnd, c.0));
                                state.strike = Tag::Strike(false);
                            }
                        },
                        _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
                    }
                },
                None => tokens.push(Token::new_single(TokenType::Text, c.0)),
            }
        },
        '_' => {
            match iter.peek() {
                Some(v) => {
                    match v.1 {
                        '_' => {
                            iter.next();
                            pos.increment();
                            if state.underline == Tag::Underline(false) {
                                tokens.push(Token::new_double(TokenType::UnderlineBegin, c.0));
                                state.underline = Tag::Underline(true);
                            } else {
                                tokens.push(Token::new_double(TokenType::UnderlineEnd, c.0));
                                state.underline = Tag::Underline(false);
                            }
                        },
                        _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
                    }
                },
                None => tokens.push(Token::new_single(TokenType::Text, c.0)),
            }
        },
        _ => panic!("In 'match_emphasis()' found char other than accounted for!"),
    }
}

pub fn match_string(query: String, text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) -> bool {
    // TODO Utilize this function in other places in code.
    for ch in query.chars() {
        match iter.next() {
            Some(v) => {
                pos.increment();
                if v.1 == ch {
                    println!("matched char");
                } else {
                    return false;
                }
            },
            None => return false,
        }
    }

    true
}

//TODO add tests
