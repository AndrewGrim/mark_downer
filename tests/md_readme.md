# Mark Downer
This is an ok markdown parser written for fun, learning and personal use.

If for some reason you want to use it:
Know that it doesn't support most of the "official" specifications.
It can also be pretty strict when it comes to what certain elements are declared like.

For a simple example of all elements have a look at the [this](tests/all.md) file.

## Features
### Supported elements
* blockquote
* checkbutton
* code
* codeblock (syntax highlighting is supported using syntax files)
* emphasis (italic, bold, underline, strikethrough)
* escape (next lexer token is skipped)
* heading
* inline html
* image
* indentblock
* link
* list
* table

### Syntax Highlighting
All codeblocks for which the program can't find syntax files have base highlighting applied.
This includes coloring digits, strings, chars, symbols and functions (identifiers followed by '(').

Syntax files for these languages are provided:
* C
* D
* Python
* Rust

### Styling
The custom made `.scss` file can be found in [this](https://github.com/AndrewGrim/mark_downer_style) repo.

## How To Build
This should work on any Rust installations that support the 2018 edition.
Simply run `cargo build --release`.
This will build both the library and the binary.

To run all the tests use `cargo test`. This will also generated the html for all markdown test files.

### Using The Library
The following codeblock is the entire public interface at the moment. It can be found in the [lib.rs](src/lib.rs) file.

```rust
pub use token::Token;
pub use token::TokenType;

pub fn markdown_to_html(input: &str, output: &str) -> Result<Vec<Token>, io::Error> {
    let text: String = fs::read_to_string(input)?;
    let tokens = lexer::lex(&text);
    let html = parser::parse(&text, &tokens);
    parser::generate_html(output.to_string(), html);

    Ok(tokens)
}
```

### Using The Binary
    md <input> <output>
`input` - The markdown file to be translated to html.

`output` - The path to where the translated file will be saved.

