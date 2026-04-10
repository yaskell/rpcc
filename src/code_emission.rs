use crate::assembly_generation::{
    ASMFunctionDefinition, ASMProgram, Instruction, Operand, Register,
};
use std::fs;

pub fn emit(program_name: &str, program: ASMProgram) -> std::io::Result<()> {
    let mut buffer = String::new();

    buffer.push_str(emit_function(program.function_definition).as_str());

    buffer.push_str("    .section .note.GNU-stack,\"\",@progbits");
    fs::write(format!("{}.s", program_name), buffer)?;
    Ok(())
}

fn emit_function(fun: ASMFunctionDefinition) -> String {
    return format!(
        "    .globl {}\n{}:\n{}",
        fun.name,
        fun.name,
        emit_instructions(fun.instructions)
    );
}

fn emit_instructions(instructions: Vec<Instruction>) -> String {
    let mut b = String::new();
    for instruction in instructions {
        match instruction {
            Instruction::Move(m) => b.push_str(
                format!(
                    "    movl    {}, {}\n",
                    emit_operand(m.src),
                    emit_operand(m.dst)
                )
                .as_str(),
            ),
            Instruction::Ret => b.push_str("    ret\n"),
        };
    }
    return b;
}

fn emit_operand(operand: Operand) -> String {
    match operand {
        Operand::Imm(i) => format!("${}", i),
        Operand::Register(register) => match register {
            Register::EAX => String::from("%eax"),
        },
    }
}
