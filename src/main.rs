use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: <path> [--lex | --parse | --codegen]");
        process::exit(1);
    }

    let path = &args[1];

    if args.len() > 2 {
        let flag = &args[2];

        match flag.as_str() {
            "--lex" => println!("Lexing"),
            "--parse" => println!("Parsing"),
            "--codegen" => println!("Code Generation"),
            _ => eprintln!("Unknown flag: {}", flag),
        }

        println!("Path: {}, Flag: {}", path, flag);
    } else {
        println!("Path: {}", path);
    }

    process::exit(0);
}

