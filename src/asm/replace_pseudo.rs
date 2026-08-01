use crate::asm;
use std::collections::HashMap;

pub fn replace_pseudo_registers(program: asm::Program) -> (asm::Program, i32) {
    let mut allocator = StackAllocator::new();
    (
        allocator.replace_program(program),
        (allocator.next_offset + 4).abs(),
    )
}

struct StackAllocator {
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

    fn replace_program(&mut self, program: asm::Program) -> asm::Program {
        asm::Program {
            function: self.replace_function(program.function),
        }
    }

    fn replace_function(&mut self, function: asm::Function) -> asm::Function {
        asm::Function {
            name: function.name,
            instructions: function
                .instructions
                .into_iter()
                .map(|instruction| self.replace_instruction(instruction))
                .collect(),
        }
    }

    fn replace_instruction(&mut self, instruction: asm::Instruction) -> asm::Instruction {
        match instruction {
            asm::Instruction::Unary { op, operand } => asm::Instruction::Unary {
                op,
                operand: self.replace_operand(operand),
            },
            asm::Instruction::Move { src, dst } => asm::Instruction::Move {
                src: self.replace_operand(src),
                dst: self.replace_operand(dst),
            },
            asm::Instruction::Binary { op, left, right } => asm::Instruction::Binary {
                op,
                left: self.replace_operand(left),
                right: self.replace_operand(right),
            },
            asm::Instruction::Idiv(operand) => {
                asm::Instruction::Idiv(self.replace_operand(operand))
            }
            asm::Instruction::Cmp { left, right } => asm::Instruction::Cmp {
                left: self.replace_operand(left),
                right: self.replace_operand(right),
            },
            asm::Instruction::SetCC { cond_code, operand } => asm::Instruction::SetCC {
                cond_code,
                operand: self.replace_operand(operand),
            },
            instruction_without_operands => instruction_without_operands,
        }
    }

    fn replace_operand(&mut self, operand: asm::Operand) -> asm::Operand {
        match operand {
            asm::Operand::Pseudo(name) => asm::Operand::Stack(self.offset_for(&name)),
            other => other,
        }
    }
}
