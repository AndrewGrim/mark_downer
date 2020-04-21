#[derive(Debug, PartialEq)]
pub enum Tag {
    Italic(bool),
    Bold(bool),
    Strike(bool),
    Underline(bool),
}

#[derive(Debug, PartialEq)]
pub struct State {
    pub italic: Tag,
    pub bold: Tag,
    pub strike: Tag,
    pub underline: Tag,
}

impl State {
    pub fn new() -> State {
        State {
            italic: Tag::Italic(false),
            bold: Tag::Bold(false),
            strike: Tag::Strike(false),
            underline: Tag::Underline(false),
        }
    }
}