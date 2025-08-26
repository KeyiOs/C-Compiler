use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;


/* * * * * * * * * */
/*      TOKEN      */
/* * * * * * * * * */
#[allow(unused)]
#[derive(Clone)]
pub struct Token {
    pub token_type: Option<TokenType>,
    pub line: usize,
    pub next: Option<Rc<RefCell<Token>>>,
    pub prev: Option<Rc<RefCell<Token>>>,
}


#[allow(unused)]
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

    pub fn get_string(&self) -> Option<&str> {
        self.token_type.as_ref().map(|t_type| t_type.get_string())
    }

    pub fn next_token(rc: &Rc<RefCell<Token>>) -> Rc<RefCell<Token>> {
        rc.borrow().next.as_ref().unwrap().clone()
    }
}


#[allow(unused)]
#[derive(Clone, Debug)]
pub enum TokenType {
    Keyword(String),
    Operator(String),
    Literal(String),
    Identifier(String),
}


#[allow(unused)]
impl TokenType {
    pub fn get_string(&self) -> &str {
        match self {
            TokenType::Keyword(s) => s.as_str(),
            TokenType::Operator(s) => s.as_str(),
            TokenType::Literal(s) => s.as_str(),
            TokenType::Identifier(s) => s.as_str(),
        }
    }

    pub fn is(&self, s: &str) -> bool {
        matches!(self, TokenType::Operator(op) if op == s)
    }
}


#[allow(unused)]
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


/* * * * * * * * * * * * */
/* Abstract Syntax Tree  */
/* * * * * * * * * * * * */
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Declaration {
    pub class: Class,
    pub name: String,
    pub type_info: Type,
    pub initializer: Option<String>,
}


impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Class: \"{}\", Name: \"{}\", Type: \"{}\"", self.class, self.name, self.type_info)?;
        if let Some(init) = &self.initializer {
            write!(f, " = {}", init)?;
        }
        Ok(())
    }
}


#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Class {
    Typedef,
    Variable,
    Function,
}


impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Class::Typedef => write!(f, "typedef"),
            Class::Variable => write!(f, "variable"),
            Class::Function => write!(f, "function"),
        }
    }
}


#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Type {
    Primitive {
        ttype: BaseType,
    }, Function {
        return_type: String,
        parameters: Vec<String>,
    },
}


impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Primitive { ttype } => write!(f, "{}", ttype),
            Type::Function { return_type, parameters } => {
                write!(f, "{}(", return_type)?;
                for (i, param) in parameters.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ")")
            }
        }
    }
}


#[allow(unused)]
#[derive(Debug, Clone)]
pub enum BaseType {
    Int,
    Float,
    Char,
    Void,
    Other(String),
}


impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaseType::Int => write!(f, "int"),
            BaseType::Float => write!(f, "float"),
            BaseType::Char => write!(f, "char"),
            BaseType::Void => write!(f, "void"),
            BaseType::Other(s) => write!(f, "{}", s),
        }
    }
}