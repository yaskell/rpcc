use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args);

    match arguments.flag {
        Some(flag) => match flag {
            Flag::Lex => lex(), 
            Flag::Parse => parse(), 
            Flag::Codegen => codegen(),
            Flag::Assembly => assembly(),
        }
        None => println!("Using file '{}'", arguments.file_path),
    }

    process::exit(0);
}

fn lex() {
    todo!();
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


struct Arguments {
    file_path: String,
    flag: Option<Flag>,
}

enum Flag {
    Lex,
    Parse,
    Codegen,
    Assembly,
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

