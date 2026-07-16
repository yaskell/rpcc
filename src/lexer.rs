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

pub fn lex(mut file: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    while !file.is_empty() {
        if file.starts_with(char::is_whitespace) {
            file = file.trim_start().to_string();
            continue;
        }

        let mut longest_capture: Option<regex::Match> = None;
        let mut captured_token: Option<&Token> = None;

        for token in OTHER_TOKENS.iter() {
            if let Some(captured) = token.regex.find(&file) {
                match &longest_capture {
                    None => {
                        longest_capture = Some(captured);
                        captured_token = Some(&token.token_type);
                    }
                    Some(existing) => {
                        if captured.len() > existing.len() {
                            longest_capture = Some(captured);
                            captured_token = Some(&token.token_type);
                        }
                    }
                }
            }
        }

        match longest_capture {
            None => match check_if_comment(&file) {
                Some(x) => {
                    file = file[x.end()..].to_string();
                    continue;
                }
                None => {
                    let start_to_first_word_boundary = Regex::new(r"^[\s\S]*?(?:\b|$)").unwrap();
                    eprintln!(
                        "Invalid token: \"{}\"",
                        start_to_first_word_boundary.find(&file).unwrap().as_str()
                    );
                    process::exit(1);
                }
            },

            Some(capture) => {
                let token_type = captured_token.unwrap();

                tokens.push(match token_type {
                    Token::Identifier(_) => {
                        let text = capture.as_str();

                        let keyword_match =
                            KEYWORD_TOKENS.iter().find(|kw| kw.regex.is_match(text));

                        if let Some(kw) = keyword_match {
                            tokens.push(kw.token_type.clone());
                        } else {
                            tokens.push(Token::Identifier(text.to_string()));
                        }
                    }

                    Token::Constant(_) => Token::Constant(
                        capture
                            .as_str()
                            .parse::<i32>()
                            .expect("Could not convert to i32"),
                    ),

                    token => token.clone(),
                });

                file = file[capture.end()..].to_string();
            }
        }
    }

    tokens
}

fn check_if_comment(file: &String) -> Option<Match<'_>> {
    let single_line_comment = Regex::new(r"//[^\r\n]*").unwrap();
    let multi_line_comment = Regex::new(r"\/\*[\s\S]*? \*\/").unwrap();

    if let Some(x) = single_line_comment.find(&file) {
        return Some(x);
    };
    if let Some(x) = multi_line_comment.find(&file) {
        return Some(x);
    };
    None
}
