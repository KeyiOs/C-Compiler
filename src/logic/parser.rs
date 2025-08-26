use crate::data_structures::token::{BaseType, Class};
#[allow(unused_imports)]
use crate::data_structures::token::{Declaration, Token, TokenType, Type};
use std::{cell::RefCell, rc::Rc};


pub fn parser_start(mut current_token: Option<Rc<RefCell<Token>>>) -> String {
    let mut _tree = Vec::<Declaration>::new();

    while let Some(ref mut token_rc) = current_token {
        let token_type = token_rc.borrow().token_type.clone();
        let next_token = token_rc.borrow().next.clone();
        
        if let Some(token_type) = &token_type {
            match token_type {
                TokenType::Keyword(_) => {
                    process_keyword(&_tree, token_rc.clone());
                }
                TokenType::Identifier(_) => {
                    
                }
                TokenType::Literal(_) => {
                    
                }
                TokenType::Operator(_) => {
                    
                }
            }
        }

        current_token = next_token;
    }

    "Parsing completed".to_string()
}


fn process_keyword(_tree: &Vec<Declaration>, mut token: Rc<RefCell<Token>>) {
    let keyword = token.borrow().get_string().unwrap().to_string();

    match keyword.as_str() {
        "typedef" => {
            let mut identifiers = Vec::new();

            token = Token::next_token(&token);
            while token.borrow().get_string().unwrap().to_string() != ";" {
                let identifier = token.borrow().get_string().unwrap().to_string();
                identifiers.push(identifier);

                token = Token::next_token(&token);
            }

            #[allow(non_snake_case)]
            let AST: Declaration;

            if identifiers[1] == "(" {
                AST = set_ast(Class::Typedef, identifiers[0].clone(), Type::Function { return_type: identifiers[0].clone(), parameters: identifiers[1..].to_vec() }, None);
            } else {
                AST = set_ast(Class::Typedef, identifiers.pop().unwrap(), Type::Primitive { ttype: BaseType::Other(identifiers.join(" ")) }, None);
            }

            println!("{}", AST);
        }
        _ => {}
    }
}


fn set_ast(class: Class, name: String, type_info: Type, initializer: Option<String>) -> Declaration {
    Declaration {
        class: class,
        name: name,
        type_info: type_info,
        initializer: initializer,
    }
}