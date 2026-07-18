pub mod fix_instruction;
pub mod replace_pseudo;
pub mod translate;

pub use fix_instruction::fix_program;
pub use replace_pseudo::replace_pseudo_registers;
pub use translate::translate_program;

pub type Identifier = String;

pub type Int = i32;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Move {
        src: Operand,
        dst: Operand,
    },
    Unary {
        op: UnaryOp,
        operand: Operand,
    },
    Binary {
        op: BinaryOp,
        left: Operand,
        right: Operand,
    },
    AllocateStack(Int),
    Ret,
    Idiv(Operand),
    Cdq,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mult,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i32),
    Register(Register),
    Pseudo(Identifier),
    Stack(Int),
}

#[derive(Debug)]
pub enum Register {
    AX,
    DX,
    R10,
    R11,
}
