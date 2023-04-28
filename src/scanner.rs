use itertools::{Itertools, MultiPeek};
use std::collections::HashMap;
use std::str::Chars;

use crate::error::RLiteError;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: MultiPeek<Chars<'a>>,
    curr_buf: Vec<char>,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().multipeek(),
            curr_buf: Vec::new(),
            keywords: HashMap::from_iter([
                (String::from(".exit"), TokenType::Exit),
                (String::from("insert"), TokenType::Insert),
                (String::from("select"), TokenType::Select),
            ]),
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, RLiteError> {
        let mut tokens = Vec::new();
        loop {
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            } else {
                break;
            }
        }
        tokens.push(Token::new(TokenType::Eof));
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance()?;
        match c {
            ' ' | '\r' | '\t' => None,
            '\n' => Some(Token::new(TokenType::Eof)),
            _ => {
                if c == '.' || c.is_alphabetic() {
                    self.identifier()
                } else {
                    Some(Token::new(TokenType::Eof))
                }
            }
        }
    }

    fn identifier(&mut self) -> Option<Token> {
        while let Some(&c) = self.source.peek() {
            if c.is_alphabetic() {
                self.advance()?;
            } else {
                break;
            }
        }
        let identifier = self.curr_buf.iter().collect::<String>();
        let token_type = self.keywords.get(&identifier).unwrap_or(&TokenType::Eof);
        Some(Token::new(*token_type))
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.next()?;
        self.curr_buf.push(c);
        Some(c)
    }
}
