use crate::asm;
use std::collections::HashMap;

pub struct StackAllocator {
    identifier_offsets: HashMap<String, i32>,
    next_offset: i32,
}

impl StackAllocator {
    fn new() -> Self {
        Self {
            identifier_offsets: HashMap::new(),
            next_offset: -4,
        }
    }

    fn offset_for(&mut self, name: &String) -> i32 {
        match self.identifier_offsets.get(name) {
            Some(offset) => *offset,
            None => {
                let offset = self.next_offset;
                self.identifier_offsets.insert(name.clone(), offset);
                self.next_offset -= 4;
                offset
            }
        }
    }
}

pub fn replace_pseudo_registers(program: asm::Program) -> asm::Program {
    asm::Program {
        function: replace_function(program.function),
    }
}

fn replace_function(function: asm::Function) -> asm::Function {
    let mut allocator = StackAllocator::new();

    asm::Function {
        name: function.name,
        instructions: function
            .instructions
            .into_iter()
            .map(|instruction| replace_instruction(instruction, &mut allocator))
            .collect(),
    }
}

fn replace_instruction(
    instruction: asm::Instruction,
    allocator: &mut StackAllocator,
) -> asm::Instruction {
    match instruction {
        asm::Instruction::Unary { op, operand } => asm::Instruction::Unary {
            op,
            operand: replace_operand(operand, allocator),
        },
        asm::Instruction::Move { src, dst } => asm::Instruction::Move {
            src: replace_operand(src, allocator),
            dst: replace_operand(dst, allocator),
        },
        asm::Instruction::AllocateStack(size) => asm::Instruction::AllocateStack(size),
        asm::Instruction::Ret => asm::Instruction::Ret,
    }
}

fn replace_operand(operand: asm::Operand, allocator: &mut StackAllocator) -> asm::Operand {
    match operand {
        asm::Operand::Pseudo(name) => asm::Operand::Stack(allocator.offset_for(&name)),
        other => other,
    }
}
