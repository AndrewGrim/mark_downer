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
    CodeBlockKeyword,
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
}

impl Clone for TokenType {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub id: TokenType,
    pub begin: usize,
    pub end: usize,
}

impl Token {
    pub fn new(id: TokenType, begin: usize, end: usize) -> Token {
        Token {
            id,
            begin,
            end,
        }
    }

    pub fn new_single(id: TokenType, begin: usize) -> Token {
        Token {
            id,
            begin,
            end: begin + 1,
        }
    }

    pub fn new_double(id: TokenType, begin: usize) -> Token {
        Token {
            id,
            begin,
            end: begin + 2,
        }
    }
}
