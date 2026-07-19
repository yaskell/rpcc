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
                format!("    movl    {}, {}", emit_operand(src), emit_operand(dst)).as_str(),
            ),
            asm::Instruction::Ret => b.push_str(
                "    movq    %rbp, %rsp
    popq    %rbp
    ret",
            ),
            asm::Instruction::Unary { op, operand } => b.push_str(
                format!("    {}    {}", emit_unary_op(op), emit_operand(operand)).as_str(),
            ),
            asm::Instruction::AllocateStack(i) => {
                b.push_str(format!("subq    ${i}, %rsp").as_str())
            }
            asm::Instruction::Binary { op, left, right } => b.push_str(
                format!(
                    "    {}    {}, {}",
                    emit_binary_op(op),
                    emit_operand(left),
                    emit_operand(right)
                )
                .as_str(),
            ),
            asm::Instruction::Idiv(operand) => {
                b.push_str(format!("    idivl    {}", emit_operand(operand)).as_str())
            }
            asm::Instruction::Cdq => b.push_str(format!("    cdq").as_str()),
        };
        b.push_str("\n");
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
            asm::Register::DX => String::from("%edx"),
            asm::Register::R11 => String::from("%r11d"),
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

fn emit_binary_op(op: asm::BinaryOp) -> String {
    match op {
        asm::BinaryOp::Add => String::from("addl"),
        asm::BinaryOp::Sub => String::from("subl"),
        asm::BinaryOp::Mult => String::from("imull"),
    }
}
