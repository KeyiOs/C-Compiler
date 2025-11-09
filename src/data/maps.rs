use std::collections::HashMap;
use std::sync::LazyLock;

use crate::data::types::{ SingleOperator, DoubleOperator, TripleOperator };


pub static SINGLE_OPERATOR_MAP: LazyLock<HashMap<char, SingleOperator>> = LazyLock::new(|| {
    HashMap::from([
        ('&', SingleOperator::Ampersand),
        ('*', SingleOperator::Asterisk),
        ('\\', SingleOperator::Backslash),
        ('^', SingleOperator::Caret),
        (':', SingleOperator::Colon),
        (',', SingleOperator::Comma),
        ('.', SingleOperator::Dot),
        ('"', SingleOperator::DoubleQuote),
        ('=', SingleOperator::Equal),
        ('!', SingleOperator::Exclamation),
        ('>', SingleOperator::GreaterThan),
        ('<', SingleOperator::LessThan),
        ('-', SingleOperator::Minus),
        ('(', SingleOperator::ParenthesisLeft),
        (')', SingleOperator::ParenthesisRight),
        ('%', SingleOperator::Percent),
        ('|', SingleOperator::Pipe),
        ('+', SingleOperator::Plus),
        ('?', SingleOperator::Question),
        (';', SingleOperator::Semicolon),
        ('\'', SingleOperator::SingleQuote),
        ('/', SingleOperator::Slash),
        ('[', SingleOperator::SquareBracketLeft),
        (']', SingleOperator::SquareBracketRight),
        ('~', SingleOperator::Tilde),
        ('{', SingleOperator::CurlyBracketLeft),
        ('}', SingleOperator::CurlyBracketRight),
    ])
});


pub static DOUBLE_OPERATOR_MAP: LazyLock<HashMap<&str, DoubleOperator>> = LazyLock::new(|| {
    HashMap::from([
        ("&&", DoubleOperator::DoubleAmpersand),
        ("--", DoubleOperator::DoubleMinus),
        ("||", DoubleOperator::DoublePipe),
        ("++", DoubleOperator::DoublePlus),
        ("->", DoubleOperator::Pointer),
        (">>", DoubleOperator::DoubleGreaterThan),
        ("<<", DoubleOperator::DoubleLessThan),
        ("<=", DoubleOperator::LessThanEqual),
        (">=", DoubleOperator::GreaterThanEqual),
        ("==", DoubleOperator::DoubleEqual),
        ("!=", DoubleOperator::ExclamationEqual),
        ("+=", DoubleOperator::PlusEqual),
        ("-=", DoubleOperator::MinusEqual),
        ("*=", DoubleOperator::AsteriskEqual),
        ("/=", DoubleOperator::SlashEqual),
        ("%=", DoubleOperator::PercentEqual),
        ("&=", DoubleOperator::AmpersandEqual),
        ("^=", DoubleOperator::CaretEqual),
        ("|=", DoubleOperator::PipeEqual),
    ])
});


pub static TRIPLE_OPERATOR_MAP: LazyLock<HashMap<&str, TripleOperator>> = LazyLock::new(|| {
    HashMap::from([
        ("<<=", TripleOperator::LeftShiftEqual),
        (">>=", TripleOperator::RightShiftEqual),
    ])
});