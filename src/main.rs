use {
    std::process,
    std::env,
    std::fs,
    std::collections::HashMap,
    regex::{Regex, Match},
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
    let token_regex: HashMap<Token, Regex> = initialize_regex_map(HashMap::new());

    while !file.is_empty() {

        if file.starts_with(char::is_whitespace) {
            file = file.trim_start().to_string();
            continue;
        } else {

            let mut longest_capture: Option<String> = None;
            let mut token: Option<&Token> = None;

             for (curr_token, regex) in &token_regex {
                if let Some(captured) = regex.find(&file) {
                    if let Some(long_cap) = &longest_capture {
                        if captured.len() > long_cap.len() {
                            token = Some(curr_token);
                            longest_capture = Some(captured.as_str().to_string());
                        }
                    } else {
                        longest_capture = Some(captured.as_str().to_string());
                        token = Some(curr_token);
                    }

                    file = file[captured.end()..].to_string();
                }
            }

            match longest_capture {
                None => {
                    eprintln!("No capture was found");
                    process::exit(1);
                },                
                Some(x) => match token.unwrap() {
                    Token::Identifier(_) => tokens.push(Token::Identifier(x.to_string())),
                    Token::Constant(_) => tokens.push(Token::Constant(x.parse::<i32>().expect("Could not convert to i32"))),
                    Token::OpenParam => tokens.push(Token::OpenParam),
                    Token::CloseParam => tokens.push(Token::CloseParam),
                    Token::OpenBrace => tokens.push(Token::OpenBrace),
                    Token::CloseBrace => tokens.push(Token::CloseBrace),
                    Token::Semicolon => tokens.push(Token::Semicolon),
                    Token::Int => tokens.push(Token::Int),
                    Token::Void => tokens.push(Token::Void),
                    Token::Return => tokens.push(Token::Return),
                }
            }

        }
    }

    tokens
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Token {
    Identifier(String),
    Constant(i32),
    OpenParam,
    CloseParam,
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
            Token::OpenParam => "OpenParam".to_string(),
            Token::CloseParam => "CloseParam".to_string(),
            Token::OpenBrace => "OpenBrace".to_string(),
            Token::CloseBrace => "CloseBrace".to_string(),
            Token::Semicolon => "Semicolon".to_string(),
            Token::Int => "Int".to_string(),
            Token::Void => "Void".to_string(),
            Token::Return => "Return".to_string(),
        }
    }
}

//Very ugly code, but I can't think of another way to let the compiler notify what to update
//exhaustively
fn initialize_regex_map(mut map: HashMap<Token, Regex>) -> HashMap<Token, Regex> {
    let something: Token = Token::Void;
    match something {
        Token::Identifier(_) |             
            Token::Constant(_) |             
            Token::CloseBrace |             
            Token::OpenBrace |             
            Token::CloseParam |             
            Token::OpenParam |             
            Token::Semicolon |             
            Token::Int |             
            Token::Void |             
            Token::Return 
            => {
                map.insert(Token::Semicolon, Regex::new(r"^;").unwrap());
                map.insert(Token::OpenParam, Regex::new(r"^\(").unwrap());
                map.insert(Token::CloseParam, Regex::new(r"^\)").unwrap());
                map.insert(Token::OpenBrace, Regex::new(r"^\{").unwrap());
                map.insert(Token::CloseBrace, Regex::new(r"^\}").unwrap());
                map.insert(Token::Constant(0), Regex::new(r"^[0-9]+\b").unwrap());
                map.insert(Token::Identifier(String::from("empty")), Regex::new(r"^[a-zA-Z_]\w*\b").unwrap());
                map.insert(Token::Int, Regex::new(r"^int\b").unwrap());
                map.insert(Token::Void, Regex::new(r"^void\b").unwrap());
                map.insert(Token::Return, Regex::new(r"^return\b").unwrap());
            }
    };
    map
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

