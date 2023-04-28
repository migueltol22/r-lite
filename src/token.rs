#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    // Keywords
    Exit,
    Insert,
    Select,
}
