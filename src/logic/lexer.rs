use crate::data_structures::keywords::{DOUBLE_SYMBOL_MAP, SYMBOL_MAP};
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
            buffer.push(character);

            if let Some(&next_char) = chars.peek() {
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
            if let Some(&next_char) = chars.peek() {
                if DOUBLE_SYMBOL_MAP.contains_key(format!("{}{}", character, next_char).as_str()) {
                    if let Some(third_char) = chars.clone().skip(1).next() {
                        let triple = format!("{}{}{}", character, next_char, third_char);
        
                        if DOUBLE_SYMBOL_MAP.contains_key(triple.as_str()) {
                            println!("{}{}{}", character, next_char, third_char);
                            chars.next();
                            chars.next();
                            continue;
                        }
                    }
                    println!("{}", format!("{}{}", character, next_char));
                    chars.next();
                } else {
                    println!("{}", character);
                }
            } else {
                println!("{}", character);
            }
        } else {
            println!("Unknown: {}", character);
        }
    };

    Ok(())
}
