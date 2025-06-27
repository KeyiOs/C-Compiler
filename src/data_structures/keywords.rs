use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;


#[allow(dead_code)]
pub enum Keyword {
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
    Include,
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


impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Keyword::Auto),
            "break" => Ok(Keyword::Break),
            "case" => Ok(Keyword::Case),
            "char" => Ok(Keyword::Char),
            "const" => Ok(Keyword::Const),
            "continue" => Ok(Keyword::Continue),
            "default" => Ok(Keyword::Default),
            "do" => Ok(Keyword::Do),
            "double" => Ok(Keyword::Double),
            "else" => Ok(Keyword::Else),
            "enum" => Ok(Keyword::Enum),
            "extern" => Ok(Keyword::Extern),
            "float" => Ok(Keyword::Float),
            "for" => Ok(Keyword::For),
            "goto" => Ok(Keyword::Goto),
            "if" => Ok(Keyword::If),
            "include" => Ok(Keyword::Include),
            "inline" => Ok(Keyword::Inline),
            "int" => Ok(Keyword::Int),
            "long" => Ok(Keyword::Long),
            "register" => Ok(Keyword::Register),
            "restrict" => Ok(Keyword::Restrict),
            "return" => Ok(Keyword::Return),
            "short" => Ok(Keyword::Short),
            "signed" => Ok(Keyword::Signed),
            "sizeof" => Ok(Keyword::Sizeof),
            "static" => Ok(Keyword::Static),
            "struct" => Ok(Keyword::Struct),
            "switch" => Ok(Keyword::Switch),
            "typedef" => Ok(Keyword::Typedef),
            "union" => Ok(Keyword::Union),
            "unsigned" => Ok(Keyword::Unsigned),
            "void" => Ok(Keyword::Void),
            "volatile" => Ok(Keyword::Volatile),
            "while" => Ok(Keyword::While),
            "_Alignas" => Ok(Keyword::_Alignas),
            "_Alignof" => Ok(Keyword::_Alignof),
            "_Atomic" => Ok(Keyword::_Atomic),
            "_Bool" => Ok(Keyword::_Bool),
            "_Complex" => Ok(Keyword::_Complex),
            "_Generic" => Ok(Keyword::_Generic),
            "_Imaginary" => Ok(Keyword::_Imaginary),
            "_Noreturn" => Ok(Keyword::_Noreturn),
            "_StaticAssert" => Ok(Keyword::_StaticAssert),
            "_ThreadLocal" => Ok(Keyword::_ThreadLocal),
            _ => Err(()),
        }
    }
}


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
        ('{', Symbol::CurlyBracketLeft),
        ('}', Symbol::CurlyBracketRight),
    ])
});


pub enum DoubleSymbol {
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
    LeftShiftEqual,
    RightShiftEqual,
    AmpersandEqual,
    CaretEqual,
    PipeEqual,
}


pub static DOUBLE_SYMBOL_MAP: Lazy<HashMap<&str, DoubleSymbol>> = Lazy::new(|| {
    HashMap::from([
        ("&&", DoubleSymbol::DoubleAmpersand),
        ("--", DoubleSymbol::DoubleMinus),
        ("||", DoubleSymbol::DoublePipe),
        ("++", DoubleSymbol::DoublePlus),
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


pub enum TripleSymbol {
    LeftShiftEqual,
    RightShiftEqual,
}


pub static TRIPLE_SYMBOL_MAP: Lazy<HashMap<&str, TripleSymbol>> = Lazy::new(|| {
    HashMap::from([
        ("<<=", TripleSymbol::LeftShiftEqual),
        (">>=", TripleSymbol::RightShiftEqual),
    ])
});