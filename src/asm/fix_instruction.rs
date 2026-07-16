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
                    .map(|instruction| fix_instruction(instruction)),
            )
            .collect(),
    }
}

pub fn fix_instruction(instruction: asm::Instruction) -> asm::Instruction {
    return instruction;
}
