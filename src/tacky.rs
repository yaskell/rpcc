use crate::parser;

type Identifier = String;
type Int = i32;
type Src = Val;
type Dst = Val;

#[derive(Debug)]
pub struct Program {
    pub function_definition: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub identifier: Identifier,
    pub body: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Return(Val),
    Unary(UnaryOperator, Src, Dst),
}

#[derive(Debug)]
pub enum Val {
    Constant(Int),
    Var(Identifier),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

pub fn translate_program(program: parser::Program) -> Program {
    Program {
        function_definition: translate_function_definition(program.function_definition),
    }
}

fn translate_function_definition(
    function_definition: parser::FunctionDefinition,
) -> FunctionDefinition {
    FunctionDefinition {
        identifier: translate_identifier(function_definition.name),
        body: translate_statement(function_definition.body),
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
        parser::Expression::Unary(unary_operator, expression) => {
            let src = translate_expression(*expression, instructions, i + 1);
            let dst = format!("tmp.{i}");

            match unary_operator {
                parser::UnaryOperator::Complement => instructions.push(Instruction::Unary(
                    UnaryOperator::Complement,
                    src,
                    Val::Var(dst.to_string()),
                )),
                parser::UnaryOperator::Negate => instructions.push(Instruction::Unary(
                    UnaryOperator::Negate,
                    src,
                    Val::Var(dst.to_string()),
                )),
            }
            return Val::Var(dst.to_string());
        }
    }
}
