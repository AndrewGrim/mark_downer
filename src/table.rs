use crate::token;

#[derive(Debug, PartialEq)]
pub enum Alignment {
    Left,
    Right,
    Center,
    LeftOrCenter,
    LeftOrRight,
}

#[derive(Debug, PartialEq)]
pub struct Column(pub usize, pub String);

#[derive(Debug, PartialEq)]
pub struct Table {
    pub possible_table_start: usize,
    pub possible_table: bool,
    pub in_table: bool,
    pub table_index: usize,
    //pub pipes: Vec<token::Token>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            possible_table_start: 0,
            possible_table: false,
            in_table: false,
            table_index: 0,
        }
    }
}
