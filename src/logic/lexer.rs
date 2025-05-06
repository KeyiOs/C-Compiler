use crate::data_structures::keywords::SYMBOL_MAP;
use std::fs;


pub fn lexer_start() -> Result<(), Box<dyn std::error::Error>> {
    let path: &str = "././examples/reverseString.c";
    let input: String = fs::read_to_string(&path)?;
    let mut chars = input.chars().peekable();
    let mut buffer = String::new();

    while let Some(character) = chars.next() {
        if character.is_alphabetic() {
            buffer.push(character);

            if let Some(&next_char) = chars.peek() {
                if next_char.is_whitespace() || SYMBOL_MAP.contains_key(&next_char) {
                    println!("{}", buffer.as_str());
                    buffer.clear();
                }
            }
        } else if character.is_numeric() {
            if let Some(&next_char) = chars.peek() {
                buffer.push(character);
                
                if next_char.is_numeric() {
                    continue;
                } else if next_char.is_whitespace() || SYMBOL_MAP.contains_key(&next_char) {
                    println!("{}", buffer.as_str());
                    buffer.clear();
                } else {
                    println!("Lexical error. Expected a number, found: {}", next_char);
                }
            }
        } else if character.is_whitespace() {
            continue;
        } else if SYMBOL_MAP.contains_key(&character) {
            println!("{}", character);
        } else {
            println!("Unknown: {}", character);
        }
    };

    set_token();

    Ok(())
}


fn set_token() {
    
}
