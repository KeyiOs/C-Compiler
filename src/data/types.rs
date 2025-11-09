#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Keyword(String),
    Operator(String),
    Literal(String),
    Identifier(String),
}


#[derive(Debug, Clone)]
pub enum AstNode {
    BinaryOperation {
        left: Box<AstNode>,
        operator: String,
        right: Box<AstNode>,
    },

    Value(String),
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
    Bool,
    Break,
    Case,
    Char,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    False,
    Float,
    For,
    If,
    Int,
    Long,
    Return,
    Short,
    Signed,
    Struct,
    Switch,
    True,
    Unsigned,
    Void,
    While,
}


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SingleOperator {
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
    CurlyBracketLeft,
    CurlyBracketRight,
}


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DoubleOperator {
    DoubleAmpersand,
    DoubleMinus,
    DoublePipe,
    DoublePlus,
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
    AmpersandEqual,
    CaretEqual,
    PipeEqual,
}


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TripleOperator {
    LeftShiftEqual,
    RightShiftEqual,
}