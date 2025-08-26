mod logic;
mod data_structures;

use data_structures::token::Token;
use logic::lexer::lexer_start;
use std::{cell::RefCell, fs::{self, File, OpenOptions}, io::Write, path::Path, process::Command, rc::Rc};

use crate::data_structures::token::{TokenType};

// Debugging constants
const OUT: bool = false;
const MAX_TO_PRINT: usize = 0;
const PRINT_TYPE: u8 = 0; // 0=All, 1=Keyword, 2=Operator, 3=Literal, 4=Identifier


fn main() {
    let prep_source = match prep_source("./examples/oddEven.c") {
        Ok(src) => src,
        Err(e) => {
            eprintln!("Preprocessing failed: {}", e);
            return;
        }
    };

    let mut token = Token::init();
    let error = lexer_start(&mut token, &prep_source);

    if let Err(e) = error {
        println!("\nError: {:?}\n", e);
    }

    let _parser_return = logic::parser::parser_start(Some(token.clone()));

    if OUT {
        prep_out(&prep_source);
        token_out(&token);
    }

    println!("\nOK\n")
}


fn prep_source(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("cpp")
        .arg(file_path)
        .output()?;

    if !output.status.success() {
        return Err(format!("Preprocessor failed with code {:?}", output.status.code()).into());
    }

    let preprocessed_code = String::from_utf8(output.stdout)?;
    Ok(preprocessed_code)
}


fn prep_out(prep_source: &String) {
    let output_path = "output/prep_out.c";
    let output_dir = Path::new(output_path).parent().unwrap();

    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create directory {:?}: {}", output_dir, e);
        return;
    }

    if let Err(e) = fs::write(output_path, &prep_source) {
        eprintln!("Failed to write preprocessed output: {}", e);
    }
}


fn token_out(token: &Rc<RefCell<Token>>) {
    let output_path = "output/token_out.txt";
    let output_dir = Path::new(output_path).parent().unwrap();

    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create directory {:?}: {}", output_dir, e);
        return;
    }

    let mut tokens = Vec::new();
    let mut current = Some(Rc::clone(token));

    while let Some(tok) = current {
        tokens.push(Rc::clone(&tok));

        let borrowed = tok.borrow();
        current = borrowed.next.as_ref().map(Rc::clone);
    }

    let start = if MAX_TO_PRINT == 0 || tokens.len() <= MAX_TO_PRINT {
        0
    } else {
        tokens.len() - MAX_TO_PRINT
    };

    if let Err(e) = File::create(output_path) {
        eprintln!("Failed to clear file {:?}: {}", output_path, e);
    }

    for (i, tok) in tokens[start..].iter().enumerate() {
        let borrowed = tok.borrow();
        if let Some(token_type) = &borrowed.token_type {
            let should_print = match (PRINT_TYPE, token_type) {
                (0, _) => true,
                (1, TokenType::Keyword(_)) => true,
                (2, TokenType::Operator(_)) => true,
                (3, TokenType::Literal(_)) => true,
                (4, TokenType::Identifier(_)) => true,
                _ => false,
            };

            if should_print {
                let content = format!("{}. {:?}\n", i + 1, token_type);
                let result = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(output_path)
                    .and_then(|mut file| file.write_all(content.as_bytes()));

                if let Err(e) = result {
                    println!("write to {:?} failed: {}", output_path, e);
                }
            }
        }
    }
}