use crate::token::Token;
use crate::token::TokenType;
use crate::position::Position;
use crate::emphasis;
use crate::markdown;
use crate::table;
use crate::wrapper;
use crate::wrapper::CharsWithPosition;

pub fn lex(text: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(text.len());
    let mut iter = CharsWithPosition::new(Position::new(), text.chars().enumerate().peekable());
    let mut emphasis: emphasis::State = emphasis::State::new();
    let mut table: table::State = table::State::new();
    loop {
        match iter.next() {
            Some(c) => {
                match c.1 {
                    '#' => {
                        markdown::match_heading(&text, &mut tokens, &mut iter, c);
                    },
                    '-' => {
                        match iter.peek() {
                            Some(v) => {
                                match v.1 {
                                    '-' => markdown::match_horizontalrule(text, &mut tokens, &mut iter, c),
                                    ' ' => markdown::match_checkbutton(text, &mut tokens, &mut iter, c),
                                    _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
                                }
                            },
                            None => tokens.push(Token::new_single(TokenType::Text, c.0)),
                        }
                    },
                    '!' => {
                        markdown::match_image(text, &mut tokens, &mut iter, c);
                    },
                    '[' => {
                        markdown::match_link(text, &mut tokens, &mut iter, c);
                    },
                    '>' => {
                        markdown::match_blockquote(&mut emphasis, text, &mut tokens, &mut iter, c);
                    },
                    '`' => {
                        match iter.peek() {
                            Some(v) => {
                                match v.1 {
                                    '`' => markdown::match_codeblock(text, &mut tokens, &mut iter, c),
                                    _ => markdown::match_code(text, &mut tokens, &mut iter, c),
                                }
                            },
                            None => tokens.push(Token::new_single(TokenType::Text, c.0)),
                        }
                    },
                    ' ' => tokens.push(Token::new_single(TokenType::Space, c.0)),
                    '\n' => {  // TODO break this off into its own function
                        tokens.push(Token::new_single(TokenType::Newline, c.0));
                        match iter.peek() {
                            Some(v) => {
                                match v.1 {
                                    '\n' => {
                                        if table.in_table {
                                            tokens.push(Token::new(TokenType::TableEnd, table.possible_table_start, v.0));
                                            tokens.insert(table.table_index, Token::new_single(TokenType::TableBegin, table.possible_table_start));
                                            table = table::State::new();
                                        }
                                        tokens.push(Token::new_single(TokenType::Newline, v.0));
                                        iter.next();
                                        match iter.peek() {
                                            Some(v) => match v.1 {
                                                '|' => {
                                                    if !table.possible_table {
                                                        table.possible_table = true;
                                                        table.possible_table_start = v.0;
                                                        table.table_index = tokens.len();
                                                    }
                                                },
                                                ' ' => markdown::match_indentblock(text, &mut tokens, &mut iter, c),
                                                _ => (),
                                            },
                                            None => (),
                                        }
                                    },
                                    '|' => {
                                        iter.next();
                                        if table.possible_table {
                                            let matched = markdown::match_table(&table, text, &mut tokens, &mut iter, c);
                                            if matched {
                                                table.in_table = true;
                                                table.possible_table = false;
                                            } else if !matched {
                                                table.possible_table = false;
                                            }
                                        } else {
                                            tokens.push(Token::new_single(TokenType::Pipe, c.0 + 1));
                                        }
                                    },
                                    ' ' => {
                                        iter.next();
                                        match iter.peek() {
                                            Some(v) => {
                                                match v.1 {
                                                    ' ' => markdown::match_indentblock(text, &mut tokens, &mut iter, c),
                                                    _ => {
                                                        tokens.push(Token::new_single(TokenType::Space, v.0));
                                                        iter.next();
                                                    },
                                                }
                                            },
                                            None => tokens.push(Token::new_single(TokenType::Space, c.0)),
                                        }
                                    },
                                    '*' => {
                                        iter.next();
                                        if let Some(v) = iter.peek() {
                                            match v.1 {
                                                ' ' => markdown::match_list(
                                                    wrapper::ListType(TokenType::UnorderedListBegin, TokenType::UnorderedListEnd),
                                                    text,
                                                    &mut tokens,
                                                    &mut iter,
                                                    (c.0 + 1, '_')
                                                ),
                                                '\n' => (),
                                                // "c.0 + 1" To step over opening newline.
                                                _ => markdown::match_emphasis(&mut emphasis, text, &mut tokens, &mut iter, (c.0 + 1, '*')),
                                            }
                                        } else {
                                            tokens.push(Token::new_single(TokenType::Text, iter.index()));
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
                                                            ' ' => markdown::match_list(
                                                                wrapper::ListType(TokenType::OrderedListBegin, TokenType::OrderedListEnd),
                                                                text,
                                                                &mut tokens,
                                                                &mut iter,
                                                                (c.0 + 1, '_')
                                                            ),
                                                            '\n' => (),
                                                            _ => tokens.push(Token::new_single(TokenType::Text, iter.index())),
                                                        }
                                                    }
                                                },
                                                '\n' => (),
                                                _ => tokens.push(Token::new_single(TokenType::Text, iter.index())),
                                            }
                                        }
                                    },
                                    _ => (),
                                }
                            },
                            None => {
                                if table.in_table {
                                    tokens.push(Token::new(TokenType::TableEnd, table.possible_table_start, c.0));
                                    tokens.insert(table.table_index, Token::new_single(TokenType::TableBegin, table.possible_table_start));
                                }
                                break;
                            },
                        }
                    },
                    '*'|'~'|'_' => markdown::match_emphasis(&mut emphasis, text, &mut tokens, &mut iter, c),
                    '|' => tokens.push(Token::new_single(TokenType::Pipe, c.0)),
                    '\t' => tokens.push(Token::new_single(TokenType::Tab, c.0)),
                    '\\' => tokens.push(Token::new_single(TokenType::Escape, c.0)),
                    _ => tokens.push(Token::new_single(TokenType::Text, c.0)),
                }
            },
            None => {
                if table.in_table {
                    tokens.push(Token::new(TokenType::TableEnd, table.possible_table_start, iter.index()));
                    tokens.insert(table.table_index, Token::new_single(TokenType::TableBegin, table.possible_table_start));
                }
                break;
            }
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
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
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
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
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
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
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
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
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
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
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
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::ItalicBegin
                |TokenType::BoldBegin|TokenType::UnderlineBegin|TokenType::StrikeBegin
                |TokenType::ItalicEnd|TokenType::BoldEnd|TokenType::UnderlineEnd|TokenType::StrikeEnd => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(bb == 3);
        assert!(be == 3);

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
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
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
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
                _ => panic!("Encounterd TokenType other than expected!"),
            }
        }
        assert!(cbb == 3);
        assert!(cbl == 3);
        assert!(cbe == 2);

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
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
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
                TokenType::Text|TokenType::Space|TokenType::Newline|TokenType::Heading
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
                TokenType::Text|TokenType::Space|TokenType::Newline => (),
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

    #[test]
    fn table() -> Result<(), io::Error> {
        let t = lex(&fs::read_to_string("tests/table.md")?);
        let mut p: usize = 0;
        let mut tb: usize = 0;
        let mut te: usize = 0;
        let mut cl: usize = 0;
        let mut cr: usize = 0;
        let mut cc: usize = 0;

        let mut ib: usize = 0;
        let mut ie: usize = 0;
        let mut bb: usize = 0;
        let mut be: usize = 0;
        let mut sb: usize = 0;
        let mut se: usize = 0;
        let mut ub: usize = 0;
        let mut ue: usize = 0;
        let mut c: usize = 0;

        let mut ia: usize = 0;
        let mut is: usize = 0;
        let mut lh: usize = 0;
        let mut lt: usize = 0;
        let mut ct: usize = 0;
        let mut cf: usize = 0;
        for token in t.iter() {
            match token.id {
                TokenType::Pipe => p += 1,
                TokenType::TableBegin => tb += 1,
                TokenType::TableEnd => te += 1,
                TokenType::TableColumnLeft => cl += 1,
                TokenType::TableColumnRight => cr += 1,
                TokenType::TableColumnCenter => cc += 1,

                TokenType::ItalicBegin => ib += 1,
                TokenType::ItalicEnd => ie += 1,
                TokenType::BoldBegin => bb += 1,
                TokenType::BoldEnd => be += 1,
                TokenType::StrikeBegin => sb += 1,
                TokenType::StrikeEnd => se += 1,
                TokenType::UnderlineBegin => ub += 1,
                TokenType::UnderlineEnd => ue += 1,
                TokenType::Code => c += 1,
                
                TokenType::ImageAlt => ia += 1,
                TokenType::ImageSrc => is += 1,
                TokenType::LinkHref => lh += 1,
                TokenType::LinkText => lt += 1,

                TokenType::Checkbutton(true) => ct += 1,
                TokenType::Checkbutton(false) => cf += 1,
                _ => (),
            }
        }
        assert!(p == 52);
        assert!(tb == 5);
        assert!(te == 5);
        assert!(cl == 9);
        assert!(cr == 2);
        assert!(cc == 2);

        assert!(ib == 2);
        assert!(ie == 2);
        assert!(bb == 2);
        assert!(be == 2);
        assert!(sb == 1);
        assert!(se == 1);
        assert!(ub == 1);
        assert!(ue == 1);
        assert!(c == 1);

        assert!(ia == 2);
        assert!(is == 2);
        assert!(lh == 2);
        assert!(lt == 2);
        assert!(ct == 1);
        assert!(cf == 1);
        
        Ok(())
    }

    // #[test]
    // fn list() -> Result<(), io::Error> {
    //     let t = lex(&fs::read_to_string("tests/list.md")?);
    //     for token in t.iter() {
    //         match token.id {
    //             TokenType::Text|TokenType::Space|TokenType::Newline => (),
    //             _ => panic!(format!("Encounterd TokenType other than expected! {:#?}", token)),
    //         }
    //     }

    //     Ok(())
    // }
}