//! A library for pasing markdown and outputting equivalent html.
//!
//! Note that this lib does not follow the popular specifications
//! like `CommonMark`. 
//!
//! This is because it was mostly written for fun
//! and to learn more about Rust and software in general. It is
//! actively used by a personal project of mine.

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

/// Converts a markdown file to an html file.
///
/// If you don't want to embed any css into the generated html,
/// you can just pass an empty &str.
///
/// Returns a vector of tokens if successful.
/// The tokens can be used for syntax highlighting using the `begin` and `end` indices.
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
    use std::path::Path;
    use std::fs::create_dir;

    #[test]
    fn log() -> Result<(), io::Error> {
        if !Path::new("generated_html").exists() {
            create_dir("generated_html").unwrap();
        }
        if !Path::new("log").exists() {
            create_dir("log").unwrap();
        }
        
    
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
            "md_readme",
            "mh_readme",
            "nm_readme",
            "eli5_readme",
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
