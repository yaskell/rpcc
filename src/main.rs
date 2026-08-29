mod asm;
mod code_emission;
mod lexer;
mod parser;
mod semantic_analysis;
mod tacky;

use std::env;
use std::fs;
use std::process;

use crate::semantic_analysis::label_loops;
use crate::semantic_analysis::resolve_variables;

fn main() {
    let arguments = Arguments::new(&env::args().collect::<Vec<String>>());
    let filename_base = &arguments.file_path.trim_end_matches(".c");

    run_command(
        "gcc",
        &[
            "-E",
            "-P",
            &arguments.file_path,
            "-o",
            format!("{filename_base}.i").as_str(),
        ],
    )
    .expect("preprocessing failed");

    let file_content =
        fs::read_to_string(format!("{filename_base}.i")).expect("Could not read file");

    let _ = fs::remove_file(format!("{}.i", filename_base).as_str());

    let mut tokens = lexer::lex(&file_content);
    if let Some(Flag::Lex) = arguments.flag {
        println!("Stopped before parsing");
        println!("Lexed file contents:");
        dbg!(&tokens);
        process::exit(0);
    }

    let ast = parser::parse(&mut tokens);
    if let Some(Flag::Parse) = arguments.flag {
        println!("Stopped before generating assembly");
        println!("Parsed file contents:",);
        dbg!(&ast);
        process::exit(0);
    }

    let ast = resolve_variables(ast);
    let ast = label_loops(ast);
    if let Some(Flag::Validate) = arguments.flag {
        println!("Stopped generating tacky, did semantic analysis");
        println!("Validated file contents:");
        dbg!(&ast);
        process::exit(0);
    }

    let tacky_ir = tacky::translate_program(ast);
    if let Some(Flag::Tacky) = arguments.flag {
        println!("Stopped before generating assembly, ran tacky compilation pass");
        println!("Parsed file contents:");
        dbg!(&tacky_ir);
        process::exit(0);
    }

    let asm_ast = asm::translate_program(tacky_ir);
    let (asm_ast, offset) = asm::replace_pseudo_registers(asm_ast);
    let asm_ast = asm::fix_program(asm_ast, offset);
    if let Some(Flag::Codegen) = arguments.flag {
        println!("Stopped before code emission");
        println!("Assembly Generation:");
        dbg!(&asm_ast);
        process::exit(0);
    }

    let program = code_emission::emit(asm_ast);
    if fs::write(format!("{filename_base}.s"), program).is_ok() {
        if let Some(Flag::Object) = arguments.flag {
            run_command(
                "gcc",
                &[
                    "-c",
                    format!("{filename_base}.s").as_str(),
                    "-o",
                    format!("{filename_base}.o").as_str(),
                ],
            )
            .expect("Linking failed");
        } else {
            run_command(
                "gcc",
                &[format!("{filename_base}.s").as_str(), "-o", filename_base],
            )
            .expect("Linking failed");
        }

        if let Some(Flag::Assembly) = arguments.flag {
            process::exit(0);
        }

        let _ = fs::remove_file(format!("{filename_base}.s").as_str());

        process::exit(0);
    }

    process::exit(1);
}

enum Flag {
    Lex,
    Parse,
    Validate,
    Codegen,
    Assembly,
    Tacky,
    Object,
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
                "--validate" => Some(Flag::Validate),
                "--codegen" => Some(Flag::Codegen),
                "-S" => Some(Flag::Assembly),
                "--tacky" => Some(Flag::Tacky),
                "-c" => Some(Flag::Object),
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
    --validate   Run lexer + parser + semantic analysis and print AST
    --tacky      Run tacky compiler pass, stop before assembly generation
    --codegen    Run up to assembly generation and print result
    -S           Emit assembly file (.s) but do not remove it
    -S           Emit object file (.o)
    --help       Show this help message
"
    );
    std::process::exit(1);
}

fn run_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = process::Command::new(command)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
