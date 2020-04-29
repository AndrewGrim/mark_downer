use crate::token::Token;
use crate::token::TokenType;
use crate::emphasis::Tag;
use crate::emphasis;
use crate::table::Alignment;
use crate::table;
use crate::wrapper;
use crate::wrapper::CharsWithPosition;

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
                                                            tokens.push(Token::new_single(TokenType::Newline, v.0));
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
                                                    tokens.push(Token::new_single(TokenType::Newline, v.0));
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

pub fn match_blockquote(mut emphasis: &mut emphasis::State, text: &String, mut tokens: &mut Vec<Token>, mut iter: &mut CharsWithPosition, c: (usize, char)) {
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
                        '*'|'~'|'_' => match_emphasis(&mut emphasis, text, &mut tokens, &mut iter, v),
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
    if c.0 == 0 || &text[c.0 - 1..c.0] == "\n" {
        iter.next();
        match iter.peek() {
            Some(v) => {
                // ".unwrap()" Is safe here because peek already matched "Some()" value.
                let v = iter.next().unwrap();
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
                                        _ => if v.1.is_alphabetic() {
                                                keyword(lang_end - lang_begin, tokens, iter, v);
                                            } else {
                                                tokens.push(Token::new_single(TokenType::CodeBlockText, v.0));
                                        },
                                    }
                                },
                                None => {
                                    tokens.push(Token::new(TokenType::Text, c.0, iter.last()));
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

fn string_or_char(begin: char, end: char, token_type: TokenType, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, v: (usize, char)) {
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

fn keyword(lang: usize, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, v: (usize, char)) {
    // TODO read file with keywords and match based on that
}

pub fn match_indentblock(text: &String, mut tokens: &mut Vec<Token>, mut iter: &mut CharsWithPosition, c: (usize, char)) {
    iter.next();
    if match_string(String::from("  "), text, &mut tokens, &mut iter) {
        loop {
            match iter.next() {
                Some(v) => {
                    match v.1 {
                        '\n' => {
                            if !match_string(String::from("    "), text, &mut tokens, &mut iter) {
                                // "c.0 + 1" Steps over the newline which is required to start an indented block.
                                tokens.push(Token::new(TokenType::IndentBlock, c.0 + 1, iter.last()));
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

pub fn match_emphasis(mut emphasis: &mut emphasis::State, text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition, c: (usize, char)) {
    match c.1 {
        '*' => {
            match iter.peek() {
                Some(v) => {
                    match v.1 {
                        '*' => {
                            iter.next();
                            if emphasis.bold == Tag::Bold(false) {
                                tokens.push(Token::new_double(TokenType::BoldBegin, c.0));
                                emphasis.bold = Tag::Bold(true);
                            } else {
                                tokens.push(Token::new_double(TokenType::BoldEnd, c.0));
                                emphasis.bold = Tag::Bold(false);
                            }
                        },
                        _ => {
                            if emphasis.italic == Tag::Italic(false) {
                                tokens.push(Token::new_single(TokenType::ItalicBegin, c.0));
                                emphasis.italic = Tag::Italic(true);
                            } else {
                                tokens.push(Token::new_single(TokenType::ItalicEnd, c.0));
                                emphasis.italic = Tag::Italic(false);
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
                            if emphasis.strike == Tag::Strike(false) {
                                tokens.push(Token::new_double(TokenType::StrikeBegin, c.0));
                                emphasis.strike = Tag::Strike(true);
                            } else {
                                tokens.push(Token::new_double(TokenType::StrikeEnd, c.0));
                                emphasis.strike = Tag::Strike(false);
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
                            if emphasis.underline == Tag::Underline(false) {
                                tokens.push(Token::new_double(TokenType::UnderlineBegin, c.0));
                                emphasis.underline = Tag::Underline(true);
                            } else {
                                tokens.push(Token::new_double(TokenType::UnderlineEnd, c.0));
                                emphasis.underline = Tag::Underline(false);
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

pub fn match_list(list_type: wrapper::ListType, text: &String, mut tokens: &mut Vec<Token>, mut iter: &mut CharsWithPosition, c: (usize, char)) {
    let mut lists: Vec<wrapper::List> = Vec::with_capacity(10);
    tokens.push(Token::new_double(list_type.0, iter.index()));
    lists.push(wrapper::List(list_type.1, 0));
    iter.next();
    tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
    let mut emphasis = emphasis::State::new();
    while let Some(v) = iter.next() {
        match v.1 {
            '\n' => {
                tokens.push(Token::new_single(TokenType::ListItemEnd, iter.index()));
                if let Some(v) = iter.peek() {
                    let indent_begin = v.0;
                    match v.1 {
                        '\n' => {
                            for i in (0..lists.len()).rev() {
                                let l = lists.pop().unwrap();
                                tokens.push(Token::new_single(l.0, iter.index()));
                            }
                            break;
                        },
                        '*' => {
                            iter.next();
                            if let Some(v) = iter.peek() {
                                match v.1 {
                                    ' ' => {
                                        push_list(
                                            wrapper::ListType(TokenType::UnorderedListBegin, TokenType::UnorderedListEnd),
                                            &mut lists,
                                            tokens, 
                                            iter
                                        );
                                    },
                                    _ => {
                                        tokens.push(Token::new_double(TokenType::Error, iter.last()));
                                        iter.next();
                                    },
                                }
                            }
                        },
                        '1' => {
                            iter.next();
                            if let Some(v) = iter.peek() {
                                match v.1 {
                                    '.' => {
                                        iter.next();
                                        if let Some(v) = iter.peek() {
                                            match v.1 {
                                                ' ' => {
                                                    push_list(
                                                        wrapper::ListType(TokenType::OrderedListBegin, TokenType::OrderedListEnd),
                                                        &mut lists,
                                                        tokens, 
                                                        iter
                                                    );
                                                },
                                                _ => {
                                                    tokens.push(Token::new_double(TokenType::Error, iter.last()));
                                                    iter.next();
                                                },
                                            }
                                        }
                                    },
                                    _ => {
                                        tokens.push(Token::new_double(TokenType::Error, iter.last()));
                                        iter.next();
                                    },
                                }
                            }
                        },
                        ' ' => {
                            loop {
                                if !match_string(String::from("    "), text, &mut tokens, &mut iter) {
                                    tokens.push(Token::new(TokenType::Error, indent_begin, iter.index()));
                                    break;
                                } else {
                                    if let Some(v) = iter.peek() {
                                        match v.1 {
                                            ' ' => continue,
                                            '*' => {
                                                let current_indent = v.0 - indent_begin;
                                                iter.next();
                                                if let Some(v) = iter.peek() {
                                                    match v.1 {
                                                        ' ' => {
                                                            push_indented_list(
                                                                current_indent,
                                                                wrapper::ListType(TokenType::UnorderedListBegin, TokenType::UnorderedListEnd),
                                                                &mut lists,
                                                                tokens, 
                                                                iter
                                                            );
                                                            break;
                                                        },
                                                        _ => {
                                                            tokens.push(Token::new_double(TokenType::Error, iter.last()));
                                                            iter.next();
                                                            break;
                                                        },
                                                    }
                                                }
                                            },
                                            '1' => {
                                                let current_indent = v.0 - indent_begin;
                                                iter.next();
                                                if let Some(v) = iter.peek() {
                                                    match v.1 {
                                                        '.' => {
                                                            iter.next();
                                                            if let Some(v) = iter.peek() {
                                                                match v.1 {
                                                                    ' ' => {
                                                                        push_indented_list(
                                                                            current_indent,
                                                                            wrapper::ListType(TokenType::OrderedListBegin, TokenType::OrderedListEnd),
                                                                            &mut lists,
                                                                            tokens, 
                                                                            iter
                                                                        );
                                                                        break;
                                                                    },
                                                                    _ => {
                                                                        tokens.push(Token::new_double(TokenType::Error, iter.last()));
                                                                        iter.next();
                                                                        break;
                                                                    },
                                                                }
                                                            }
                                                        },
                                                        _ => {
                                                            tokens.push(Token::new_double(TokenType::Error, iter.last()));
                                                            iter.next();
                                                        },
                                                    }
                                                }
                                            },
                                            _ => tokens.push(Token::new(TokenType::Error, indent_begin, v.0)),
                                        }
                                    }
                                }
                            }
                        },
                        _ => {
                            tokens.push(Token::new_single(TokenType::Error, iter.index()));
                            iter.next();
                        },
                    }
                } else {
                    tokens.push(Token::new_single(TokenType::Newline, iter.last()));
                }
            },
            '*'|'~'|'_' => match_emphasis(&mut emphasis, text, &mut tokens, &mut iter, v),
            '[' => match_link(text, &mut tokens, &mut iter, v),
            '-' => match_checkbutton(text, &mut tokens, &mut iter, v),
            _ => tokens.push(Token::new_single(TokenType::Text, iter.last())), // TODO why is last() the correct thing to do here?
        }
    }
    // maybe push token
}

fn push_list(list_type: wrapper::ListType, lists: &mut Vec<wrapper::List>, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition) {
    if lists[lists.len() - 1].1 > 0 {
        for i in (1..lists.len()).rev() {
            if lists[i].1 != 0 || lists[i].0 != list_type.1 {
                let l = lists.pop().unwrap();
                tokens.push(Token::new_single(l.0, iter.index()));
            }
        }
        if lists[lists.len() - 1].0 == list_type.1 {
            tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
            iter.next();
        } else {
            let l = lists.pop().unwrap();
            tokens.push(Token::new_single(l.0, iter.index()));
            tokens.push(Token::new_double(list_type.0, iter.index()));
            lists.push(wrapper::List(list_type.1, 0));
            iter.next();
            tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
        }
    } else {
        if lists[lists.len() - 1].0 == list_type.1 {
            tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
            iter.next();
        } else {
            let l = lists.pop().unwrap();
            tokens.push(Token::new_single(l.0, iter.index()));
            tokens.push(Token::new_double(list_type.0, iter.index()));
            lists.push(wrapper::List(list_type.1, 0));
            iter.next();
            tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
        }
    }
}

fn push_indented_list(current_indent: usize, list_type: wrapper::ListType, lists: &mut Vec<wrapper::List>, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition) {
    if lists[lists.len() - 1].1 > current_indent {
        while let Some(l) = lists.get(lists.len() - 1) {
            if l.1 == current_indent {
                break;
            }
            let l = lists.pop().unwrap();
            tokens.push(Token::new_single(l.0, iter.index()));
        }
        if lists[lists.len() - 1].0 == list_type.1 {
            tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
            iter.next();
        } else {
            let l = lists.pop().unwrap();
            tokens.push(Token::new_single(l.0, iter.index()));
            tokens.push(Token::new_double(list_type.0, iter.index()));
            lists.push(wrapper::List(list_type.1, current_indent));
            iter.next();
            tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
        }
    } else if lists[lists.len() - 1].1 == current_indent {
        if lists[lists.len() - 1].0 == list_type.1 {
            tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
            iter.next();
        } else {
            let l = lists.pop().unwrap();
            tokens.push(Token::new_single(l.0, iter.index()));
            tokens.push(Token::new_double(list_type.0, iter.index()));
            lists.push(wrapper::List(list_type.1, current_indent));
            iter.next();
            tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
        }
    } else {
        tokens.push(Token::new_double(list_type.0, iter.index()));
        lists.push(wrapper::List(list_type.1, current_indent));
        iter.next();
        tokens.push(Token::new_single(TokenType::ListItemBegin, iter.index()));
    }
}

pub fn match_string(query: String, text: &String, tokens: &mut Vec<Token>, iter: &mut CharsWithPosition) -> bool {
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
