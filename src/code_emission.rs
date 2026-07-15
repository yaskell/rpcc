use crate::asm;
use std::fs;

pub fn emit(program_name: &str, program: asm::Program) -> std::io::Result<()> {
    let mut buffer = String::new();

    buffer.push_str(emit_function(program.function).as_str());

    buffer.push_str("    .section .note.GNU-stack,\"\",@progbits");
    fs::write(format!("{}.s", program_name), buffer)?;
    Ok(())
}

fn emit_function(fun: asm::Function) -> String {
    return format!(
        "    .globl {}\n{}:\n{}",
        fun.name,
        fun.name,
        emit_instructions(fun.instructions)
    );
}

fn emit_instructions(instructions: Vec<asm::Instruction>) -> String {
    let mut b = String::new();
    for instruction in instructions {
        match instruction {
            asm::Instruction::Move(m) => b.push_str(
                format!(
                    "    movl    {}, {}\n",
                    emit_operand(m.src),
                    emit_operand(m.dst)
                )
                .as_str(),
            ),
            asm::Instruction::Ret => b.push_str("    ret\n"),
            asm::Instruction::Unary(asm::UnaryInstruction { op, operand }) => todo!(),
            asm::Instruction::AllocateStack(_) => todo!(),
        };
    }
    return b;
}

fn emit_operand(operand: asm::Operand) -> String {
    match operand {
        asm::Operand::Imm(i) => format!("${}", i),
        asm::Operand::Register(register) => match register {
            asm::Register::AX => String::from("%eax"),
            _ => todo!(),
        },
        asm::Operand::Pseudo(_) => todo!(),
        asm::Operand::Stack(_) => todo!(),
    }
}
