pub mod replace_pseudo;
pub mod translate;

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
    Unary { op: UnaryOp, operand: Operand },
    AllocateStack(Int),
    Move { src: Operand, dst: Operand },
    Ret,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
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
    R10,
}
