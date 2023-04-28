use itertools::{Itertools, MultiPeek};
use std::collections::HashMap;
use std::str::Chars;

use crate::error::RLiteError;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: MultiPeek<Chars<'a>>,
    curr_buf: Vec<char>,
    line: u32,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().multipeek(),
            curr_buf: Vec::new(),
            line: 1,
            keywords: HashMap::from_iter([
                (String::from("exit"), TokenType::Exit),
                (String::from("insert"), TokenType::Insert),
                (String::from("select"), TokenType::Select),
            ]),
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, RLiteError> {
        todo!()
    }
}
