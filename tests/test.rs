use std::fs;
use std::io;
use std::io::Write;

use markdowner;

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
    ];

    for test in test_files.iter() {
        let r = markdowner::markdown_to_html(format!("tests/{}.md", test).as_str(),
                                format!("generated_html/{}.html", test).as_str());

        match r {
            Ok(tokens) => log_tokens(tokens, test)?,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn log_tokens(tokens: Vec<markdowner::Token>, output: &str) -> Result<(), io::Error> {
    let mut log = fs::File::create(format!("log/{}.log", output.to_string()))?;
    log.write(format!("{:#?}", tokens).as_bytes())?;

    Ok(())
}