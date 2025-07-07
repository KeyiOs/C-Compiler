mod logic;
mod data_structures;

use data_structures::objects::Token;
use logic::lexer::lexer_start;
use std::process::Command;

const DEBUG: bool = false;


fn main() {
    let preprocessed_source = match preprocess_source("./examples/test.c") {
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


pub fn debug_tokens(token: &Token) {
    let mut current = Some(token);

    while let Some(tok) = current {
        if let Some(token_type) = &tok.token_type {
            println!("{}", token_type);
        }

        current = tok.next.as_deref();
    }
}