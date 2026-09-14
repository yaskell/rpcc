use crate::asm;

pub fn emit(program: asm::Program) -> String {
    let mut buffer = String::new();

    program
        .functions
        .into_iter()
        .for_each(|f| buffer.push_str(emit_function(f).as_str()));

    buffer.push_str("    .section .note.GNU-stack,\"\",@progbits");

    buffer
}

fn emit_function(fun: asm::Function) -> String {
    format!(
        "    .globl {}
  {}:
    pushq   %rbp
    movq    %rsp, %rbp
{}
",
        fun.name,
        fun.name,
        emit_instructions(fun.instructions)
    )
}

fn emit_instructions(instructions: Vec<asm::Instruction>) -> String {
    let mut b = String::new();
    for instruction in instructions {
        let s: String = match instruction {
            asm::Instruction::Move { src, dst } => format!(
                "movl    {}, {}",
                emit_operand(src, RegisterWidth::Bits32),
                emit_operand(dst, RegisterWidth::Bits32)
            ),
            asm::Instruction::Ret => String::from(
                "movq    %rbp, %rsp
    popq    %rbp
    ret",
            ),
            asm::Instruction::Unary { op, operand } => format!(
                "{}    {}",
                emit_unary_op(op),
                emit_operand(operand, RegisterWidth::Bits32)
            ),
            asm::Instruction::AllocateStack(i) => format!("subq    ${i}, %rsp"),
            asm::Instruction::Binary { op, left, right } => format!(
                "{}    {}, {}",
                emit_binary_op(op),
                emit_operand(left, RegisterWidth::Bits32),
                emit_operand(right, RegisterWidth::Bits32)
            ),
            asm::Instruction::Idiv(operand) => {
                format!("idivl    {}", emit_operand(operand, RegisterWidth::Bits32))
            }
            asm::Instruction::Cdq => "cdq".to_string(),
            asm::Instruction::Cmp { left, right } => format!(
                "cmpl    {}, {}",
                emit_operand(left, RegisterWidth::Bits32),
                emit_operand(right, RegisterWidth::Bits32)
            ),
            asm::Instruction::Jmp(l) => format!("jmp    .L{l}"),
            asm::Instruction::JmpCC { cond_code, target } => {
                format!("j{}    .L{}", emit_conditional_code(cond_code), target)
            }
            asm::Instruction::SetCC { cond_code, operand } => format!(
                "set{}    {}",
                emit_conditional_code(cond_code),
                emit_operand(operand, RegisterWidth::Bits8)
            ),
            asm::Instruction::Label(l) => format!(".L{l}:"),
            asm::Instruction::DeallocateStack(int) => format!("addq    ${int}, %rsp"),
            asm::Instruction::Push(operand) => {
                format!("pushq    {}", emit_operand(operand, RegisterWidth::Bits64))
            }
            asm::Instruction::Call(label) => format!("call {label}@PLT"),
        };
        b.push_str(&format!("    {s}\n"));
    }
    b
}

pub enum RegisterWidth {
    Bits64,
    Bits32,
    Bits8,
}

fn emit_operand(operand: asm::Operand, register_width: RegisterWidth) -> String {
    match operand {
        asm::Operand::Imm(i) => format!("${i}"),
        asm::Operand::Stack(i) => format!("{i}(%rbp)"),

        asm::Operand::Register(register) => String::from(match register_width {
            RegisterWidth::Bits64 => match register {
                asm::Register::AX => "%rax",
                asm::Register::CX => "%rcx",
                asm::Register::DX => "%rdx",
                asm::Register::DI => "%rdi",
                asm::Register::SI => "%rsi",
                asm::Register::R8 => "%r8",
                asm::Register::R9 => "%r9",
                asm::Register::R10 => "%r10",
                asm::Register::R11 => "%r11",
            },

            RegisterWidth::Bits32 => match register {
                asm::Register::AX => "%eax",
                asm::Register::CX => "%ecx",
                asm::Register::DX => "%edx",
                asm::Register::DI => "%edi",
                asm::Register::SI => "%esi",
                asm::Register::R8 => "%r8d",
                asm::Register::R9 => "%r9d",
                asm::Register::R10 => "%r10d",
                asm::Register::R11 => "%r11d",
            },

            RegisterWidth::Bits8 => match register {
                asm::Register::AX => "%al",
                asm::Register::CX => "%cl",
                asm::Register::DX => "%dl",
                asm::Register::DI => "%dil",
                asm::Register::SI => "%sil",
                asm::Register::R8 => "%r8b",
                asm::Register::R9 => "%r9b",
                asm::Register::R10 => "%r10b",
                asm::Register::R11 => "%r11b",
            },
        }),
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
