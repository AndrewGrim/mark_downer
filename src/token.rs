/// Enum describing all the possible tokens.
///
/// `TokenType::Checkbutton(bool)` is the only type to store a value.
/// True indicates that the checkbutton is checked.
#[derive(Debug, PartialEq, Copy)]
pub enum TokenType {
    Newline,
    Tab,
    Space,
    Heading,
    Hash,
    Text,
    Error,
    Checkbutton(bool),
    ImageAlt,
    ImageSrc,
    LinkText,
    LinkHref,
    HorizontalRule,
    BlockquoteBegin,
    BlockquoteEnd,
    Code,
    CodeBlockBegin,
    CodeBlockEnd,
    CodeBlockLanguage,
    CodeBlockText,
    CodeBlockString,
    CodeBlockChar,
    CodeBlockDigit,
    CodeBlockKeyword1,
    CodeBlockKeyword2,
    CodeBlockKeyword3,
    CodeBlockSymbol,
    CodeBlockFunction,
    CodeBlockSingleLineComment,
    CodeBlockMultiLineComment,
    CodeBlockEscape,
    IndentBlock,
    Escape,
    ItalicBegin,
    ItalicEnd,
    BoldBegin,
    BoldEnd,
    StrikeBegin,
    StrikeEnd,
    UnderlineBegin,
    UnderlineEnd,
    Pipe,
    PossibleTableStart,
    TableBegin,
    TableEnd,
    TableColumnLeft,
    TableColumnRight,
    TableColumnCenter,
    UnorderedListBegin,
    UnorderedListEnd,
    OrderedListBegin,
    OrderedListEnd,
    ListItemBegin,
    ListItemEnd,
    Html,
}

impl Clone for TokenType {
    fn clone(&self) -> Self {
        *self
    }
}

/// Struct that represents a lexer token.
#[derive(Debug, PartialEq)]
pub struct Token {
    pub id: TokenType,
    pub begin: usize,
    pub end: usize,
}

impl Token {
    /// Creates a new `Token` with specified `begin` and `end` indices.
    pub fn new(id: TokenType, begin: usize, end: usize) -> Token {
        Token {
            id,
            begin,
            end,
        }
    }

    /// Creates a new `Token` with the length of one char.
    pub fn new_single(id: TokenType, begin: usize) -> Token {
        Token {
            id,
            begin,
            end: begin + 1,
        }
    }

    /// Creates a new `Token` with the length of two chars.
    pub fn new_double(id: TokenType, begin: usize) -> Token {
        Token {
            id,
            begin,
            end: begin + 2,
        }
    }
}
