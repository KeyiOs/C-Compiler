use crate::data_structures::keywords::SYMBOL_MAP;
use std::fs;

pub fn lexer_start() -> Result<(), Box<dyn std::error::Error>> {
    let path: &str = "././examples/reverseString.c";
    let content: String = fs::read_to_string(&path)?;
    println!("File content: {}", content);

    test();

    Ok(())
}


fn symbol_exists(c: char) -> bool {
    SYMBOL_MAP.contains_key(&c)
}


fn test() {
    let plus_sign = '×';

    if symbol_exists(plus_sign) {
        println!("true");
    } else {
        println!("false");
    }
}