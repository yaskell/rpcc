use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config.flag {
        Some(x) => println!("Using file '{}' with flag '{}'", config.file_path, x),
        None => println!("Using file '{}'", config.file_path),
    }

    process::exit(0);
}

struct Config {
    file_path: String,
    flag: Option<String>,
}

impl Config {
    fn new(args: &[String]) -> Config {

        if args.len() < 2 {
            eprintln!("ERROR: not enough arguments");
            eprintln!("Usage: <path> [--lex | --parse | --codegen]");
            process::exit(1);
        }

        if args.len() > 3 {
            eprintln!("ERROR: too many arguments");
            eprintln!("Usage: <path> [--lex | --parse | --codegen]");
            process::exit(1)
        }

        let file_path = args[1].clone();
        let flag: Option<String> = if args.len() == 3 {
            match args[2].as_str() {
                "--lex" => Some(String::from("lex")),
                "--parse" => Some(String::from("parse")),
                "--codegen" => Some(String::from("codegen")),
                _ => {
                    eprintln!("ERROR: flag `{}` not recognized", args[2].as_str());
                    eprintln!("Usage: <path> [--lex | --parse | --codegen]");
                    process::exit(1);
                }
            }
        } else {
            None
        };

        Config { file_path, flag }
    }
}

