use std::fmt;


#[derive(Clone)]
pub struct Token {
    pub token_type: Option<TokenType>,
    pub line: usize,
    pub next: Option<Box<Token>>,
    pub prev: Option<Box<Token>>,
}


#[derive(Clone)]
pub enum TokenType {
    Keyword(String),
    Operator(String),
    Literal(String),
    Identifier(String),
}


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Keyword(kw) => write!(f, "Keyword({})", kw),
            TokenType::Operator(op) => write!(f, "Operator({})", op),
            TokenType::Literal(lit) => write!(f, "Literal({})", lit),
            TokenType::Identifier(id) => write!(f, "Identifier({})", id),
        }
    }
}


impl Token {
    pub fn init() -> Token {
        Token {
            token_type: None,
            line: 0,
            next: None,
            prev: None,
        }
    }

    pub fn set(
        token: &mut Token,
        token_type: TokenType,
        line: usize,
    ) -> &mut Token {
        if token.token_type.is_none() {
            token.token_type = Some(token_type);
            token.line = line;
            token.prev = Some(Box::new(Token::init()));
            return token;
        } else {
            let mut token_new = Token::init();
            token_new.token_type = Some(token_type);
            token_new.line = line;
            token_new.prev = token.prev.clone();

            token.next = Some(Box::new(token_new));
            return token.next.as_mut().unwrap();
        }
    }
}