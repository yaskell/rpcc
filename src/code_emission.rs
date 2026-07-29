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
        let s: String = match instruction {
            asm::Instruction::Move { src, dst } => format!(
                "    movl    {}, {}",
                emit_operand(src, false),
                emit_operand(dst, false)
            ),
            asm::Instruction::Ret => String::from(
                "    movq    %rbp, %rsp
    popq    %rbp
    ret",
            ),
            asm::Instruction::Unary { op, operand } => format!(
                "    {}    {}",
                emit_unary_op(op),
                emit_operand(operand, false)
            ),
            asm::Instruction::AllocateStack(i) => format!("subq    ${i}, %rsp"),
            asm::Instruction::Binary { op, left, right } => format!(
                "    {}    {}, {}",
                emit_binary_op(op),
                emit_operand(left, false),
                emit_operand(right, false)
            ),
            asm::Instruction::Idiv(operand) => {
                format!("    idivl    {}", emit_operand(operand, false))
            }
            asm::Instruction::Cdq => format!("    cdq"),
            asm::Instruction::Cmp { left, right } => format!(
                "    cmpl    {}, {}",
                emit_operand(left, false),
                emit_operand(right, false)
            ),
            asm::Instruction::Jmp(l) => format!("    jmp    .L{}", l),
            asm::Instruction::JmpCC { cond_code, target } => {
                format!("    j{}    .L{}", emit_conditional_code(cond_code), target)
            }
            asm::Instruction::SetCC { cond_code, operand } => format!(
                "    set{}    {}",
                emit_conditional_code(cond_code),
                emit_operand(operand, true)
            ),
            asm::Instruction::Label(l) => format!(".L{l}:"),
        };
        b.push_str(&format!("{}\n", &s));
    }
    return b;
}

fn emit_operand(operand: asm::Operand, emit_one_byte_register: bool) -> String {
    match operand {
        asm::Operand::Imm(i) => format!("${i}"),
        asm::Operand::Stack(i) => format!("{i}(%rbp)"),
        asm::Operand::Register(register) => match emit_one_byte_register {
            true => match register {
                asm::Register::AX => String::from("%al"),
                asm::Register::R10 => String::from("%r10b"),
                asm::Register::DX => String::from("%dl"),
                asm::Register::R11 => String::from("%r11d"),
            },
            false => match register {
                asm::Register::AX => String::from("%eax"),
                asm::Register::R10 => String::from("%r10d"),
                asm::Register::DX => String::from("%edx"),
                asm::Register::R11 => String::from("%r11d"),
            },
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

fn emit_conditional_code(code: asm::ConditionalCode) -> String {
    match code {
        asm::ConditionalCode::E => String::from("e"),
        asm::ConditionalCode::NE => String::from("ne"),
        asm::ConditionalCode::G => String::from("g"),
        asm::ConditionalCode::GE => String::from("ge"),
        asm::ConditionalCode::L => String::from("l"),
        asm::ConditionalCode::LE => String::from("le"),
    }
}
