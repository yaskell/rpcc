#![feature(variant_count)]

use {
    regex::Regex, 
    std::{env, fs, mem, process}
}; 


fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args);
    let file_content = fs::read_to_string(&arguments.file_path).expect("Could not read file");
    
    let tokens: Vec<Token> = lex(file_content);
    println!("final array = {:?}", tokens);
    process::exit(0);
}

fn lex(mut file: String) -> Vec<Token> {
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
                    eprintln!("Invalid keyword found");
                    process::exit(1);
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

#[derive(PartialEq, Eq, Hash, Debug)]
enum Token {
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
struct TokenDefinition {
    token_type: Token,
    regex: Regex
}

fn initialize_token_definition() -> Vec<TokenDefinition> {
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
            regex: Regex::new(r"}").unwrap(),

        },
        TokenDefinition {
            token_type: Token::OpenBrace,
            regex: Regex::new(r"\{").unwrap(),

        },        
        TokenDefinition {
            token_type: Token::CloseParan,
            regex: Regex::new(r"\)").unwrap(),

        },
        TokenDefinition {
            token_type: Token::OpenParan,
            regex: Regex::new(r"\(").unwrap(),

        },
        TokenDefinition {
            token_type: Token::Semicolon,
            regex: Regex::new(r"^;").unwrap(),

        },        
        TokenDefinition {
            token_type: Token::Int,
            regex: Regex::new(r"int\b").unwrap(),

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

enum Flag {
    Lex,
    Parse,
    Codegen,
    Assembly,
}

struct Arguments {
    file_path: String,
    flag: Option<Flag>,
}

impl Arguments {
    fn new(args: &[String]) -> Arguments {

        if args.len() < 2 {
            eprintln!("ERROR: not enough arguments");
            usage_message();
            process::exit(1);
        }

        if args.len() > 3 {
            eprintln!("ERROR: too many arguments");
            usage_message();
            process::exit(1)
        }

        let file_path = args[1].clone();
        let flag: Option<Flag> = if args.len() == 3 {
            match args[2].as_str() {
                "--lex" => Some(Flag::Lex),
                "--parse" => Some(Flag::Parse),
                "--codegen" => Some(Flag::Codegen),
                "-S" => Some(Flag::Assembly),
                _ => {
                    eprintln!("ERROR: flag `{}` not recognized", args[2].as_str());
                    usage_message();
                    process::exit(1);
                }
            }
        } else {
            None
        };

        Arguments { file_path, flag }
    }
}

fn usage_message() {
    eprintln!("Usage: <path> [--lex | --parse | --codegen | -S]");
}

