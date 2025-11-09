mod data;
mod logic;

use std::path::Path;
use std::fs;
use std::process::Command;
use std::io::Write;
use data::Token;
use logic::lexer_start;
use logic::parser_start;

use crate::data::TokenType;

const INPUT_CODE: &str = "./examples/oddEven.c";
const DEBUG: bool = false;
const MAX_TO_PRINT: usize = 20;
const PRINT_TYPE: u8 = 4; // 0=All, 1=Keyword, 2=Operator, 3=Literal, 4=Identifier

fn main() {
    let preproces_source = match preproces_source(INPUT_CODE) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("Preprocessing failed: {}", e);
            return;
        }
    };

    let token = match lexer_start(&preproces_source) {
        Ok(token) => token,
        Err(e) => {
            println!("\nError: {:?}\n", e);
            return;
        }
    };

    let _parser_return = match parser_start(&token) {
        Ok(parser_return) => parser_return,
        Err(e) => {
            println!("\nError: {:?}\n", e);
            return;
        }
    };

    if DEBUG {
        preproces_out(preproces_source);
        token_out(token);
    }
}


fn preproces_source(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("cpp")
        .arg(file_path)
        .output()?;

    if !output.status.success() {
        return Err(format!("Preprocessor failed with code {:?}", output.status.code()).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}


/* * * * * * * */
/*  - DEGUB -  */
/* * * * * * * */
fn preproces_out(preproces_source: String) {
    let output_path = "output/prep_out.c";
    let output_dir = Path::new(output_path).parent().unwrap();

    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create directory {:?}: {}", output_dir, e);
        return;
    }

    if let Err(e) = fs::write(output_path, preproces_source) {
        eprintln!("Failed to write preprocessed output: {}", e);
    }
}


pub fn token_out(tokens: Vec<Token>) {
    let output_path = "output/token_out.txt";
    let output_dir = Path::new(output_path).parent().unwrap();

    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create directory {:?}: {}", output_dir, e);
        return;
    }

    let mut file = match fs::File::create(output_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create output file {}: {}", output_path, e);
            return;
        }
    };

    let mut count = 0;
    for token in tokens.iter().rev() {
        let (type_id, text) = match &token.token_type {
            TokenType::Keyword(s) => (1, s),
            TokenType::Operator(s) => (2, s),
            TokenType::Literal(s) => (3, s),
            TokenType::Identifier(s) => (4, s),
        };

        if PRINT_TYPE != 0 && PRINT_TYPE != type_id {
            continue;
        }

        if let Err(e) = writeln!(file, "{}", text) {
            eprintln!("Failed to write to file: {}", e);
            break;
        }

        count += 1;
        if MAX_TO_PRINT != 0 && count >= MAX_TO_PRINT {
            break;
        }
    }
}