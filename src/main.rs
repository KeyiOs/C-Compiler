mod logic;
mod data_structures;

use data_structures::objects::Token;
use logic::{utils::preprocess_source, lexer::lexer_start};

const DEBUG: bool = true;


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

    if error.is_err() {
        println!("Error: {:?}", error);
    } else if DEBUG {
        debug_tokens(&token);
    } else {
        println!("\nOK\n");
    }
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