#![feature(variant_count)]

mod lexer;
mod parser;
mod assembly_generation;
mod code_emission;

use std::process;
use std::fs;
use std::env;

use crate::assembly_generation::translate_program;
use crate::lexer::lex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args);
    let file_content = fs::read_to_string(&arguments.file_path).expect("Could not read file");

    let mut stop_before_parsing: bool = false;
    let mut stop_before_assembly_generation: bool = false;
    let mut stop_before_code_emission: bool = false;
    let mut emit_assembly_file: bool = false;

    match arguments.flag {
        Some(Flag::Lex) => stop_before_parsing = true,
        Some(Flag::Parse) => stop_before_assembly_generation = true,
        Some(Flag::Codegen) => stop_before_code_emission = true,
        Some(Flag::Assembly) => emit_assembly_file = true,
        None => {},
    };

    let lexed_val = lex(file_content);
    println!("Lexed file contents: {:?}", &lexed_val);
    if stop_before_parsing { println!("Stopped before parsing"); process::exit(0); }
    let parsed_val = parser::parse(&mut lexed_val.clone());
    println!("Parsed file contents: {:?}", &parsed_val);
    if stop_before_assembly_generation { println!("Stopped before generating assembly"); process::exit(0); }
    println!("Assembly Generation: {:?}", translate_program(parsed_val));
    if stop_before_code_emission { println!("Stopped before code emission"); process::exit(0); }
    if emit_assembly_file { todo!() }
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

        let mut file_path = args[2].clone();
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

