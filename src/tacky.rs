use crate::parser;

type Identifier = String;
type Int = i32;

pub struct TemporaryVarAllocator {
    count: i32,
}

impl TemporaryVarAllocator {
    fn new() -> TemporaryVarAllocator {
        TemporaryVarAllocator { count: 0 }
    }
}

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub identifier: Identifier,
    pub body: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Return(Val),
    Unary { op: UnaryOp, src: Val, dst: Val },
}

#[derive(Debug, Clone)]
pub enum Val {
    Constant(Int),
    Var(Identifier),
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
}

pub fn translate_program(program: parser::Program) -> Program {
    let mut tva = TemporaryVarAllocator::new();
    Program {
        function: translate_function(program.function, &mut tva),
    }
}

fn translate_function(function: parser::Function, tva: &mut TemporaryVarAllocator) -> Function {
    Function {
        identifier: translate_identifier(function.name),
        body: translate_statement(function.body, tva),
    }
}

fn translate_identifier(identifier: Identifier) -> String {
    identifier
}

fn translate_statement(
    statement: parser::Statement,
    tva: &mut TemporaryVarAllocator,
) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match statement {
        parser::Statement::Return(expression) => {
            let value = translate_expression(expression, &mut instructions, tva);
            instructions.push(Instruction::Return(value))
        }
    };
    instructions
}

fn translate_expression(
    expression: parser::Expression,
    instructions: &mut Vec<Instruction>,
    tva: &mut TemporaryVarAllocator,
) -> Val {
    match expression {
        parser::Expression::Constant(int) => Val::Constant(int),
        parser::Expression::Unary { operator, operand } => {
            let dst = Val::Var(format!("tmp.{}", tva.count).to_string());
            let src = {
                tva.count += 1;
                translate_expression(*operand, instructions, tva)
            };

            match operator {
                parser::UnaryOp::Complement => instructions.push(Instruction::Unary {
                    op: UnaryOp::Complement,
                    src,
                    dst: dst.clone(),
                }),
                parser::UnaryOp::Negate => instructions.push(Instruction::Unary {
                    op: UnaryOp::Negate,
                    src,
                    dst: dst.clone(),
                }),
            }
            dst
        }
    }
}
