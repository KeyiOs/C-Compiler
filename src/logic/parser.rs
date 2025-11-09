use std::fmt::Error;

use crate::{Token, TokenType, data::ParserState};


pub fn parser_start(tokens: &Vec<Token>) -> Result<(), Box<Error>> {
    let mut parser_state = ParserState { tokens, iterator: 0 };

    process_switch(&mut parser_state)?;

    Ok(())
}

fn process_switch(parser_state: &mut ParserState) -> Result<(), Box<Error>> {
    let tokens = parser_state.tokens;
    let mut i = parser_state.iterator;

    while i < tokens.len() {
        match &tokens[i].token_type {
            TokenType::Keyword(_) => {

            } TokenType::Identifier(_) => {

            } TokenType::Literal(_) => {
                process_expression(parser_state)?;
            } TokenType::Operator(_) => {
                process_expression(parser_state)?;
            }
        }

        i += 1;
    }

    parser_state.iterator = i;
    Ok(())
}

fn process_expression(parser_state: &mut ParserState) -> Result<(), Box<Error>> {
    let tokens = &parser_state.tokens;
    let i = parser_state.iterator;
    let string = tokens[i].token_type.value();
    
    if string == "(" {
        // if !stack.empty() {
        //  return Err();
        // }

        parser_state.iterator = i + 1;
        process_expression(parser_state)?;
    } else if matches!(tokens[i].token_type, TokenType::Literal(_)) {
        // stack.push(tokens[i]);
        
        parser_state.iterator = i + 1;
        process_expression(parser_state)?;
    } else if matches!(tokens[i].token_type, TokenType::Operator(_)) {
        // ast.type = BinaryOperation;
        // ast.left = stack.pop();
        // ast.operator = tokens[i];

        parser_state.iterator = i + 1;
        process_expression(parser_state)?;
    } else if string == ")" {
        // ast.type = Literal;
        // ast.value = tokens[i];
        // return ast;
    }

    Ok(())  // Replace with return Ast
}
