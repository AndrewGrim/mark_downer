use std::fs;
use std::io::Write;

use crate::token::Token;
use crate::token::TokenType;

pub fn parse(text: &String, tokens: &Vec<Token>) -> Vec<String> {
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
            TokenType::Escape => {
                if let Some(v) = iter.next() {
                    html.push(text[v.begin..v.end].to_string());
                } else {
                    break;
                }
            },
            TokenType::HorizontalRule => html.push("<hr>\n".to_string()),
            TokenType::Code => html.push(format!("<code>{}</code>", text[t.begin..t.end].to_string())),
            TokenType::IndentBlock => html.push(format!("<pre>{}</pre>", text[t.begin + 4..t.end].replace("\n    ", "\n"))),
            TokenType::ItalicBegin => html.push("<i>".to_string()),
            TokenType::ItalicEnd => html.push("</i>".to_string()),
            TokenType::BoldBegin => html.push("<b>".to_string()),
            TokenType::BoldEnd => html.push("</b>".to_string()),
            TokenType::StrikeBegin => html.push("<strike>".to_string()),
            TokenType::StrikeEnd => html.push("</strike>".to_string()),
            TokenType::UnderlineBegin => html.push("<u>".to_string()),
            TokenType::UnderlineEnd => html.push("</u>".to_string()),
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

pub fn generate_html(output_file: String, html: Vec<String>) {
    let mut file = fs::File::create(output_file).unwrap();
    file.write("<link rel=\"stylesheet\" href=\"default.css\">\n".as_bytes()).unwrap();
    file.write("<div class=\"markdown-body\">\n".as_bytes()).unwrap();
    for tag in html.iter() {
        file.write(tag.as_bytes()).unwrap();
    }
    file.write("\n</div>".as_bytes()).unwrap();
}

//TODO Add parser tests
