use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;


pub enum _Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
    _Alignas,
    _Alignof,
    _Atomic,
    _Bool,
    _Complex,
    _Generic,
    _Imaginary,
    _Noreturn,
    _StaticAssert,
    _ThreadLocal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Ampersand,
    Asterisk,
    Backslash,
    Caret,
    Colon,
    Comma,
    Dot,
    DoubleQuote,
    Equal,
    Exclamation,
    GreaterThan,
    Hash,
    LessThan,
    Minus,
    ParenthesisLeft,
    ParenthesisRight,
    Percent,
    Pipe,
    Plus,
    Question,
    Semicolon,
    SingleQuote,
    Slash,
    SquareBracketLeft,
    SquareBracketRight,
    Tilde,
    Underscore,
    CurlyBracketLeft,
    CurlyBracketRight,
}

pub enum DoubleSymbol {
    DoubleAmpersand,
    DoubleMinus,
    DoublePipe,
    DoublePlus,
    DoubleSlash,
    Pointer,
    DoubleGreaterThan,
    DoubleLessThan,
    LessThanEqual,
    GreaterThanEqual,
    DoubleEqual,
    ExclamationEqual,
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    SlashEqual,
    PercentEqual,
    LeftShiftEqual,
    RightShiftEqual,
    AmpersandEqual,
    CaretEqual,
    PipeEqual,
}

pub enum _MathSymbol {
    Ampersand,
    Asterisk,
    Caret,
    Equal,
    GreaterThan,
    LessThan,
    Minus,
    Percent,
    Pipe,
    Plus,
    Slash,
}

pub static SYMBOL_MAP: Lazy<HashMap<char, Symbol>> = Lazy::new(|| {
    HashMap::from([
        ('&', Symbol::Ampersand),
        ('*', Symbol::Asterisk),
        ('\\', Symbol::Backslash),
        ('^', Symbol::Caret),
        (':', Symbol::Colon),
        (',', Symbol::Comma),
        ('.', Symbol::Dot),
        ('"', Symbol::DoubleQuote),
        ('=', Symbol::Equal),
        ('!', Symbol::Exclamation),
        ('>', Symbol::GreaterThan),
        ('#', Symbol::Hash),
        ('<', Symbol::LessThan),
        ('-', Symbol::Minus),
        ('(', Symbol::ParenthesisLeft),
        (')', Symbol::ParenthesisRight),
        ('%', Symbol::Percent),
        ('|', Symbol::Pipe),
        ('+', Symbol::Plus),
        ('?', Symbol::Question),
        (';', Symbol::Semicolon),
        ('\'', Symbol::SingleQuote),
        ('/', Symbol::Slash),
        ('[', Symbol::SquareBracketLeft),
        (']', Symbol::SquareBracketRight),
        ('~', Symbol::Tilde),
        ('_', Symbol::Underscore),
        ('{', Symbol::CurlyBracketLeft),
        ('}', Symbol::CurlyBracketRight),
    ])
});

pub static _MATH_SYMBOL_MAP: Lazy<HashMap<char, _MathSymbol>> = Lazy::new(|| {
    HashMap::from([
        ('&', _MathSymbol::Ampersand),
        ('*', _MathSymbol::Asterisk),
        ('^', _MathSymbol::Caret),
        ('=', _MathSymbol::Equal),
        ('>', _MathSymbol::GreaterThan),
        ('<', _MathSymbol::LessThan),
        ('-', _MathSymbol::Minus),
        ('%', _MathSymbol::Percent),
        ('|', _MathSymbol::Pipe),
        ('+', _MathSymbol::Plus),
        ('/', _MathSymbol::Slash),
    ])
});

pub static DOUBLE_SYMBOL_MAP: Lazy<HashMap<&str, DoubleSymbol>> = Lazy::new(|| {
    HashMap::from([
        ("&&", DoubleSymbol::DoubleAmpersand),
        ("--", DoubleSymbol::DoubleMinus),
        ("||", DoubleSymbol::DoublePipe),
        ("++", DoubleSymbol::DoublePlus),
        ("//", DoubleSymbol::DoubleSlash),
        ("->", DoubleSymbol::Pointer),
        (">>", DoubleSymbol::DoubleGreaterThan),
        ("<<", DoubleSymbol::DoubleLessThan),
        ("<=", DoubleSymbol::LessThanEqual),
        (">=", DoubleSymbol::GreaterThanEqual),
        ("==", DoubleSymbol::DoubleEqual),
        ("!=", DoubleSymbol::ExclamationEqual),
        ("+=", DoubleSymbol::PlusEqual),
        ("-=", DoubleSymbol::MinusEqual),
        ("*=", DoubleSymbol::AsteriskEqual),
        ("/=", DoubleSymbol::SlashEqual),
        ("%=", DoubleSymbol::PercentEqual),
        ("<<=", DoubleSymbol::LeftShiftEqual),
        (">>=", DoubleSymbol::RightShiftEqual),
        ("&=", DoubleSymbol::AmpersandEqual),
        ("^=", DoubleSymbol::CaretEqual),
        ("|=", DoubleSymbol::PipeEqual),
    ])
});

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TokenType {
    Keyword(String),
    Operator(char),
    Literal(String),
    Identifier(String),
    Punctuation(char),
    EOF,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub line: usize,
    pub next: Option<&'a Token<'a>>,
    pub prev: Option<&'a Token<'a>>,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Keyword(kw) => write!(f, "Keyword({})", kw),
            TokenType::Operator(op) => write!(f, "Operator({})", op),
            TokenType::Literal(lit) => write!(f, "Literal({})", lit),
            TokenType::Identifier(id) => write!(f, "Identifier({})", id),
            TokenType::Punctuation(punc) => write!(f, "Punctuation({})", punc),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}