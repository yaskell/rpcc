use crate::token::*;
use crate::process;
use regex::Regex;
use regex::Match;

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
            None => {
                match check_if_comment(&file) {
                    Some(x) => {
                        file = file[x.end()..].to_string();
                        continue;
                    }
                    None => {
                        let start_to_first_word_boundary =
                            Regex::new(r"^[\s\S]*?(?:\b|$)").unwrap();
                        eprintln!(
                            "Invalid token: \"{}\"",
                            start_to_first_word_boundary.find(&file).unwrap().as_str()
                        );
                        process::exit(1);
                    }
                }
            }

            Some(capture) => {
                let token_type = captured_token.unwrap();

                match token_type {
                    Token::Identifier(_) => {
                        let text = capture.as_str();

                        let keyword_match = KEYWORD_TOKENS.iter().find(|kw| kw.regex.is_match(text));

                        if let Some(kw) = keyword_match {
                            tokens.push(kw.token_type.clone());
                        } else {
                            tokens.push(Token::Identifier(text.to_string()));
                        }
                    }

                    Token::Constant(_) => {
                        tokens.push(Token::Constant(
                            capture.as_str().parse::<i32>().expect("Could not convert to i32"),
                        ));
                    }

                    Token::OpenParan => tokens.push(Token::OpenParan),
                    Token::CloseParan => tokens.push(Token::CloseParan),
                    Token::OpenBrace => tokens.push(Token::OpenBrace),
                    Token::CloseBrace => tokens.push(Token::CloseBrace),
                    Token::Semicolon => tokens.push(Token::Semicolon),

                    Token::Int => tokens.push(Token::Int),
                    Token::Void => tokens.push(Token::Void),
                    Token::Return => tokens.push(Token::Return),
                }

                file = file[capture.end()..].to_string();
            }
        }
    }

    tokens
}

fn check_if_comment(file: &String) -> Option<Match> {
    let single_line_comment = Regex::new(r"//[^\r\n]*").unwrap();
    let multi_line_comment = Regex::new(r"\/\*[\s\S]*? \*\/").unwrap();

    if let Some(x) = single_line_comment.find(&file) { return Some(x) };
    if let Some(x) = multi_line_comment.find(&file) { return Some(x) };
    None
}
