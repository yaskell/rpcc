use crate::asm;

pub fn emit(program: asm::Program) -> String {
    let mut buffer = String::new();

    buffer.push_str(emit_function(program.function).as_str());

    buffer.push_str("    .section .note.GNU-stack,\"\",@progbits");

    buffer
}

fn emit_function(fun: asm::Function) -> String {
    return format!(
        "    .globl {}
  {}:
    pushq   %rbp
    movq    %rsp, %rbp
    {}

",
        fun.name,
        fun.name,
        emit_instructions(fun.instructions)
    );
}

fn emit_instructions(instructions: Vec<asm::Instruction>) -> String {
    let mut b = String::new();
    for instruction in instructions {
        match instruction {
            asm::Instruction::Move { src, dst } => b.push_str(
                format!("    movl    {}, {}\n", emit_operand(src), emit_operand(dst)).as_str(),
            ),
            asm::Instruction::Ret => b.push_str(
                "    movq    %rbp, %rsp
    popq    %rbp
    ret",
            ),
            asm::Instruction::Unary { op, operand } => b.push_str(
                format!("    {}    {}\n", emit_unary_op(op), emit_operand(operand)).as_str(),
            ),
            asm::Instruction::AllocateStack(i) => {
                b.push_str(format!("subq    ${i}, %rsp\n").as_str())
            }
            asm::Instruction::Binary { op, left, right } => todo!(),
            asm::Instruction::Idiv(operand) => todo!(),
            asm::Instruction::Cdq => todo!(),
        };
    }
    return b;
}

fn emit_operand(operand: asm::Operand) -> String {
    match operand {
        asm::Operand::Imm(i) => format!("${i}"),
        asm::Operand::Stack(i) => format!("{i}(%rbp)"),
        asm::Operand::Register(register) => match register {
            asm::Register::AX => String::from("%eax"),
            asm::Register::R10 => String::from("%r10d"),
            asm::Register::DX => todo!(),
            asm::Register::R11 => todo!(),
        },
        asm::Operand::Pseudo(_) => {
            unreachable!("All pseudo registers should've been replaced during assembly generation")
        }
    }
}

fn emit_unary_op(op: asm::UnaryOp) -> String {
    match op {
        asm::UnaryOp::Neg => String::from("negl"),
        asm::UnaryOp::Not => String::from("notl"),
    }
}
