use std::path;
use std::fs;
use std::collections::HashMap;

use crate::markdown;
use crate::position::Position;
use crate::wrapper::CharsWithPosition;
use crate::token::Token;
use crate::token::TokenType;

#[derive(Debug)]
pub struct Syntax {
    pub keywords1: Vec<String>,
    pub keywords2: Vec<String>,
    pub keywords3: Vec<String>,
    pub single_line_comment: String,
    pub multi_line_comment_open: String,
    pub multi_line_comment_close: String,
}

impl Syntax {
    pub fn new() -> Syntax {
        Syntax {
            keywords1: Vec::new(),
            keywords2: Vec::new(),
            keywords3: Vec::new(),
            single_line_comment: String::new(),
            multi_line_comment_open: String::new(),
            multi_line_comment_close: String::new(),
        }
    }

    pub fn single_open(&self) -> char {
        self.single_line_comment.chars().next().unwrap()
    }

    pub fn multi_open(&self) -> char {
        self.multi_line_comment_open.chars().next().unwrap()
    }
}

pub fn load_language_file(lang: &str) -> Option<Syntax> {
    let p = format!("syntax/{}.toml", lang.to_string());
    let path = path::Path::new(&p);

    if path.exists() {
        let mut content = fs::read_to_string(path).expect("Couldn't load syntax file!");
        if cfg!(windows) {
            content = content.replace("\r", " ");
        }
        return Some(parse(&content));
    }

    None
}

fn parse(text: &String) -> Syntax {
    let mut iter = CharsWithPosition::new(Position::new(), text.chars().enumerate().peekable());
    let mut syntax = Syntax::new();
    let mut map: HashMap<String, Vec<String>> = HashMap::with_capacity(5);

    while let Some(v) = iter.next() {
        if v.1.is_alphabetic() {
            let begin: usize = v.0;
            let mut key = String::new();
            let mut values: Vec<String> = Vec::with_capacity(10);
            while let Some(v) = iter.next() {
                if v.1 == ' ' {
                    key.push_str(&text[begin..iter.last()]);
                    break;
                }
            }
            while let Some(v) = iter.next() {
                if v.1 == '[' {
                    while let Some(v) = iter.next() {
                        match v.1 {
                            ']' => break,
                            '"' => {
                                let begin = v.0 + 1; // step over '"'
                                while let Some(v) = iter.next() {
                                    if v.1 == '"' {
                                        values.push(String::from(&text[begin..iter.last()]));
                                        break;
                                    }
                                }
                            },
                            _ => (),
                        }
                    }
                    break;
                } else if v.1 == '"' {
                    let begin = v.0 + 1; // step over '"'
                    while let Some(v) = iter.next() {
                        if v.1 == '"' {
                            values.push(String::from(&text[begin..iter.last()]));
                            break;
                        }
                    }
                    break;
                }
            }
            map.insert(key, values);
        }
    }

    if let Some(v) = map.remove("keywords1") {
        syntax.keywords1 = v;
    } 
    if let Some(v) = map.remove("keywords2") {
        syntax.keywords2 = v;
    } 
    if let Some(v) = map.remove("keywords3") {
        syntax.keywords3 = v;
    } 
    if let Some(mut v) = map.remove("single_line_comment") {
        syntax.single_line_comment = v.pop().unwrap();
    }
    if let Some(mut v) = map.remove("multi_line_comment_open") {
        syntax.multi_line_comment_open = v.pop().unwrap();
    } 
    if let Some(mut v) = map.remove("multi_line_comment_close") {
        syntax.multi_line_comment_close = v.pop().unwrap();
    }

    syntax
}

