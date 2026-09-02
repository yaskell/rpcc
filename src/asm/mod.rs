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
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub stack_size: i32,
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
    Idiv(Operand),
    Cdq,
    Cmp {
        left: Operand,
        right: Operand,
    },
    Jmp(Identifier),
    JmpCC {
        cond_code: ConditionalCode,
        target: Identifier,
    },
    SetCC {
        cond_code: ConditionalCode,
        operand: Operand,
    },
    Label(Identifier),
    DeallocateStack(Int),
    Push(Operand),
    Call(Identifier),
    Ret,
}

#[derive(Debug)]
pub enum ConditionalCode {
    E,
    NE,
    G,
    GE,
    L,
    LE,
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operand {
    Imm(i32),
    Register(Register),
    Pseudo(Identifier),
    Stack(Int),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Register {
    AX,
    CX,
    DX,
    DI,
    SI,
    R8,
    R9,
    R10,
    R11,
}
