#![allow(dead_code, unused_imports, unused_variables)]

use std::fs;
use std::io;
use std::io::Write;
use std::iter;
use std::str;
use std::option;

mod position;
use position::Position;

mod token;
pub use token::Token;
pub use token::TokenType;

pub fn markdown_to_html(input: &str, output: &str) -> Result<Vec<Token>, io::Error> {
    let text: String = fs::read_to_string(input)?;
    let tokens = lex(&text);
    let html = parse(&text, &tokens);
    generate_html(output.to_string(), html);

    Ok(tokens)
}

fn lex(text: &String) -> Vec<Token> {
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
                        match iter.peek() {
                            Some(v) => {
                                match v.1 {
                                    '-' => match_horizontalrule(text, &mut tokens, &mut iter, &mut pos, c),
                                    ' ' => match_checkbutton(text, &mut tokens, &mut iter, &mut pos, c),
                                    _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
                                }
                            },
                            None => tokens.push(Token::new_single(TokenType::Text, c.0)),
                        }
                    },
                    '!' => {
                        match_image(text, &mut tokens, &mut iter, &mut pos, c);
                    },
                    '[' => {
                        match_link(text, &mut tokens, &mut iter, &mut pos, c);
                    },
                    '>' => {
                        match_blockquote(text, &mut tokens, &mut iter, &mut pos, c);
                    },
                    '`' => {
                        match iter.peek() {
                            Some(v) => {
                                match v.1 {
                                    '`' => match_codeblock(text, &mut tokens, &mut iter, &mut pos, c),
                                    _ => match_code(text, &mut tokens, &mut iter, &mut pos, c),
                                }
                            },
                            None => tokens.push(Token::new_single(TokenType::Text, c.0)),
                        }
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

    tokens
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

fn match_link(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
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

fn match_horizontalrule(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
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

fn match_blockquote(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
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

fn match_code(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
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

fn match_codeblock(text: &String, tokens: &mut Vec<Token>, iter: &mut iter::Peekable<iter::Enumerate<str::Chars>>, pos: &mut Position, c: (usize, char)) {
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
            TokenType::LinkHref => {
                html.push(format!("<a href=\"{}\">", text[t.begin..t.end].to_string()));
                let tok = iter.next().unwrap();
                if text[tok.begin..tok.end].len() == 0 {
                    html.push(format!("{}</a>", text[t.begin..t.end].to_string()));
                } else {
                    html.push(format!("{}</a>", text[tok.begin..tok.end].to_string()));
                }
            },
            TokenType::BlockquoteBegin => {
                while let Some(tok) = iter.peek() {
                    match tok.id {
                        TokenType::Text => {iter.next();},
                        TokenType::BlockquoteEnd => {
                            html.push(format!("<blockquote>{}</blockquote>", text[tok.begin + 1..tok.end - 1].to_string()));
                            iter.next();
                        },
                        _ => break,
                    }
                }
                match iter.peek() {
                    Some(n) => {
                        if n.id == TokenType::Newline {
                            iter.next();
                        }
                    },
                    None => (),
                }
            },
            TokenType::CodeBlockBegin => {
                let lang_iter = match iter.peek() {
                    Some(n) => match n.id {
                            TokenType::CodeBlockLanguage => iter.next().unwrap(),
                            _ => continue,
                        },
                    None => break,
                };
                let mut lang = String::new();
                if lang_iter.end - lang_iter.begin == 1 {
                    lang += "base";
                } else {
                    lang = text[lang_iter.begin..lang_iter.end - 1].to_string();
                }

                let block = match iter.peek() {
                    Some(n) => match n.id {
                        TokenType::CodeBlockEnd => iter.next().unwrap(),
                        _ => continue,
                    },
                    None => break,
                };
                html.push(format!("<pre class=\"language-{}\">{}</pre>",
                        lang, text[block.begin..block.end - 2].to_string()));
                match iter.peek() {
                    Some(n) => {
                        if n.id == TokenType::Newline {
                            iter.next();
                        }
                    },
                    None => (),
                }
            },
            TokenType::HorizontalRule => html.push("<hr>\n".to_string()),
            TokenType::Code => html.push(format!("<code>{}</code>", text[t.begin..t.end].to_string())),
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
    fn heading() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/heading.md")?);
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

        Ok(())
    }

    #[test]
    fn checkbutton() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/checkbutton.md")?);
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

        Ok(())
    }

    #[test]
    fn image() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/image.md")?);
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

        Ok(())
    }

    #[test]
    fn link() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/link.md")?);
        let mut link_text: usize = 0;
        let mut link_href: usize = 0;
        let mut errors: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::LinkText => {
                    link_text += 1;
                },
                TokenType::LinkHref => {
                    link_href += 1;
                },
                TokenType::Error => {
                    errors += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(link_text == 2);
        assert!(link_href == 2);
        assert!(errors == 1);

        Ok(())
    }

    #[test]
    fn horizontalrule() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/horizontalrule.md")?);
        let mut hr: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::HorizontalRule => {
                    hr += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(hr == 1);

        Ok(())
    }

    #[test]
    fn blockqoute() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/blockquote.md")?);
        let mut bb: usize = 0;
        let mut be: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::BlockquoteBegin => {
                    bb += 1;
                },
                TokenType::BlockquoteEnd => {
                    be += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(bb == 2);
        assert!(be == 2);

        Ok(())
    }

    #[test]
    fn code() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/code.md")?);
        let mut code: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::Code => {
                    code += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(code == 2);

        Ok(())
    }

    #[test]
    fn codeblock() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/codeblock.md")?);
        let mut cbb: usize = 0;
        let mut cbe: usize = 0;
        let mut cbl: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::CodeBlockBegin => {
                    cbb += 1;
                },
                TokenType::CodeBlockEnd => {
                    cbe += 1;
                },
                TokenType::CodeBlockLanguage => {
                    cbl += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(cbb == 3);
        assert!(cbe == 2);
        assert!(cbl == 2);

        Ok(())
    }
}