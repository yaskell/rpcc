use crate::asm;

pub fn fix_program(program: asm::Program, offset: i32) -> asm::Program {
    asm::Program {
        function: fix_function(program.function, offset),
    }
}

pub fn fix_function(function: asm::Function, offset: i32) -> asm::Function {
    asm::Function {
        name: function.name,
        instructions: std::iter::once(asm::Instruction::AllocateStack(offset))
            .chain(
                function
                    .instructions
                    .into_iter()
                    .flat_map(|instruction| fix_instruction(instruction)),
            )
            .collect(),
    }
}

pub fn fix_instruction(instruction: asm::Instruction) -> Vec<asm::Instruction> {
    if let asm::Instruction::Move { src, dst } = instruction {
        if let (asm::Operand::Stack(_), asm::Operand::Stack(_)) = (&src, &dst) {
            vec![
                asm::Instruction::Move {
                    src,
                    dst: asm::Operand::Register(asm::Register::R10),
                },
                asm::Instruction::Move {
                    src: asm::Operand::Register(asm::Register::R10),
                    dst,
                },
            ]
        } else {
            vec![asm::Instruction::Move { src, dst }]
        }
    } else {
        vec![instruction]
    }
}
