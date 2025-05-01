use crate::data_structures::keywords::SYMBOL_MAP;


pub fn lexer_start() {
    let plus_sign = '×';

    if symbol_exists(plus_sign) {
        println!("true");
    } else {
        println!("false");
    }
}


pub fn symbol_exists(c: char) -> bool {
    SYMBOL_MAP.contains_key(&c)
}
