use std::iter;
use std::str;

use crate::position;
use crate::token;

#[derive(Debug)]
pub struct CharsWithPosition<'a> {
    pos: position::Position,
    iter: iter::Peekable<iter::Enumerate<str::Chars<'a>>>,
}

impl<'a> CharsWithPosition<'a> {
    pub fn new(pos: position::Position, iter: iter::Peekable<iter::Enumerate<str::Chars>>) -> CharsWithPosition {
        CharsWithPosition {
            pos,
            iter,
        }
    }

    pub fn next(&mut self) -> Option<(usize, char)> {
        self.pos.increment();
        self.iter.next()
    }

    pub fn peek(&mut self) -> Option<&(usize, char)> {
        self.iter.peek()
    }

    pub fn nth(&mut self, n: usize) -> Option<(usize, char)> {
        self.pos.index += n + 1;
        self.iter.nth(n)
    }

    pub fn index(&self) -> usize {
        self.pos.index
    }

    pub fn last(&self) -> usize {
        self.pos.index - 1
    }
}

#[derive(Debug)]
pub struct List(pub token::TokenType, pub usize);

#[derive(Debug)]
pub struct ListType(pub token::TokenType, pub token::TokenType);
