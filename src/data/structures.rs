use crate::data::TokenType;


#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u16,
}

#[derive(Debug)]
pub struct ParserState<'a> {
    pub tokens: &'a Vec<Token>,
    pub iterator: usize
}