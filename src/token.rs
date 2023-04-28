#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self { token_type }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Keywords
    Exit,
    Insert,
    Select,
    Eof,
}
