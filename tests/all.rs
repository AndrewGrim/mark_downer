use std::io;

use markdowner;

#[test]
fn all() -> Result<(), io::Error> {
    let tokens: Vec<markdowner::Token> = markdowner::markdown_to_html("tests/all.md", "generated_html/all.html")?;
    let mut count: usize = 0;
    for t in tokens {
        match t.id {
            markdowner::TokenType::Heading|
            markdowner::TokenType::UnorderedListBegin|
            markdowner::TokenType::UnorderedListEnd|
            markdowner::TokenType::OrderedListBegin|
            markdowner::TokenType::OrderedListEnd|
            markdowner::TokenType::ListItemBegin|
            markdowner::TokenType::ListItemEnd|
            markdowner::TokenType::BlockquoteBegin|
            markdowner::TokenType::BlockquoteEnd|
            markdowner::TokenType::ItalicBegin|
            markdowner::TokenType::ItalicEnd|
            markdowner::TokenType::BoldBegin|
            markdowner::TokenType::BoldEnd|
            markdowner::TokenType::StrikeBegin|
            markdowner::TokenType::StrikeEnd|
            markdowner::TokenType::UnderlineBegin|
            markdowner::TokenType::UnderlineEnd|
            markdowner::TokenType::HorizontalRule|
            markdowner::TokenType::Code|
            markdowner::TokenType::CodeBlockBegin|
            markdowner::TokenType::CodeBlockLanguage|
            markdowner::TokenType::CodeBlockEnd|
            markdowner::TokenType::Checkbutton(true)|
            markdowner::TokenType::Checkbutton(false)|
            markdowner::TokenType::ImageAlt|
            markdowner::TokenType::ImageSrc|
            markdowner::TokenType::LinkHref|
            markdowner::TokenType::LinkText|
            markdowner::TokenType::TableBegin|
            markdowner::TokenType::TableEnd|
            markdowner::TokenType::TableColumnCenter|
            markdowner::TokenType::TableColumnLeft|
            markdowner::TokenType::TableColumnRight => count += 1,
            _ => (),
        }
    }
    assert!(count == 65);

    Ok(())
}