use std::iter;
use std::str;

use crate::position::Position;
use crate::token::Token;
use crate::token::TokenType;
use crate::emphasis::Tag;
use crate::emphasis;
use crate::table::Alignment;
use crate::table;
use crate::lexer;
use crate::wrapper::CharsWithPosition;

// TODO anywhere we need to verify indexes we must leave a comment
// of why the index is as such

pub fn match_heading(text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    if c.0 == 0 || &text[c.0 - 1..c.0] == "\n" {
        let mut heading_count: usize = 1;
        while let Some(v) = iter.next() {
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
                    tokens.push(Token::new(TokenType::Text, c.0, iter.index()));
                    break;
                },
            }
        }
    } else {
        tokens.push(Token::new_single(TokenType::Text, c.0));
    }
}

pub fn match_checkbutton(text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    match text.get(c.0 + 2..c.0 + 6) {
        Some(v) => {
            if v == "[ ] " {
                tokens.push(Token::new(TokenType::Checkbutton(false), c.0, c.0 + 5));
                iter.nth(3);
            } else if v == "[x] " {
                tokens.push(Token::new(TokenType::Checkbutton(true), c.0, c.0 + 5));
                iter.nth(3);
            } else {
                tokens.push(Token::new_single(TokenType::Text, c.0));
            }
        },
        None => tokens.push(Token::new_single(TokenType::Text, c.0)),
    }
}

