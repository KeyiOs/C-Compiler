use crate::data_structures::keywords::{Keyword, DOUBLE_SYMBOL_MAP, SYMBOL_MAP, TRIPLE_SYMBOL_MAP};
use crate::data_structures::objects::{Token, TokenType};
use crate::logic::utils::{CharExt, StrExt};
use std::str::{Chars, FromStr};
use std::iter::Peekable;


/* * * * * * * * * * * * * * * * * * * */
/*               TODO                  */
/* * * * * * * * * * * * * * * * * * * */
/* Add File name into Error messages   */
/* Remove while let Some() Nesting     */
/* * * * * * * * * * * * * * * * * * * */


pub fn lexer_start(token_head: &mut Token, source: &str) -> Result<(), Box<dyn std::error::Error>> {
    /**/ /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */ /**/
    /**/ /*                        Lexer State Variables                        */ /**/
    /**/ /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */ /**/
    /**/ let mut token = token_head;                                   /**/
    /**/ let mut chars = source.chars().peekable();    /**/
    /**/ let mut buffer = String::new();                                   /**/
    /**/ let mut start_of_line = true;                                       /**/
    /**/ let mut has_decimal = false;                                        /**/
    /**/ let mut line = 1;                                                  /**/
    /**/ /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */ /**/

    while let Some(character) = chars.next() {
        if character.is_ascii_alphabetic() || character == '_' {
            buffer.push(character);

            while let Some(&next_char) = chars.peek() {
                if !next_char.is_ascii_alphanumeric() && next_char != '_' {
                    break;
                }

                buffer.push(chars.next().unwrap());
            }

            if Keyword::from_str(buffer.as_str()).is_ok() {
                token = Token::set(token, TokenType::Keyword(buffer.clone()), 0);
            } else {
                token = Token::set(token, TokenType::Identifier(buffer.clone()), 0);
            }
            
            buffer.clear();
        } else if character.is_numeric() || (character == '.' && chars.peek().map_or(false, |c| c.is_ascii_digit())) {
            if buffer.is_empty() && character == '0' {
                let Some(&next_char) = chars.peek() else {
                    return Err(format!("Unexpected end of input after '0' on line {}", line).into());
                };

                if next_char >= '0' && next_char <= '7' {
                    let escape_char = chars.next().unwrap();
                    let oct_digits = format!("0{}", process_octal(&mut chars, escape_char));
                    let oct_value = u32::from_str_radix(&oct_digits, 8).map_err(|_| format!("Invalid octal number '{}' on line {}", oct_digits, line))?;

                    token = Token::set(token, TokenType::Literal(oct_value.to_string()), 0);
                    start_of_line = false;

                    continue;
                } else if next_char == 'x' || next_char == 'X' {
                    chars.next();
                    let mut hex_digits = String::new();

                    while let Some(&c) = chars.peek() {
                        if !c.is_digit(16) {
                            break;
                        }

                        hex_digits.push(c);
                        chars.next();
                    }

                    if hex_digits.is_empty() {
                        return Err(format!("Invalid hex number on line {}", line).into());
                    }

                    let hex_value = u32::from_str_radix(&hex_digits, 16).map_err(|_| format!("Invalid hex number '{}' on line {}", hex_digits, line))?;

                    token = Token::set(token, TokenType::Literal(hex_value.to_string()), 0);
                    start_of_line = false;

                    continue;
                }
            }

            buffer.push(character);

            if character == '.' {
                has_decimal = true;
            }

            while let Some(&next_char) = chars.peek() {
                if next_char.is_ascii_digit() {
                    buffer.push(chars.next().unwrap());
                } else if next_char.is_dot() {
                    if has_decimal {
                        return Err(format!("Multiple decimal points in number on line {}", line).into());
                    }

                    has_decimal = true;
                    buffer.push(chars.next().unwrap());
                } else if next_char.is_sci_notation() {
                    buffer.push(chars.next().unwrap());

                    if matches!(chars.peek(), Some(&c) if c == '+' || c == '-') {
                        buffer.push(chars.next().unwrap());
                    }

                    let mut found_digit = false;
                    while let Some(&exp_digit) = chars.peek() {
                        if !exp_digit.is_ascii_digit() {
                            break;
                        }

                        if !found_digit {
                            found_digit = true;
                        }

                        buffer.push(chars.next().unwrap());
                    }

                    if !found_digit {
                        return Err(format!("Invalid exponent in number on line {}", line).into());
                    }
                } else if next_char.is_ascii_alphabetic() {
                    let mut suffix = String::new();

                    for _ in 0..3 {
                        if let Some(&c) = chars.peek() {
                            let c_lower = c.to_ascii_lowercase();

                            if !c_lower.is_suffix_char() {
                                break;
                            }

                            suffix.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if let Some(&next) = chars.peek() {
                        if !next.is_whitespace() && !SYMBOL_MAP.contains_key(&next) {
                            return Err(format!("Invalid number suffix on line {}", line).into());
                        }
                    }

                    if !suffix.is_num_suffix() {
                        return Err(format!("Invalid number suffix on line {}", line).into());
                    }

                    buffer.push_str(&suffix);
                } else {
                    break;
                }
            }

            token = Token::set(token, TokenType::Literal(buffer.clone()), 0);

            has_decimal = false;
            buffer.clear();
        } else if character.is_whitespace() {
            if character == '\n' {
                line += 1;
                start_of_line = true;
            } else {
                start_of_line = false;
            }

            continue;
        } else if SYMBOL_MAP.contains_key(&character) {
            if character.is_char_literal() {
                if let Some(character) = chars.next() {
                    let mut char_literal = String::new();

                    if character == '\'' {
                        return Err(format!("Empty character literal on line {}", line).into());
                    } else if character == '\\' {
                        if let Some(escape_char) = chars.next() {
                            process_escape_sequence(&mut char_literal, escape_char, &mut chars, line)?;
                        } else {
                            return Err(format!("Unterminated character literal on line {}", line).into());
                        }
                    } else {
                        char_literal.push(character);
                    }

                    if chars.next() == Some('\'') {
                        token = Token::set(token, TokenType::Literal(char_literal), 0);
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
            } else if character.is_string_literal() {
                let mut string_lit = String::new();
                let mut ok = false;

                while let Some(next_char) = chars.next() {
                    match next_char {
                        '"' => {
                            let mut lookahead = chars.clone();

                            while let Some(&c) = lookahead.peek() {
                                if !c.is_whitespace() {
                                    break;
                                }

                                lookahead.next();
                            }

                            if lookahead.peek() != Some(&'"') {
                                token = Token::set(token, TokenType::Literal(string_lit), 0);
                                ok = true;

                                break;
                            }

                            for _ in 0..(chars.clone().count() - lookahead.clone().count()) {
                                chars.next();
                            }
                            
                            chars.next();

                            continue;
                        } '\\' => {
                            if let Some(escape_char) = chars.next() {
                                if escape_char == '\n' || escape_char == '\r' {
                                    chars.next();

                                    continue;
                                }

                                process_escape_sequence(&mut string_lit, escape_char, &mut chars, line)?;
                            } else {
                                return Err(format!("Unmatched escape sequence on line {}", line).into());
                            }
                        }

                        '\n' => return Err(format!("Unterminated string literal on line {}", line).into()),

                        _ => string_lit.push(next_char),
                    }
                }

                if !ok {
                    return Err(format!("Unterminated string literal on line {}", line).into());
                }
            } else if let Some(&second_char) = chars.peek() {
                let mut chars_clone = chars.clone();
                chars_clone.next();

                if let Some(third_char) = chars_clone.next() {
                    let triple_symbol = format!("{}{}{}", character, second_char, third_char);

                    if TRIPLE_SYMBOL_MAP.contains_key(triple_symbol.as_str()) {
                        for _ in 0..2 {
                            chars.next();
                        }

                        token = Token::set(token, TokenType::Operator(triple_symbol), line);
                        start_of_line = false;

                        continue;
                    }
                }

                let double_symbol = [character, second_char].iter().collect::<String>();

                if DOUBLE_SYMBOL_MAP.contains_key(double_symbol.as_str()) {
                    token = Token::set(token, TokenType::Operator(double_symbol), 0);
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


fn process_escape_sequence(string_lit: &mut String, escape_char: char, chars: &mut Peekable<Chars<'_>>, line: usize) -> Result<(), Box<dyn std::error::Error>> {
    match escape_char {
        'a' => { string_lit.push('\x07'); return Ok(()); }
        'b' => { string_lit.push('\x08'); return Ok(()); }
        'f' => { string_lit.push('\x0C'); return Ok(()); }
        'n' => { string_lit.push('\n'); return Ok(()); }
        'r' => { string_lit.push('\r'); return Ok(()); }
        't' => { string_lit.push('\t'); return Ok(()); }
        'v' => { string_lit.push('\x0B'); return Ok(()); }
        '\\' => { string_lit.push('\\'); return Ok(()); }
        '\'' => { string_lit.push('\''); return Ok(()); }
        '"'  => { string_lit.push('"'); return Ok(()); }
        
        'x' => {
            let mut hex_digits = String::new();

            for _ in 0..2 {
                let Some(&c) = chars.peek() else {
                    break;
                };

                if !c.is_ascii_hexdigit() {
                    break;
                }

                hex_digits.push(c);
                chars.next();
            }

            if hex_digits.is_empty() {
                return Err(format!("Expected hex digits after \\x on line {}", line).into());
            }

            let value = u8::from_str_radix(&hex_digits, 16).map_err(|_| format!("Invalid hex escape: \\x{} on line {}", hex_digits, line))?;
            string_lit.push(value as char);

            return Ok(());
        }

        'u' => {
            let Some('{') = chars.next() else {
                return Err(format!("Expected '{{' after \\u on line {}", line).into());
            };

            let mut unicode_digits = String::new();

            while let Some(&c) = chars.peek() {
                if c == '}' {
                    chars.next();
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

            let codepoint = u32::from_str_radix(&unicode_digits, 16).map_err(|_| format!("Invalid unicode escape: \\u{{{}}} on line {}", unicode_digits, line))?;

            let Some(ch) = char::from_u32(codepoint) else {
                return Err(format!("Invalid unicode codepoint: \\u{{{}}} on line {}", unicode_digits, line).into());
            };

            string_lit.push(ch);

            return Ok(());
        }

        '0'..='7' => {
            let oct_digits = process_octal(chars, escape_char);
            let oct_value = u8::from_str_radix(&oct_digits, 8).map_err(|_| format!("Invalid octal escape: \\{} on line {}", oct_digits, line))?;
            
            string_lit.push(oct_value as char);

            return Ok(());
        }
        
        _ => return Err(format!("Invalid escape sequence: \\{} on line {}", escape_char, line).into()),
    }
}


fn process_octal(chars: &mut Peekable<Chars<'_>>, escape_char: char) -> String {
    let mut oct_digits = String::new();
    oct_digits.push(escape_char);

    for _ in 0..2 {
        let Some(&c) = chars.peek() else {
            break;
        };

        if c < '0' || c > '7' {
            break;
        }

        chars.next();
        oct_digits.push(c);
    }

    oct_digits
}