use crate::token;

#[derive(Debug, PartialEq)]
pub struct Table {
    pub possible_table_start: usize,
    pub possible_table: bool,
    pub in_table: bool,
    pub pipes: Vec<token::Token>,
}
