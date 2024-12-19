use {
    std::process,
    std::env,
    std::fs,
}; 

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args);
    let file_content = fs::read_to_string(&arguments.file_path).expect("Could not read file");

    match arguments.flag {
        Some(flag) => match flag {
            Flag::Lex => lex(file_content), 
            Flag::Parse => parse(), 
            Flag::Codegen => codegen(),
            Flag::Assembly => assembly(),
        }
        None => println!("Using file '{}'", arguments.file_path),
    }

    process::exit(0);
}

fn lex(mut file: String) {
    }

fn parse() {
    todo!();
}

fn codegen() {
    todo!();
}

fn assembly() {
    todo!();
}

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


impl Token {
    fn get_regex(self) -> &str {
        match self {
            Token::Identifier(_) => r",[a-zA-Z_]\w*\b",
            Token::Constant(_) => r"[0-9]+\b",
            Token::OpenParam => r"\(",
            Token::CloseParam => r"\)",
            Token::OpenBrace => r"{",
            Token::CloseBrace => r"}",
            Token::Semicolon => r";",
            Token::Int => r"int\b",
            Token::Void => r"void\b",
            Token::Return => r"return\b",
        }

    }
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