pub fn single_comment(single_comment: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, v: (usize, char)) {
    let begin = v.0;
    if single_comment.len() != 1 {
        let single = single_comment.get(1..).unwrap();
        if markdown::match_string(single, iter) {
            while let Some(v) = iter.next() {
                match v.1 {
                    '\n' => {
                        tokens.push(Token::new(TokenType::CodeBlockSingleLineComment, begin, iter.index()));
                        break;
                    },
                    _ => (),
                }
            }
        } 
    } else {
        while let Some(v) = iter.next() {
            match v.1 {
                '\n' => {
                    tokens.push(Token::new(TokenType::CodeBlockSingleLineComment, begin, iter.index()));
                    break;
                },
                _ => (),
            }
        }
    }
}

pub fn multi_comment(multi_comment_open: &String, multi_comment_close: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, v: (usize, char)) {
    let begin = v.0;
    let multi = multi_comment_open.get(1..).unwrap();
    let multi_close = multi_comment_close.get(0..).unwrap();
    if markdown::match_string(multi, iter) {
        while let Some(v) = iter.next() {
            if markdown::match_string(multi_close, iter) {
                tokens.push(Token::new(TokenType::CodeBlockMultiLineComment, begin, iter.index()));
                break;
            }
        }
    }
}

pub fn string_or_char(begin: char, end: char, token_type: TokenType, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, v: (usize, char)) {
    if v.1 == begin {
        let mut start: usize = v.0;
        loop {
            match iter.next() {
                Some(v) => {
                    if v.1 == end {
                        tokens.push(Token::new(token_type, start, iter.index()));
                        break;
                    }
                    else if v.1 == '\\' {
                        tokens.push(Token::new(token_type, start, iter.last()));
                        tokens.push(Token::new_double(TokenType::CodeBlockEscape, iter.last()));
                        iter.next();
                        start = iter.index();
                    }
                },
                None => break,
            }
        }
    }
}

fn is_keyword(text: &str, keywords: &Vec<String>) -> bool {
    for k in keywords {
        if &text == k {
            return true;
        }
    }

    false
}

pub fn keyword(lang: &str, keywords: (&Vec<String>, &Vec<String>, &Vec<String>), text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, v: (usize, char)) {
    let begin = v.0;
    while let Some(v) = iter.next() {
        if !v.1.is_alphanumeric() && v.1 != '_' {
            if is_keyword(&text[begin..iter.last()], keywords.0) {
                tokens.push(Token::new(TokenType::CodeBlockKeyword1, begin, iter.last()));
                tokens.push(Token::new(TokenType::CodeBlockSymbol, iter.last(), iter.index()));
            } else if is_keyword(&text[begin..iter.last()], keywords.1) {
                tokens.push(Token::new(TokenType::CodeBlockKeyword2, begin, iter.last()));
                tokens.push(Token::new(TokenType::CodeBlockSymbol, iter.last(), iter.index()));
            } else if is_keyword(&text[begin..iter.last()], keywords.2) {
                tokens.push(Token::new(TokenType::CodeBlockKeyword3, begin, iter.last()));
                tokens.push(Token::new(TokenType::CodeBlockSymbol, iter.last(), iter.index()));
            } else if v.1 == '(' {
                tokens.push(Token::new(TokenType::CodeBlockFunction, begin, iter.last()));
                tokens.push(Token::new(TokenType::CodeBlockSymbol, iter.last(), iter.index()));
            } else {
                tokens.push(Token::new(TokenType::CodeBlockText, begin, iter.last()));
                tokens.push(Token::new(TokenType::CodeBlockSymbol, iter.last(), iter.index()));
            }
            break;
        }
    }
}

