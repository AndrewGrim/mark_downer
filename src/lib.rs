#![forbid(unsafe_code)]
#![allow(dead_code, unused_variables)] // TODO remove this once ready

use std::fs;
use std::io;

mod position;
mod token;
mod emphasis;
mod markdown;
mod lexer;
mod parser;
mod table;
mod wrapper;
mod syntax;

pub use token::Token;
pub use token::TokenType;

pub fn markdown_to_html(input: &str, output: &str, css: &str) -> Result<Vec<Token>, io::Error> {
    let text: String = fs::read_to_string(input)?;
    let tokens = lexer::lex(&text);
    let html = parser::parse(&text, &tokens);
    parser::generate_html(output.to_string(), html, css);

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn log() -> Result<(), io::Error> {
        let test_files = [
            "heading",
            "checkbutton",
            "image",
            "link",
            "horizontalrule",
            "blockquote",
            "code",
            "codeblock",
            "indentblock",
            "escape",
            "emphasis",
            "html",
            "table",
            "list",
            "all",
        ];

        for test in test_files.iter() {
            let r = markdown_to_html(format!("tests/{}.md", test).as_str(),
                                    format!("generated_html/{}.html", test).as_str(), "css/light_theme.css");

            match r {
                Ok(tokens) => log_tokens(tokens, test)?,
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn log_tokens(tokens: Vec<Token>, output: &str) -> Result<(), io::Error> {
        let mut log = fs::File::create(format!("log/{}.log", output.to_string()))?;
        log.write(format!("{:#?}", tokens).as_bytes())?;

        Ok(())
    }
}
