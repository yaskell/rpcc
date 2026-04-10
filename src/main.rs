mod assembly_generation;
mod code_emission;
mod lexer;
mod parser;

use std::env;
use std::fs;
use std::process;

use crate::assembly_generation::translate_program;
use crate::code_emission::emit;
use crate::lexer::lex;

use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args);
    let file_content = fs::read_to_string(&arguments.file_path).expect("Could not read file");
    let filename = &arguments.file_path.trim_end_matches(".c");

    let mut stop_before_parsing: bool = false;
    let mut stop_before_assembly_generation: bool = false;
    let mut stop_before_code_emission: bool = false;
    let mut emit_assembly_file: bool = false;

    match arguments.flag {
        Some(Flag::Lex) => stop_before_parsing = true,
        Some(Flag::Parse) => stop_before_assembly_generation = true,
        Some(Flag::Codegen) => stop_before_code_emission = true,
        Some(Flag::Assembly) => emit_assembly_file = true,
        None => {}
    };

    let lexed_val = lex(file_content);
    if stop_before_parsing {
        println!("Stopped before parsing");
        println!("Lexed file contents: {:?}", &lexed_val);
        process::exit(0);
    }
    let parsed_val = parser::parse(&mut lexed_val.clone());
    if stop_before_assembly_generation {
        println!("Stopped before generating assembly");
        println!("Parsed file contents: {:?}", &parsed_val);
        process::exit(0);
    }
    let asm_generation_val = translate_program(parsed_val);
    if stop_before_code_emission {
        println!("Stopped before code emission");
        println!("Assembly Generation: {:?}", &asm_generation_val);
        process::exit(0);
    }

    if let Ok(_) = emit(filename, asm_generation_val) {
        Command::new("gcc")
            .args([format!("{}.s", filename).as_str(), "-o", filename])
            .output()
            .expect("has failed");

        if !emit_assembly_file {
            let _ = fs::remove_file(format!("{}.s", filename).as_str());
        }

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
                    process::exit(1);
                }
            };
        }

        Arguments { file_path, flag }
    }
}

fn usage_message() {
    eprintln!("Usage: [--lex | --parse | --codegen | -S] <path>");
}
