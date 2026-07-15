pub mod translate;

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
    Unary(UnaryInstruction),
    AllocateStack(Int),
    Move(MoveInstruction),
    Ret,
}

#[derive(Debug)]
pub struct MoveInstruction {
    pub src: Operand,
    pub dst: Operand,
}

#[derive(Debug)]
pub struct UnaryInstruction {
    pub op: UnaryOp,
    pub operand: Operand,
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
