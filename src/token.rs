use regex::Regex;
use std::mem;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    OpenParan,
    CloseParan,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Int,
    Void,
    Return,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Identifier(s) => format!("Identifier({})", s),
            Token::Constant(i) => format!("Constant({})", i),
            Token::OpenParan => "OpenParan".to_string(),
            Token::CloseParan => "CloseParan".to_string(),
            Token::OpenBrace => "OpenBrace".to_string(),
            Token::CloseBrace => "CloseBrace".to_string(),
            Token::Semicolon => "Semicolon".to_string(),
            Token::Int => "Int".to_string(),
            Token::Void => "Void".to_string(),
            Token::Return => "Return".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct TokenDefinition {
    pub token_type: Token,
    pub regex: Regex
}

pub fn initialize_token_definition() -> Vec<TokenDefinition> {
    let token_def_vec: Vec<TokenDefinition> = vec![
        TokenDefinition {
            token_type: Token::Identifier("".to_string()),
            regex: Regex::new(r"^[a-zA-Z_]\w*\b").unwrap(),

        },
        TokenDefinition {
            token_type: Token::Constant(0),
            regex: Regex::new(r"^[0-9]+\b").unwrap(),

        },
        TokenDefinition {
            token_type: Token::CloseBrace,
            regex: Regex::new(r"^\}").unwrap(),

        },
        TokenDefinition {
            token_type: Token::OpenBrace,
            regex: Regex::new(r"^\{").unwrap(),

        },        
        TokenDefinition {
            token_type: Token::CloseParan,
            regex: Regex::new(r"^\)").unwrap(),

        },
        TokenDefinition {
            token_type: Token::OpenParan,
            regex: Regex::new(r"^\(").unwrap(),

        },
        TokenDefinition {
            token_type: Token::Semicolon,
            regex: Regex::new(r"^;").unwrap(),

        },        
        TokenDefinition {
            token_type: Token::Int,
            regex: Regex::new(r"^int\b").unwrap(),

        },
        TokenDefinition {
            token_type: Token::Void,
            regex: Regex::new(r"^void\b").unwrap(),

        },
        TokenDefinition {
            token_type: Token::Return,
            regex: Regex::new(r"^return\b").unwrap(),
        }
    ];

    if token_def_vec.len() != mem::variant_count::<Token>() {
        panic!("ERROR: Token Definition amount does not match Tokens defined in enum 'Token'");
    }

    token_def_vec
}

