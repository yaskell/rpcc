mod assembly_generation;
mod code_emission;
mod lexer;
mod parser;

use std::env;
use std::fs;
use std::process;

fn main() {
    let arguments = Arguments::new(&env::args().collect::<Vec<String>>());
    let file_content = fs::read_to_string(&arguments.file_path).expect("Could not read file");

    let lexed_val = lexer::lex(file_content);
    if let Some(Flag::Lex) = arguments.flag {
        println!("Stopped before parsing");
        println!("Lexed file contents: {:?}", &lexed_val);
        process::exit(0);
    }

    let parsed_val = parser::parse(&mut lexed_val.clone());
    if let Some(Flag::Parse) = arguments.flag {
        println!("Stopped before generating assembly");
        println!("Parsed file contents: {:?}", &parsed_val);
        process::exit(0);
    }

    let asm_generation_val = assembly_generation::translate_program(parsed_val);
    if let Some(Flag::Codegen) = arguments.flag {
        println!("Stopped before code emission");
        println!("Assembly Generation: {:?}", &asm_generation_val);
        process::exit(0);
    }

    let filename = &arguments.file_path.trim_end_matches(".c");
    if let Ok(_) = code_emission::emit(filename, asm_generation_val) {
        process::Command::new("gcc")
            .args([format!("{}.s", filename).as_str(), "-o", filename])
            .output()
            .expect("has failed");

        if let Some(Flag::Assembly) = arguments.flag {
            process::exit(0);
        }

        let _ = fs::remove_file(format!("{}.s", filename).as_str());

        process::exit(0);
    }

    process::exit(1);
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
        if args.len() > 1 && args[1] == "--help" {
            usage_message();
        }

        if args.len() < 2 {
            eprintln!("ERROR: not enough arguments");
            usage_message();
        }

        if args.len() > 3 {
            eprintln!("ERROR: too many arguments");
            usage_message();
        }

        let mut file_path = args[1].clone();
        let mut flag = None;

        if args.len() == 3 {
            file_path = args[2].clone();
            flag = match args[1].as_str() {
                "--lex" => Some(Flag::Lex),
                "--parse" => Some(Flag::Parse),
                "--codegen" => Some(Flag::Codegen),
                "-S" => Some(Flag::Assembly),
                _ => {
                    eprintln!("ERROR: flag `{}` not recognized", args[1].as_str());
                    usage_message();
                }
            };
        }

        Arguments { file_path, flag }
    }
}

fn usage_message() -> ! {
    eprintln!(
        "Usage: crust [OPTIONS] <file.c>

Options:
    --lex        Run lexer only and print tokens
    --parse      Run lexer + parser and print AST
    --codegen    Run up to assembly generation and print result
    -S           Emit assembly file (.s) but do not remove it
    --help       Show this help message
"
    );
    std::process::exit(1);
}
