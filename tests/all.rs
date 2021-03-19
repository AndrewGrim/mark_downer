use std::io;

use markdown;

#[test]
fn all() -> Result<(), io::Error> {
    let tokens: Vec<markdown::Token> = markdown::markdown_to_html("tests/all.md", "generated_html/all.html", "css/light_theme.css")?;
    let mut count: usize = 0;
    for t in tokens {
        match t.id {
            markdown::TokenType::Heading|
            markdown::TokenType::UnorderedListBegin|
            markdown::TokenType::UnorderedListEnd|
            markdown::TokenType::OrderedListBegin|
            markdown::TokenType::OrderedListEnd|
            markdown::TokenType::ListItemBegin|
            markdown::TokenType::ListItemEnd|
            markdown::TokenType::BlockquoteBegin|
            markdown::TokenType::BlockquoteEnd|
            markdown::TokenType::ItalicBegin|
            markdown::TokenType::ItalicEnd|
            markdown::TokenType::BoldBegin|
            markdown::TokenType::BoldEnd|
            markdown::TokenType::StrikeBegin|
            markdown::TokenType::StrikeEnd|
            markdown::TokenType::UnderlineBegin|
            markdown::TokenType::UnderlineEnd|
            markdown::TokenType::HorizontalRule|
            markdown::TokenType::Code|
            markdown::TokenType::CodeBlockBegin|
            markdown::TokenType::CodeBlockLanguage|
            markdown::TokenType::CodeBlockEnd|
            markdown::TokenType::Checkbutton(true)|
            markdown::TokenType::Checkbutton(false)|
            markdown::TokenType::ImageAlt|
            markdown::TokenType::ImageSrc|
            markdown::TokenType::LinkHref|
            markdown::TokenType::LinkText|
            markdown::TokenType::TableBegin|
            markdown::TokenType::TableEnd|
            markdown::TokenType::TableColumnCenter|
            markdown::TokenType::TableColumnLeft|
            markdown::TokenType::TableColumnRight => count += 1,
            _ => (),
        }
    }
    assert!(count == 69);

    Ok(())
}