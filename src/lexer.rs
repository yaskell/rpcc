use regex::Match;
use regex::Regex;

use std::process;
use std::sync::LazyLock;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    OpenParan,
    CloseParan,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Minus,
    DoubleMinus,
    Int,
    Void,
    Return,
}

impl Token {
    fn new(token_value: regex::Match, token_type: &Token) -> Token {
        match token_type {
            Token::Identifier(_) => KEYWORD_TOKENS_DEFINITIONS
                .iter()
                .find(|kw| kw.regex.is_match(token_value.as_str()))
                .map(|kw| kw.token_type.clone())
                .unwrap_or_else(|| Token::Identifier(token_value.as_str().to_string())),
            Token::Constant(_) => Token::Constant(
                token_value
                    .as_str()
                    .parse::<i32>()
                    .expect("Could not convert to i32"),
            ),
            token => token.clone(),
        }
    }
}

#[derive(Debug)]
pub struct TokenDefinition {
    pub token_type: Token,
    pub regex: Regex,
}

pub static KEYWORD_TOKENS_DEFINITIONS: LazyLock<Vec<TokenDefinition>> = LazyLock::new(|| {
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

pub static OTHER_TOKENS_DEFINITIONS: LazyLock<Vec<TokenDefinition>> = LazyLock::new(|| {
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
        TokenDefinition {
            token_type: Token::Minus,
            regex: Regex::new(r"^-").unwrap(),
        },
        TokenDefinition {
            token_type: Token::DoubleMinus,
            regex: Regex::new(r"^--").unwrap(),
        },
        TokenDefinition {
            token_type: Token::Tilde,
            regex: Regex::new(r"^~").unwrap(),
        },
    ]
});

#[derive(Debug)]
struct Capture<'a> {
    value: regex::Match<'a>,
    token_type: &'a Token,
}

pub fn lex(mut file: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    while !file.is_empty() {
        if file.starts_with(char::is_whitespace) {
            file = file.trim_start();
            continue;
        }

        if let Some(comment) = check_if_comment(file) {
            file = &file[comment.end()..];
            continue;
        }

        match find_longest_match(file) {
            Some(m) => {
                tokens.push(Token::new(m.value, m.token_type));
                file = &file[m.value.end()..];
            }
            None => {
                let invalid_token = file.split_whitespace().next().unwrap_or(file);
                eprintln!("Invalid token: \"{invalid_token}\"");
                process::exit(1);
            }
        }
    }
    tokens
}

fn find_longest_match<'a>(file: &'a str) -> Option<Capture<'a>> {
    let mut longest_capture: Option<Capture<'a>> = None;

    for token_definition in OTHER_TOKENS_DEFINITIONS.iter() {
        if let Some(capture) = token_definition.regex.find(file) {
            if longest_capture
                .as_ref()
                .is_none_or(|current_longest| capture.len() > current_longest.value.len())
            {
                longest_capture = Some(Capture {
                    value: capture,
                    token_type: &token_definition.token_type,
                });
            }
        }
    }
    longest_capture
}

fn check_if_comment(file: &str) -> Option<Match<'_>> {
    let single_line_comment = Regex::new(r"^//[^\r\n]*").unwrap();
    let multi_line_comment = Regex::new(r"^\/\*[\s\S]*? \*\/").unwrap();

    if let Some(x) = single_line_comment.find(&file) {
        return Some(x);
    };
    if let Some(x) = multi_line_comment.find(&file) {
        return Some(x);
    };
    None
}
