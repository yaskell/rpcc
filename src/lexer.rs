use crate::token::*;
use crate::process;
use regex::Regex;
use regex::Match;

pub fn lex(mut file: String) -> Vec<Token> {

    let mut tokens: Vec<Token> = Vec::new();
    let token_definitions: Vec<TokenDefinition> = initialize_token_definition();

    while !file.is_empty() {
        if file.starts_with(char::is_whitespace) {
            file = file.trim_start().to_string();
            continue;
        } else {

            let mut longest_capture: Option<regex::Match> = None;
            let mut captured_token: Option<&Token> = None;

            for token in &token_definitions {
                if let Some(captured) = token.regex.find(&file) {
                    match &longest_capture {
                        None => { // Set if not set
                            longest_capture = Some(captured);
                            captured_token = Some(&token.token_type);
                        },
                        Some(x) => { // Otherwise check if should update
                            if captured.len() > x.len() {
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
                        Some(x) =>  {
                            longest_capture = Some(x);
                        },
                        None => {
                            let start_to_first_word_boundry = Regex::new(r"^[\s\S]*?(:?\b|$)").unwrap();
                            eprintln!("Invalid keyword: \"{}\"", start_to_first_word_boundry.find(&file).unwrap().as_str());
                            process::exit(1);
                        }
                    }

                },                
                Some(x) => match &captured_token.unwrap() {
                    Token::Identifier(_) => tokens.push(Token::Identifier(x.as_str().to_string())),
                    Token::Constant(_) => tokens.push(Token::Constant(x.as_str().parse::<i32>().expect("Could not convert to i32"))),
                    Token::OpenParan => tokens.push(Token::OpenParan),
                    Token::CloseParan => tokens.push(Token::CloseParan),
                    Token::OpenBrace => tokens.push(Token::OpenBrace),
                    Token::CloseBrace => tokens.push(Token::CloseBrace),
                    Token::Semicolon => tokens.push(Token::Semicolon),
                    Token::Int => tokens.push(Token::Int),
                    Token::Void => tokens.push(Token::Void),
                    Token::Return => tokens.push(Token::Return),
                }
            }
            file = file[longest_capture.unwrap().end()..].to_string();
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
