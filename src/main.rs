mod logic;
mod data_structures;

use data_structures::objects::Token;
use logic::lexer::lexer_start;
use std::{cell::RefCell, process::Command, rc::Rc};

use crate::data_structures::objects::TokenType;

const DEBUG: bool = false;
const MAX_TO_PRINT: usize = 85;
const PRINT_TYPE: u8 = 1; // 1=Keyword, 2=Operator, 3=Literal, 4=Identifier

fn main() {
    let preprocessed_source = match preprocess_source("./examples/oddEven.c") {
        Ok(src) => src,
        Err(e) => {
            eprintln!("Preprocessing failed: {}", e);
            return;
        }
    };

    //println!("Preprocessed Source:\n{}", preprocessed_source);

    let mut token = Token::init();
    let error = lexer_start(&mut token, &preprocessed_source);

    if let Err(e) = error {
        println!("\nError: {:?}\n", e);
    } else if DEBUG {
        debug_tokens(&token);
    } else {
        println!("\nOK\n");
    }
}


pub fn preprocess_source(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("cpp")
        .arg(file_path)
        .output()?;

    if !output.status.success() {
        return Err(format!("Preprocessor failed with code {:?}", output.status.code()).into());
    }

    let preprocessed_code = String::from_utf8(output.stdout)?;
    Ok(preprocessed_code)
}


pub fn debug_tokens(token: &Rc<RefCell<Token>>) {
    let mut tokens = Vec::new();
    let mut current = Some(Rc::clone(token)); // clone Rc to own it

    while let Some(tok) = current {
        tokens.push(Rc::clone(&tok)); // store owned Rc

        let borrowed = tok.borrow();
        current = borrowed.next.as_ref().map(Rc::clone);
    }

    let start = if tokens.len() > MAX_TO_PRINT { tokens.len() - MAX_TO_PRINT } else { 0 };
    for (i, tok) in tokens[start..].iter().enumerate() {
        let borrowed = tok.borrow();
        if let Some(token_type) = &borrowed.token_type {
            let should_print = match (PRINT_TYPE, token_type) {
                (1, TokenType::Keyword(_)) => true,
                (2, TokenType::Operator(_)) => true,
                (3, TokenType::Literal(_)) => true,
                (4, TokenType::Identifier(_)) => true,
                _ => false,
            };

            if should_print {
                println!("{}. {:?}", i + 1, token_type);
            }
        }
    }
}