pub fn highlight_language(syntax: Syntax, lang: &str, lang_end: usize, text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
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
                                                        match iter.peek() {
                                                            Some(v) => match v.1 {
                                                                '\n' => {
                                                                    // "iter.index() - 1" Because we dont want to include the "```" in the codeblock.
                                                                    tokens.push(Token::new(TokenType::CodeBlockEnd, lang_end, iter.index() - 1));
                                                                    iter.next();
                                                                    tokens.push(Token::new_single(TokenType::Newline, iter.index()));
                                                                    break;
                                                                },
                                                                _ => (),
                                                            },
                                                            None => tokens.push(Token::new(TokenType::Text, c.0, iter.last())),
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
                                tokens.push(Token::new(TokenType::Text, c.0, iter.last()));
                                break;
                            },
                        }
                    },
                    '"' => string_or_char('"', '"', TokenType::CodeBlockString, tokens, iter, v),
                    '\'' => string_or_char('\'', '\'', TokenType::CodeBlockChar, tokens, iter, v),
                    '0'..='9' => tokens.push(Token::new_single(TokenType::CodeBlockDigit, v.0)),
                    _ => if v.1.is_alphabetic() || v.1 == '_' {
                            keyword(lang, (&syntax.keywords1, &syntax.keywords2, &syntax.keywords3), text, tokens, iter, v);
                        } else if v.1 == syntax.single_open() {
                            // TODO what an awful mess
                            if syntax.single_open() == syntax.multi_open() {
                                if let Some(p) = iter.peek() {
                                    if syntax.single_open() != p.1 {
                                        multi_comment(&syntax.multi_line_comment_open,
                                            &syntax.multi_line_comment_close, tokens, iter, v);
                                    } else {
                                        single_comment(&syntax.single_line_comment, tokens, iter, v);
                                    }
                                }
                            } else {
                                single_comment(&syntax.single_line_comment, tokens, iter, v);
                            }
                        } else if v.1 == syntax.multi_open() {
                            multi_comment(&syntax.multi_line_comment_open,
                                                &syntax.multi_line_comment_close, tokens, iter, v);
                        } else {
                            tokens.push(Token::new_single(TokenType::CodeBlockSymbol, v.0));
                    },
                }
            },
            None => {
                tokens.push(Token::new(TokenType::Text, c.0, iter.last()));
                break;
            },
        }
    }
}

pub fn highlight_generic(lang_end: usize, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
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
                                                        match iter.peek() {
                                                            Some(v) => match v.1 {
                                                                '\n' => {
                                                                    // "iter.index() - 1" Because we dont want to include the "```" in the codeblock.
                                                                    tokens.push(Token::new(TokenType::CodeBlockEnd, lang_end, iter.index() - 1));
                                                                    iter.next();
                                                                    tokens.push(Token::new_single(TokenType::Newline, iter.index()));
                                                                    break;
                                                                },
                                                                _ => (),
                                                            },
                                                            None => tokens.push(Token::new(TokenType::Text, c.0, iter.last())),
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
                                tokens.push(Token::new(TokenType::Text, c.0, iter.last()));
                                break;
                            },
                        }
                    },
                    '"' => string_or_char('"', '"', TokenType::CodeBlockString, tokens, iter, v),
                    '\'' => string_or_char('\'', '\'', TokenType::CodeBlockChar, tokens, iter, v),
                    '0'..='9' => tokens.push(Token::new_single(TokenType::CodeBlockDigit, v.0)),
                    _ => if v.1.is_alphabetic() || v.1 == '_' {
                            let begin = v.0;
                            while let Some(v) = iter.next() {
                                if !v.1.is_alphanumeric() && v.1 != '_' {
                                    if v.1 == '(' {
                                        tokens.push(Token::new(TokenType::CodeBlockFunction, begin, iter.last()));
                                        tokens.push(Token::new(TokenType::CodeBlockSymbol, iter.last(), iter.index()));
                                    } else {
                                        tokens.push(Token::new(TokenType::CodeBlockText, begin, iter.last()));
                                        tokens.push(Token::new(TokenType::CodeBlockSymbol, iter.last(), iter.index()));
                                    }
                                    break;
                                }
                            }
                        } else {
                            tokens.push(Token::new_single(TokenType::CodeBlockSymbol, v.0));
                    },
                }
            },
            None => {
                tokens.push(Token::new(TokenType::Text, c.0, iter.last()));
                break;
            },
        }
    }
}