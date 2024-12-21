#![feature(variant_count)]

mod lexer;
mod token;

use lexer::lex;
use token::Token;

use std::process;
use std::fs;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args);
    let file_content = fs::read_to_string(&arguments.file_path).expect("Could not read file");
    
    let tokens: Vec<Token> = lex(file_content);
    println!("final array = {:?}", tokens);
    process::exit(0);
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

