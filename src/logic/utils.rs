pub trait CharExt {
    fn is_dot(self) -> bool;
    fn is_sci_notation(self) -> bool;
    fn is_suffix_char(self) -> bool;
    fn is_char_literal(self) -> bool;
    fn is_string_literal(self) -> bool;
}


impl CharExt for char {
    fn is_dot(self) -> bool {
        self == '.'
    }

    fn is_sci_notation(self) -> bool {
        self == 'e' || self == 'E'
    }    

    fn is_suffix_char(self) -> bool {
        self == 'u' || self == 'l' || self == 'f'
    }

    fn is_char_literal(self) -> bool {
        self == '\''
    }

    fn is_string_literal(self) -> bool {
        self == '"'
    }
}


pub trait StrExt {
    fn is_num_suffix(&self) -> bool;
}


impl StrExt for str {
    fn is_num_suffix(&self) -> bool {
        matches!(
            self.to_ascii_lowercase().as_str(),
            "u" | "l" | "f" | "ul" | "lu" | "ll" | "ull" | "llu" | "lf"
        )
    }
}