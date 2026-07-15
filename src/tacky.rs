use crate::parser;

type Identifier = String;
type Int = i32;
type Src = Val;
type Dst = Val;

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
    Unary(UnaryOp, Src, Dst),
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
    Program {
        function: translate_function(program.function),
    }
}

fn translate_function(function: parser::Function) -> Function {
    Function {
        identifier: translate_identifier(function.name),
        body: translate_statement(function.body),
    }
}

fn translate_identifier(identifier: Identifier) -> String {
    identifier
}

fn translate_statement(statement: parser::Statement) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match statement {
        parser::Statement::Return(expression) => {
            //FIXME: count will reset on new statement
            let value = translate_expression(expression, &mut instructions, 0);
            instructions.push(Instruction::Return(value))
        }
    };
    instructions
}

fn translate_expression(
    expression: parser::Expression,
    instructions: &mut Vec<Instruction>,
    i: i32,
) -> Val {
    match expression {
        parser::Expression::Constant(int) => return Val::Constant(int),
        parser::Expression::Unary(unary_op, expression) => {
            let src = translate_expression(*expression, instructions, i + 1);
            let dst = Val::Var(format!("tmp.{i}").to_string());

            match unary_op {
                parser::UnaryOp::Complement => {
                    instructions.push(Instruction::Unary(UnaryOp::Complement, src, dst.clone()))
                }
                parser::UnaryOp::Negate => {
                    instructions.push(Instruction::Unary(UnaryOp::Negate, src, dst.clone()))
                }
            }
            return dst;
        }
    }
}