pub fn match_image(text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    match iter.peek() {
        Some(v) => {
            match v.1 {
                '[' => {
                    let alt_begin: usize = v.0 + 1;
                    iter.next();
                    while let Some(v) = iter.next() {
                        match v.1 {
                            ']' =>  {
                                let alt_end: usize = v.0;
                                match iter.peek() {
                                    Some(v) => {
                                        match v.1 {
                                            '(' => {
                                                let src_begin: usize = v.0 + 1;
                                                while let Some(v) = iter.next() {
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

pub fn match_link(text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    let text_begin: usize = c.0 + 1;
    loop {
        match iter.next() {
            Some(v) => {
                match v.1 {
                    ']' =>  {
                        let text_end: usize = v.0;
                        match iter.peek() {
                            Some(v) => {
                                match v.1 {
                                    '(' => {
                                        let href_begin: usize = v.0 + 1;
                                        while let Some(v) = iter.next() {
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

pub fn match_horizontalrule(text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    iter.next();
    match iter.next() {
        Some(v) => {
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

pub fn match_blockquote(text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    if c.0 == 0 || &text[c.0 - 1..c.0] == "\n" {
        tokens.push(Token::new_single(TokenType::BlockquoteBegin, c.0));
        loop {
            match iter.next() {
                Some(v) => {
                    match v.1 {
                        '\n' => {
                            match iter.peek() {
                                Some(v) => {
                                    match v.1 {
                                        '\n' => {
                                            tokens.push(Token::new(TokenType::BlockquoteEnd, c.0, v.0));
                                            tokens.push(Token::new_single(TokenType::Newline, v.0));
                                            break;
                                        },
                                        _ => tokens.push(Token::new_single(TokenType::Text, v.0)),
                                    }
                                    iter.next();
                                },
                                None => tokens.push(Token::new(TokenType::BlockquoteEnd, c.0, v.0)),
                            }
                        },
                        _ => tokens.push(Token::new_single(TokenType::Text, v.0)),
                    }
                },
                None => {
                    tokens.push(Token::new(TokenType::BlockquoteEnd, c.0, iter.last()));
                    break;
                },
            }
        }
    } else  {
        tokens.push(Token::new_single(TokenType::Text, c.0));
    }
}

pub fn match_code(text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    if c.0 == 0 || &text[c.0 - 1..c.0] != "`" {
        loop {
            match iter.next() {
                Some(v) => {
                    match v.1 {
                        '`' => {
                            tokens.push(Token::new(TokenType::Code, c.0 + 1, v.0));
                            break;
                        },
                        _ => (),
                    }
                },
                None => {
                    tokens.push(Token::new(TokenType::Text, c.0, iter.last()));
                    break;
                },
            }
        }
    } else {
        tokens.push(Token::new_single(TokenType::Text, c.0));
    }
}

pub fn match_codeblock(text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    // TODO Perhaps when looking for closing backticks also check if the following characters is a newline.
    // And only then push a closing token.
    if c.0 == 0 || &text[c.0 - 1..c.0] == "\n" {
        iter.next();
        match iter.peek() {
            Some(v) => {
                let v = iter.next().unwrap(); // TODO are we sure about this
                match v.1 {
                    '`' =>{
                        tokens.push(Token::new(TokenType::CodeBlockBegin, c.0, iter.index()));
                        let lang_begin: usize = iter.index();
                        loop {
                            match iter.next() {
                                Some(v) => {
                                    match v.1 {
                                        '\n' => {
                                            tokens.push(Token::new(TokenType::CodeBlockLanguage, lang_begin, iter.index()));
                                            break;
                                        },
                                        _ => (),
                                    }
                                },
                                None => break,
                            }
                        }
                        let lang_end: usize = iter.index();
                        loop {
                            match iter.next() {
                                Some(v) => {
                                    match v.1 {
                                        '`' => {
                                            match iter.next() {
                                                Some(v) => {
                                                    match v.1 {
                                                        '`' => {
                                                            match iter.next() {
                                                                Some(v) => {
                                                                    match v.1 {
                                                                        '`' => {
                                                                            tokens.push(Token::new(TokenType::CodeBlockEnd, lang_end, v.0));
                                                                            break;
                                                                        },
                                                                        _ => (),
                                                                    }
                                                                },
                                                                None => {
                                                                    tokens.push(Token::new(TokenType::Text, c.0, iter.last()));
                                                                    break;
                                                                },
                                                            }
                                                        },
                                                        _ => (),
                                                    }
                                                },
                                                None => {
                                                    tokens.push(Token::new(TokenType::Text, c.0, iter.last()));
                                                    break;
                                                },
                                            }
                                        },
                                        _ => (),
                                    }
                                },
                                None => {
                                    tokens.push(Token::new(TokenType::Text, c.0, iter.last() - 1)); // TODO check this out
                                    break;
                                },
                            }
                        }
                    },
                    _ => tokens.push(Token::new(TokenType::Text, c.0, v.0)),
                }
            },
            None => tokens.push(Token::new(TokenType::Text, c.0, iter.index())),
        }
    } else {
        tokens.push(Token::new_single(TokenType::Text, c.0));
    }
}

pub fn match_indentblock(text: &String, mut tokens: &mut Vec<Token>, mut iter: &mut CharsWithPosition, c: (usize, char)) {
    iter.next();
    if match_string(String::from("  "), text, &mut tokens, &mut iter, c) {
        loop {
            match iter.next() {
                Some(v) => {
                    match v.1 {
                        '\n' => {
                            if !match_string(String::from("    "), text, &mut tokens, &mut iter, c) {
                                tokens.push(Token::new(TokenType::IndentBlock, c.0, iter.last())); // TODO why is this  - 1
                                tokens.push(Token::new_single(TokenType::Text, iter.last())); // TODO why is this  - 1
                                break;
                            } 
                        },
                        _ => (),
                    }
                },
                None => {
                    tokens.push(Token::new(TokenType::Text, c.0, iter.last()));
                    break;
                },
            }
        }
    }
}

pub fn match_emphasis(mut state: &mut emphasis::State, text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    match c.1 {
        '*' => {
            match iter.peek() {
                Some(v) => {
                    match v.1 {
                        '*' => {
                            iter.next();
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

pub fn match_table(table: &table::State, text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) -> bool {
    // TODO verify index positions for errors and others 
    let mut index_start = c.0 + 2;
    let mut index_end = index_start + 4;
    loop {
        let mut _column_alignment = Alignment::Left;
        match text.get(index_start..index_end) {
            Some(v) => {
                if v == " ---" || v == ":---" {
                    match iter.next() {
                        Some(v) => {
                            match v.1 {
                                ' ' => _column_alignment = Alignment::LeftOrRight,
                                ':' => _column_alignment = Alignment::LeftOrCenter,
                                _ => panic!("In 'match_table()' found char other than accounted for!"),
                            }
                            iter.nth(2);
                            loop {
                                match iter.next() {
                                    Some(v) => {
                                        match v.1 {
                                            '-' => (),
                                            ':' => {
                                                if _column_alignment == Alignment::LeftOrCenter {
                                                    _column_alignment = Alignment::Center;
                                                    break;
                                                } else if _column_alignment == Alignment::LeftOrRight {
                                                    _column_alignment = Alignment::Right;
                                                    break;
                                                }
                                            },
                                            ' ' => {
                                                _column_alignment = Alignment::Left;
                                                break;
                                            },
                                            _ => {
                                                tokens.push(Token::new_single(TokenType::Error, iter.index()));
                                                return false;
                                            },
                                        }
                                    },
                                    None => {
                                        tokens.push(Token::new_single(TokenType::Text, c.0 + 1));
                                        tokens.push(Token::new(TokenType::Text, c.0 + 2, iter.last()));
                                        return false;
                                    },
                                }
                            }
                            match iter.next() {
                                Some(v) => {
                                    match v.1 {
                                        '|' => {
                                            match _column_alignment {
                                                Alignment::Left => tokens.insert(table.table_index, Token::new(TokenType::TableColumnLeft, index_start - 1, iter.last())), // TODO verify the indexes
                                                Alignment::Right => tokens.insert(table.table_index, Token::new(TokenType::TableColumnRight, index_start - 1, iter.last())), // TODO verify the indexes
                                                Alignment::Center => tokens.insert(table.table_index, Token::new(TokenType::TableColumnCenter, index_start - 1, iter.last())), // TODO verify the indexes
                                                _ => {
                                                    tokens.push(Token::new(TokenType::Error, index_start - 1, iter.last()));
                                                    return false;
                                                },
                                            }
                                            match iter.peek() {
                                                Some(v) => {
                                                    match v.1 {
                                                        '\n' => {
                                                            iter.next();
                                                            break;
                                                        },
                                                        ' '|':' => {
                                                            index_start = iter.index();
                                                            index_end = index_start + 4;
                                                            continue;
                                                        },
                                                        _ => {
                                                            tokens.push(Token::new(TokenType::Error, c.0 + 1, iter.last()));
                                                            return false;
                                                        },
                                                    }
                                                },
                                                None => {
                                                    tokens.push(Token::new(TokenType::Error, c.0 + 1, iter.last()));
                                                    return false;
                                                },
                                            }
                                        },
                                        _ => {
                                            tokens.push(Token::new(TokenType::Error, c.0 + 1, iter.last()));
                                            return false;
                                        },
                                    }
                                },
                                None => return false,
                            }
                        },
                        None => panic!("In 'match_table()' found None even though v matched correctly!"),
                    }
                } else {
                    tokens.push(Token::new_single(TokenType::Text, iter.last()));
                    return false;
                }
            },
            None => {
                tokens.push(Token::new_single(TokenType::Text, iter.last()));
                return false;
            },
        }
    }

    true
}

pub fn match_string(query: String, text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) -> bool {
    // TODO Utilize this function in other places in code.
    for ch in query.chars() {
        match iter.peek() {
            Some(v) => {
                if v.1 != ch {
                    return false;
                }
                iter.next();
            },
            None => return false,
        }
    }

    true
}

//TODO add tests
