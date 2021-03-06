#[derive(Debug, PartialEq)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub line_offset: usize,
}

impl Position {
    pub fn new() -> Position {
        Position {
            index: 0,
            line: 0,
            line_offset: 0,
        }
    }

    pub fn new_at(index: usize, line: usize, line_offset: usize) -> Position {
        Position {
            index,
            line,
            line_offset,
        }
    }

    pub fn update(&mut self, position: Position) {
        self.index = position.index;
        self.line = position.line;
        self.line_offset = position.line_offset;
    }

    pub fn increment(&mut self) {
        self.index += 1;
        self.line_offset += 1;
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.line_offset = 0;
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn set_line(&mut self, line: usize) {
        self.line = line;
    }

    pub fn set_line_offset(&mut self, line_offset: usize) {
        self.line_offset = line_offset;
    }
}
