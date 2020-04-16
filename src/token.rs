enum TokenType {
    Newline,
    Tab,
    Space,
    Heading,
}

struct Token {
    id: TokenType,
    begin: usize,
    end: usize,
}

impl Token {
    fn new(id: TokenType, begin: usize, end: usize) -> Token {
        Token {
            id,
            begin,
            end,
        }
    }

    fn new_single(id: TokenType, begin: usize) -> Token {
        Token {
            id,
            begin,
            end: begin + 1,
        }
    }

    fn new_double(id: TokenType, begin: usize) -> Token {
        Token {
            id,
            begin,
            end: begin + 2,
        }
    }
}