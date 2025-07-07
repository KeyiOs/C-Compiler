use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Clone)]
pub struct Token {
    pub token_type: Option<TokenType>,
    pub line: usize,
    pub next: Option<Rc<RefCell<Token>>>,
    pub prev: Option<Rc<RefCell<Token>>>,
}


#[derive(Clone, Debug)]
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
    pub fn init() -> Rc<RefCell<Token>> {
        Rc::new(RefCell::new(Token {
            token_type: None,
            line: 0,
            next: None,
            prev: None,
        }))
    }

    pub fn set(
        token: Rc<RefCell<Token>>,
        token_type: TokenType,
        line: usize,
    ) -> Rc<RefCell<Token>> {
        let mut tok = token.borrow_mut();

        if tok.token_type.is_none() {
            tok.token_type = Some(token_type);
            tok.line = line;
            return Rc::clone(&token);
        } else {
            let new_token = Rc::new(RefCell::new(Token {
                token_type: Some(token_type),
                line,
                next: None,
                prev: Some(Rc::clone(&token)),
            }));

            tok.next = Some(Rc::clone(&new_token));
            return new_token;
        }
    }
}