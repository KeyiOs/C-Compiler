use crate::data_structures::keywords::{Keyword, DOUBLE_SYMBOL_MAP, SYMBOL_MAP};
use crate::data_structures::objects::{Token, TokenType};
use std::iter::Peekable;
use std::str::{Chars, FromStr};


/* * * * * * * * * * * * * * * * * * * */
/*               TODO                  */
/*                                     */
/* Finish Preprocessor # Line Markers  */
/* & Add File name into Error messages */
/* Header Files                        */
/* * * * * * * * * * * * * * * * * * * */


pub fn lexer_start(token_head: &mut Token, source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut token = token_head;
    let mut chars = source.chars().peekable();
    let mut buffer = String::new();
    let mut line = 1;
    let mut start_of_line = true;

    while let Some(character) = chars.next() {
        if character.is_ascii_alphabetic() || character == '_' {
            buffer.push(character);

            while let Some(&next_char) = chars.peek() {
                if next_char.is_ascii_alphanumeric() || next_char == '_' {
                    buffer.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            if Keyword::from_str(buffer.as_str()).is_ok() {
                token = Token::set(token, TokenType::Keyword(buffer.clone()), 0);
            } else if buffer.chars().all(|c| c.is_alphanumeric() || c == '_') {
                token = Token::set(token, TokenType::Identifier(buffer.clone()), 0);
            }
            
            buffer.clear();
        } else if character.is_numeric() {
            buffer.push(character);
            
            while let Some(&next_char) = chars.peek() {
                if next_char.is_ascii_digit() || next_char == '.' {
                    buffer.push(chars.next().unwrap());
                } else if next_char == 'e' || next_char == 'E' {
                    buffer.push(chars.next().unwrap());

                    if let Some(&sign) = chars.peek() {
                        if sign == '+' || sign == '-' {
                            buffer.push(chars.next().unwrap());
                        }
                    }

                    let mut found_digit = false;
                    while let Some(&exp_digit) = chars.peek() {
                        if exp_digit.is_ascii_digit() {
                            found_digit = true;
                            buffer.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if !found_digit {
                        return Err(format!("Invalid exponent in number on line {}", line).into());
                    }
                } else if next_char.is_ascii_alphabetic() {
                    let suffix_char = chars.next().unwrap();
                    let suffix_char_lower = suffix_char.to_ascii_lowercase();
                    if matches!(suffix_char_lower, 'u' | 'l' | 'f') {
                        buffer.push(suffix_char);

                        if let Some(&next_suffix) = chars.peek() {
                            let next_suffix_lower = next_suffix.to_ascii_lowercase();
                            if (suffix_char_lower == 'u' && (next_suffix_lower == 'l')) ||
                            (suffix_char_lower == 'l' && (next_suffix_lower == 'u')) ||
                            (suffix_char_lower == 'l' && (next_suffix_lower == 'f')) {
                                buffer.push(chars.next().unwrap());
                            }
                        }
                    } else {
                        return Err(format!("Invalid suffix in number on line {}", line).into());
                    }

                    break;
                } else {
                    break;
                }
            }

            token = Token::set(token, TokenType::Literal(buffer.clone()), 0);
            buffer.clear();
        } else if character.is_whitespace() {
            if character == '\n' {
                line += 1;
                start_of_line = true;
            }

            continue;
        } else if SYMBOL_MAP.contains_key(&character) {
            if character == '\'' {
                if let Some(character) = chars.next() {
                    if character == '\'' {
                        return Err(format!("Empty character literal on line {}", line).into());
                    }





















                    if chars.next() == Some('\'') {
                        token = Token::set(token, TokenType::Literal(next_char.to_string()), 0);
                        ok = true;
                        break;
                    } else {
                        while let Some(next_char) = chars.next() {
                            if next_char == '\n' {
                                return Err(format!("Unterminated character literal on line {}", line).into());
                            } else if next_char == '\'' {
                                return Err(format!("Character literal too long on line {}", line).into());
                            }
                        }

                        return Err(format!("Unterminated character literal on line {}", line).into());
                    }



















                    




                } else {
                    return Err(format!("Unterminated character literal on line {}", line).into());
                }
            } else if character == '"' {
                let mut string_lit = String::new();
                let mut ok = false;

                while let Some(next_char) = chars.next() {
                    match next_char {
                        '"' => {
                            let mut lookahead = chars.clone();

                            while let Some(&c) = lookahead.peek() {
                                if c == '\n' || c == '\r' {
                                    return Err(format!("Unterminated string literal on line {}", line).into());
                                }

                                if c.is_whitespace() {
                                    lookahead.next();
                                } else {
                                    break;
                                }
                            }

                            if let Some(&'"') = lookahead.peek() {
                                for _ in 0..(chars.clone().count() - lookahead.clone().count()) {
                                    chars.next();
                                }
                                chars.next();
                                continue;
                            }

                            token = Token::set(token, TokenType::Literal(string_lit), 0);
                            ok = true;
                            break;
                        }
                        '\\' => {
                            let Some(escaped) = chars.next() else {
                                return Err(format!("Unmatched escape sequence on line {}", line).into());
                            };

                            if escaped == '\n' || escaped == '\r' {
                                chars.next();
                                continue;
                            }

                            match escaped {
                                'a' => string_lit.push('\x07'),
                                'b' => string_lit.push('\x08'),
                                'f' => string_lit.push('\x0C'),
                                'n' => string_lit.push('\n'),
                                'r' => string_lit.push('\r'),
                                't' => string_lit.push('\t'),
                                'v' => string_lit.push('\x0B'),
                                '\\' => string_lit.push('\\'),
                                '\'' => string_lit.push('\''),
                                '"'  => string_lit.push('"'),
                                
                                'x' => {
                                    let mut hex_digits = String::new();

                                    for _ in 0..2 {
                                        if let Some(&c) = chars.peek() {
                                            if c.is_ascii_hexdigit() {
                                                hex_digits.push(c);
                                                chars.next();
                                            } else {
                                                break;
                                            }
                                        }
                                    }

                                    if hex_digits.is_empty() {
                                        return Err(format!("Expected hex digits after \\x on line {}", line).into());
                                    }

                                    let value = u8::from_str_radix(&hex_digits, 16)
                                        .map_err(|_| format!("Invalid hex escape: \\x{} on line {}", hex_digits, line))?;
                                    string_lit.push(value as char);

                                    println!("Hex escape: \\x{} -> {}", hex_digits, value);
                                }

                                'u' => {
                                    if let Some('{') = chars.next() {
                                        let mut unicode_digits = String::new();

                                        while let Some(&c) = chars.peek() {
                                            if c == '}' {
                                                chars.next(); // consume '}'
                                                break;
                                            } else if c.is_ascii_hexdigit() {
                                                unicode_digits.push(c);
                                                chars.next();
                                            } else {
                                                return Err(format!("Invalid character '{}' in unicode escape on line {}", c, line).into());
                                            }
                                        }

                                        if unicode_digits.is_empty() {
                                            return Err(format!("Empty unicode escape on line {}", line).into());
                                        }

                                        let codepoint = u32::from_str_radix(&unicode_digits, 16)
                                            .map_err(|_| format!("Invalid unicode escape: \\u{{{}}} on line {}", unicode_digits, line))?;

                                        if let Some(ch) = char::from_u32(codepoint) {
                                            string_lit.push(ch);
                                        } else {
                                            return Err(format!("Invalid unicode codepoint: \\u{{{}}} on line {}", unicode_digits, line).into());
                                        }
                                    } else {
                                        return Err(format!("Expected '{{' after \\u on line {}", line).into());
                                    }
                                }

                                '0'..='7' => {
                                    let mut oct_digits = String::new();
                                    oct_digits.push(escaped); // first digit

                                    for _ in 0..2 {
                                        if let Some(&c) = chars.peek() {
                                            if c >= '0' && c <= '7' {
                                                chars.next();
                                                oct_digits.push(c);
                                            } else {
                                                break;
                                            }
                                        }
                                    }

                                    let value = u8::from_str_radix(&oct_digits, 8)
                                        .map_err(|_| format!("Invalid octal escape: \\{} on line {}", oct_digits, line))?;
                                    string_lit.push(value as char);
                                }
                                
                                _ => return Err(format!("Invalid escape sequence: \\{} on line {}", escaped, line).into()),
                            }
                        }

                        '\n' => return Err(format!("Unterminated string literal on line {}", line).into()),

                        _ => string_lit.push(next_char),
                    }
                }

                if !ok {
                    return Err(format!("Unterminated string literal on line {}", line).into());
                }
            } else if let Some(&next_char) = chars.peek() {
                let double = [character, next_char].iter().collect::<String>();

                if double == "//" {
                    while let Some(&ch) = chars.peek() {
                        if ch == '\n' {
                            line += 1;
                            break;
                        }
                        
                        chars.next();
                    }

                    continue;
                } else if double == "/*" {
                    while let Some(ch) = chars.next() {
                        if ch == '*' {
                            if let Some(&next) = chars.peek() {
                                if next == '/' {
                                    chars.next();
                                    break;
                                }
                            }
                        } else if ch == '\n' {
                            line += 1;
                        } else if ch == '\0' {
                            return Err(format!("Unterminated multiline comment on line {}", line).into());
                        }
                    }
                    continue;
                }

                let mut char_iter = chars.clone();
                char_iter.next();

                if let Some(third_char) = char_iter.next() {
                    let triple = format!("{}{}{}", character, next_char, third_char);
                    if DOUBLE_SYMBOL_MAP.contains_key(triple.as_str()) {
                        token = Token::set(token, TokenType::Operator(triple), line);
                        chars.next();
                        chars.next();
                        continue;
                    }
                }

                if DOUBLE_SYMBOL_MAP.contains_key(double.as_str()) {
                    token = Token::set(token, TokenType::Operator(double), 0);
                    chars.next();
                } else {
                    token = Token::set(token, TokenType::Operator(character.to_string()), 0);
                }
            } else {
                token = Token::set(token, TokenType::Operator(character.to_string()), 0);
            }
        } else if character == '#' && start_of_line {
            while let Some(c) = chars.next() {
                if c == '\n' || c == '\r' {
                    chars.next();
                    break;
                }
            }

            continue;
        } else {
            return Err(format!("Unknown character on line {}: {}", line, character).into());
        }

        start_of_line = false;
    };

    Ok(())
}