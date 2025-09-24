use std::sync::LazyLock;
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
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

#[derive(Debug)]
pub struct TokenDefinition {
    pub token_type: Token,
    pub regex: Regex,
}

pub static KEYWORD_TOKENS: LazyLock<Vec<TokenDefinition>> = LazyLock::new(|| {
    vec![
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
        },
    ]
});

pub static OTHER_TOKENS: LazyLock<Vec<TokenDefinition>> = LazyLock::new(|| {
    vec![
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
    ]
});
