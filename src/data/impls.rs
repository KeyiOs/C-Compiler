use crate::Token;
use crate::data::{ Keyword, TokenType };
use std::str::FromStr;
use phf::phf_map;

impl Token {
    pub fn new(token_type: TokenType, line: u16) -> Self {
        Self { token_type, line }
    }
}


impl TokenType {
    pub fn value(&self) -> &str {
        match self {
            TokenType::Keyword(s)
            | TokenType::Operator(s)
            | TokenType::Literal(s)
            | TokenType::Identifier(s) => s.as_str(),
        }
    }
}


impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
            "bool" => Keyword::Bool,
            "break" => Keyword::Break,
            "case" => Keyword::Case,
            "char" => Keyword::Char,
            "continue" => Keyword::Continue,
            "default" => Keyword::Default,
            "do" => Keyword::Do,
            "double" => Keyword::Double,
            "else" => Keyword::Else,
            "enum" => Keyword::Enum,
            "false" => Keyword::False,
            "float" => Keyword::Float,
            "for" => Keyword::For,
            "if" => Keyword::If,
            "int" => Keyword::Int,
            "long" => Keyword::Long,
            "return" => Keyword::Return,
            "short" => Keyword::Short,
            "signed" => Keyword::Signed,
            "struct" => Keyword::Struct,
            "switch" => Keyword::Switch,
            "true" => Keyword::True,
            "unsigned" => Keyword::Unsigned,
            "void" => Keyword::Void,
            "while" => Keyword::While,
        };

        KEYWORDS.get(s).copied().ok_or(())
    }
}