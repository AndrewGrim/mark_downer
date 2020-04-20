use crate::token::Token;
use crate::token::TokenType;
use crate::position::Position;
use crate::emphasis_state::State;
use crate::markdown;

pub fn lex(text: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(text.len());
    let mut iter = text.chars().enumerate().peekable();
    let mut pos: Position = Position::new(0, 0, 0);
    let mut state: State = State::new();
    loop {
        match iter.next() {
            Some(c) => {
                pos.increment();
                match c.1 {
                    '#' => {
                        markdown::match_heading(&mut tokens, &mut iter, &mut pos, c);
                    },
                    '-' => {
                        match iter.peek() {
                            Some(v) => {
                                match v.1 {
                                    '-' => markdown::match_horizontalrule(text, &mut tokens, &mut iter, &mut pos, c),
                                    ' ' => markdown::match_checkbutton(text, &mut tokens, &mut iter, &mut pos, c),
                                    _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
                                }
                            },
                            None => tokens.push(Token::new_single(TokenType::Text, c.0)),
                        }
                    },
                    '!' => {
                        markdown::match_image(text, &mut tokens, &mut iter, &mut pos, c);
                    },
                    '[' => {
                        markdown::match_link(text, &mut tokens, &mut iter, &mut pos, c);
                    },
                    '>' => {
                        markdown::match_blockquote(text, &mut tokens, &mut iter, &mut pos, c);
                    },
                    '`' => {
                        match iter.peek() {
                            Some(v) => {
                                match v.1 {
                                    '`' => markdown::match_codeblock(text, &mut tokens, &mut iter, &mut pos, c),
                                    _ => markdown::match_code(text, &mut tokens, &mut iter, &mut pos, c),
                                }
                            },
                            None => tokens.push(Token::new_single(TokenType::Text, c.0)),
                        }
                    },
                    ' ' => match iter.peek() {
                        Some(v) => {
                            match v.1 {
                                ' ' => markdown::match_indentblock(text, &mut tokens, &mut iter, &mut pos, c),
                                _ => tokens.push(Token::new_single(TokenType::Space, c.0)),
                            }
                        },
                        None => tokens.push(Token::new_single(TokenType::Space, c.0)),
                    },
                    '*'|'~'|'_' => markdown::match_emphasis(&mut state, text, &mut tokens, &mut iter, &mut pos, c),
                    '\n' => tokens.push(Token::new_single(TokenType::Newline, c.0)),
                    '\t' => tokens.push(Token::new_single(TokenType::Tab, c.0)),
                    '\\' => tokens.push(Token::new_single(TokenType::Escape, c.0)),
                    _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
                }
            },
            None => break,
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::fs;

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

    #[test]
    fn indentblock() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/indentblock.md")?);
        let mut indent: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::IndentBlock => {
                    indent += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(indent == 4);

        Ok(())
    }

    #[test]
    fn escape() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/escape.md")?);
        let mut esc: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::Escape => {
                    esc += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ')|TokenType::Heading
                |TokenType::ItalicBegin|TokenType::ItalicEnd|TokenType::BoldBegin|TokenType::BoldEnd
                |TokenType::StrikeBegin|TokenType::StrikeEnd|TokenType::UnderlineBegin|TokenType::UnderlineEnd => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(esc == 10);

        Ok(())
    }

    #[test]
    fn emphasis() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/emphasis.md")?);
        let mut i: usize = 0;
        let mut b: usize = 0;
        let mut s: usize = 0;
        let mut u: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::ItalicBegin|TokenType::ItalicEnd => {
                    i += 1;
                },
                TokenType::BoldBegin|TokenType::BoldEnd => {
                    b += 1;
                },
                TokenType::StrikeBegin|TokenType::StrikeEnd => {
                    s += 1;
                },
                TokenType::UnderlineBegin|TokenType::UnderlineEnd => {
                    u += 1;
                },
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Whitespace(' ') => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(i == 6);
        assert!(b == 4);
        assert!(s == 2);
        assert!(u == 2);

        Ok(())
    }

    #[test]
    fn html() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/html.md")?);
        for token in t.iter() {
            match token.id {
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
                _ => panic!(format!("Encounterd TokenType other than expected! {:#?}", token)),
            }
        }

        Ok(())
    }
}