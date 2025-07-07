use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;


#[allow(dead_code)]
pub enum Keyword {
    Alignas,
    Alignof,
    Auto,
    Bool,
    Break,
    Case,
    Char,
    Const,
    Constexpr,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    False,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Nullptr,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    StaticAssert,
    Struct,
    Switch,
    ThreadLocal,
    True,
    Typedef,
    Typeof,
    TypeofUnqual,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
    _Atomic,
    _BitInt,
    _Complex,
    _Decimal128,
    _Decimal32,
    _Decimal64,
    _Generic,
    _Imaginary,
}

/*
    _Alignas (deprecated in C23)
    _Alignof (deprecated in C23)
    _Bool (deprecated in C23)
    _Noreturn (deprecated in C23)
    _Static_assert (deprecated in C23)
    _Thread_local (deprecated in C23)
*/

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alignas" => Ok(Keyword::Alignas),
            "alignof" => Ok(Keyword::Alignof),
            "auto" => Ok(Keyword::Auto),
            "bool" => Ok(Keyword::Bool),
            "break" => Ok(Keyword::Break),
            "case" => Ok(Keyword::Case),
            "char" => Ok(Keyword::Char),
            "const" => Ok(Keyword::Const),
            "constexpr" => Ok(Keyword::Constexpr),
            "continue" => Ok(Keyword::Continue),
            "default" => Ok(Keyword::Default),
            "do" => Ok(Keyword::Do),
            "double" => Ok(Keyword::Double),
            "else" => Ok(Keyword::Else),
            "enum" => Ok(Keyword::Enum),
            "extern" => Ok(Keyword::Extern),
            "false" => Ok(Keyword::False),
            "float" => Ok(Keyword::Float),
            "for" => Ok(Keyword::For),
            "goto" => Ok(Keyword::Goto),
            "if" => Ok(Keyword::If),
            "inline" => Ok(Keyword::Inline),
            "int" => Ok(Keyword::Int),
            "long" => Ok(Keyword::Long),
            "nullptr" => Ok(Keyword::Nullptr),
            "register" => Ok(Keyword::Register),
            "restrict" => Ok(Keyword::Restrict),
            "return" => Ok(Keyword::Return),
            "short" => Ok(Keyword::Short),
            "signed" => Ok(Keyword::Signed),
            "sizeof" => Ok(Keyword::Sizeof),
            "static" => Ok(Keyword::Static),
            "static_assert" => Ok(Keyword::StaticAssert),
            "struct" => Ok(Keyword::Struct),
            "switch" => Ok(Keyword::Switch),
            "thread_local" => Ok(Keyword::ThreadLocal),
            "true" => Ok(Keyword::True),
            "typedef" => Ok(Keyword::Typedef),
            "typeof" => Ok(Keyword::Typeof),
            "typeof_unqual" => Ok(Keyword::TypeofUnqual),
            "union" => Ok(Keyword::Union),
            "unsigned" => Ok(Keyword::Unsigned),
            "void" => Ok(Keyword::Void),
            "volatile" => Ok(Keyword::Volatile),
            "while" => Ok(Keyword::While),
            "_Atomic" => Ok(Keyword::_Atomic),
            "_BitInt" => Ok(Keyword::_BitInt),
            "_Complex" => Ok(Keyword::_Complex),
            "_Decimal128" => Ok(Keyword::_Decimal128),
            "_Decimal32" => Ok(Keyword::_Decimal32),
            "_Decimal64" => Ok(Keyword::_Decimal64),
            "_Generic" => Ok(Keyword::_Generic),
            "_Imaginary" => Ok(Keyword::_Imaginary),
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