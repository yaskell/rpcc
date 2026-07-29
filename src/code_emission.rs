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
            true => String::from(match register {
                asm::Register::AX => "%al",
                asm::Register::R10 => "%r10b",
                asm::Register::DX => "%dl",
                asm::Register::R11 => "%r11d",
            }),
            false => String::from(match register {
                asm::Register::AX => "%eax",
                asm::Register::R10 => "%r10d",
                asm::Register::DX => "%edx",
                asm::Register::R11 => "%r11d",
            }),
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
    String::from(match op {
        asm::BinaryOp::Add => "addl",
        asm::BinaryOp::Sub => "subl",
        asm::BinaryOp::Mult => "imull",
    })
}

fn emit_conditional_code(code: asm::ConditionalCode) -> String {
    String::from(match code {
        asm::ConditionalCode::E => "e",
        asm::ConditionalCode::NE => "ne",
        asm::ConditionalCode::G => "g",
        asm::ConditionalCode::GE => "ge",
        asm::ConditionalCode::L => "l",
        asm::ConditionalCode::LE => "le",
    })
}
