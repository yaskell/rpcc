use crate::parser;

type Identifier = String;
type Int = i32;

pub struct TemporaryVarAllocator {
    var_count: i32,
    label_count: i32,
}

impl TemporaryVarAllocator {
    fn new() -> TemporaryVarAllocator {
        TemporaryVarAllocator {
            var_count: 0,
            label_count: 0,
        }
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
    Unary {
        op: UnaryOp,
        src: Val,
        dst: Val,
    },
    Binary {
        op: BinaryOp,
        left: Val,
        right: Val,
        dst: Val,
    },
    Copy {
        src: Val,
        dst: Val,
    },
    Jump {
        target: Identifier,
    },
    JumpIfZero {
        condition: Val,
        target: Identifier,
    },
    JumpIfNotZero {
        condition: Val,
        target: Identifier,
    },
    Label(Identifier),
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
    Not,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
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
        body: function
            .body
            .into_iter()
            .flat_map(|block_item| translate_block_item(block_item, tva))
            .chain(std::iter::once(Instruction::Return(Val::Constant(0))))
            .collect(),
    }
}

fn translate_identifier(identifier: Identifier) -> String {
    identifier
}

fn translate_block_item(
    block_item: parser::BlockItem,
    tva: &mut TemporaryVarAllocator,
) -> Vec<Instruction> {
    match block_item {
        parser::BlockItem::S(statement) => translate_statement(statement, tva),
        parser::BlockItem::D(declaration) => translate_declaration(declaration, tva),
    }
}

fn translate_declaration(
    parser::Declaration { name, init }: parser::Declaration,
    tva: &mut TemporaryVarAllocator,
) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match init {
        Some(e) => {
            let var = parser::Expression::Assignment {
                lvalue: Box::new(parser::Expression::Var(name)),
                expression: Box::new(e),
            };
            translate_expression(var, &mut instructions, tva);
            instructions
        }
        None => instructions,
    }
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
        parser::Statement::Expression(expression) => {
            translate_expression(expression, &mut instructions, tva);
        }
        parser::Statement::Null => {}
        parser::Statement::If {
            condition,
            then,
            otherwise,
        } => {
            let label_count = tva.label_count;
            tva.label_count += 1;

            let c = translate_expression(condition, &mut instructions, tva);
            match otherwise {
                Some(otherwise) => {
                    instructions.push(Instruction::JumpIfZero {
                        condition: c,
                        target: String::from(format!("if_else{}", label_count)),
                    });
                    instructions.extend(translate_statement(*then, tva));

                    instructions.push(Instruction::Jump {
                        target: String::from(format!("if_end{}", label_count)),
                    });
                    instructions.push(Instruction::Label(String::from(format!(
                        "if_else{}",
                        label_count
                    ))));
                    instructions.extend(translate_statement(*otherwise, tva));
                }
                None => {
                    instructions.push(Instruction::JumpIfZero {
                        condition: c,
                        target: String::from(format!("if_end{}", label_count)),
                    });
                    instructions.extend(translate_statement(*then, tva));
                }
            }
            instructions.push(Instruction::Label(String::from(format!(
                "if_end{}",
                label_count
            ))));
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
        parser::Expression::Var(v) => Val::Var(v),
        parser::Expression::Assignment { lvalue, expression } => {
            let right = translate_expression(*expression, instructions, tva);
            let left = translate_expression(*lvalue, instructions, tva);
            instructions.push(Instruction::Copy {
                src: right,
                dst: left.clone(),
            });
            left
        }
        parser::Expression::Constant(int) => Val::Constant(int),
        parser::Expression::Unary { operator, operand } => {
            let dst = Val::Var(format!("tmp.{}", tva.var_count).to_string());
            let src = {
                tva.var_count += 1;
                translate_expression(*operand, instructions, tva)
            };

            let op = match operator {
                parser::UnaryOp::Complement => UnaryOp::Complement,
                parser::UnaryOp::Negate => UnaryOp::Negate,
                parser::UnaryOp::Not => UnaryOp::Not,
            };

            instructions.push(Instruction::Unary {
                op,
                src,
                dst: dst.clone(),
            });

            dst
        }
        parser::Expression::Binary {
            operator,
            left_expression,
            right_expression,
        } => match operator {
            parser::BinaryOp::And => {
                let dst = Val::Var(format!("tmp.{}", tva.var_count).to_string());
                tva.var_count += 1;

                let label_count = tva.label_count;
                tva.label_count += 1;

                let left = translate_expression(*left_expression, instructions, tva);
                instructions.push(Instruction::JumpIfZero {
                    condition: left,
                    target: String::from(format!("and_false{}", label_count)),
                });

                let right = translate_expression(*right_expression, instructions, tva);
                instructions.push(Instruction::JumpIfZero {
                    condition: right,
                    target: String::from(format!("and_false{}", label_count)),
                });

                instructions.push(Instruction::Copy {
                    src: Val::Constant(1),
                    dst: dst.clone(),
                });

                instructions.push(Instruction::Jump {
                    target: String::from(format!("and_end{}", label_count)),
                });

                instructions.push(Instruction::Label(String::from(format!(
                    "and_false{}",
                    label_count
                ))));

                instructions.push(Instruction::Copy {
                    src: Val::Constant(0),
                    dst: dst.clone(),
                });

                instructions.push(Instruction::Label(String::from(format!(
                    "and_end{}",
                    label_count
                ))));
                dst
            }
            parser::BinaryOp::Or => {
                let dst = Val::Var(format!("tmp.{}", tva.var_count).to_string());
                tva.var_count += 1;

                let label_count = tva.label_count;
                tva.label_count += 1;

                let left = translate_expression(*left_expression, instructions, tva);
                instructions.push(Instruction::JumpIfNotZero {
                    condition: left,
                    target: String::from(format!("or_false{}", label_count)),
                });

                let right = translate_expression(*right_expression, instructions, tva);
                instructions.push(Instruction::JumpIfNotZero {
                    condition: right,
                    target: String::from(format!("or_false{}", label_count)),
                });

                instructions.push(Instruction::Copy {
                    src: Val::Constant(0),
                    dst: dst.clone(),
                });

                instructions.push(Instruction::Jump {
                    target: String::from(format!("or_end{}", label_count)),
                });

                instructions.push(Instruction::Label(String::from(format!(
                    "or_false{}",
                    label_count
                ))));

                instructions.push(Instruction::Copy {
                    src: Val::Constant(1),
                    dst: dst.clone(),
                });

                instructions.push(Instruction::Label(String::from(format!(
                    "or_end{}",
                    label_count
                ))));
                dst
            }
            other_operator => {
                let dst = Val::Var(format!("tmp.{}", tva.var_count).to_string());
                tva.var_count += 1;
                let left = translate_expression(*left_expression, instructions, tva);
                let right = translate_expression(*right_expression, instructions, tva);
                let op = match other_operator {
                    parser::BinaryOp::Add => BinaryOp::Add,
                    parser::BinaryOp::Subtract => BinaryOp::Subtract,
                    parser::BinaryOp::Multiply => BinaryOp::Multiply,
                    parser::BinaryOp::Divide => BinaryOp::Divide,
                    parser::BinaryOp::Remainder => BinaryOp::Remainder,
                    parser::BinaryOp::Equal => BinaryOp::Equal,
                    parser::BinaryOp::NotEqual => BinaryOp::NotEqual,
                    parser::BinaryOp::LessThan => BinaryOp::LessThan,
                    parser::BinaryOp::LessOrEqual => BinaryOp::LessOrEqual,
                    parser::BinaryOp::GreaterThan => BinaryOp::GreaterThan,
                    parser::BinaryOp::GreaterOrEqual => BinaryOp::GreaterOrEqual,
                    parser::BinaryOp::And | parser::BinaryOp::Or | parser::BinaryOp::Assignment => {
                        unreachable!("Should've matched super branch")
                    }
                    parser::BinaryOp::Ternary => {
                        unreachable!("Ternary Expression not represented as BinaryOp in Tacky")
                    }
                };

                instructions.push(Instruction::Binary {
                    op,
                    left,
                    right,
                    dst: dst.clone(),
                });
                dst
            }
        },
        parser::Expression::Conditional {
            condition,
            then,
            otherwise,
        } => {
            let dst = Val::Var(format!("tmp.{}", tva.var_count).to_string());
            tva.var_count += 1;

            let label_count = tva.label_count;
            tva.label_count += 1;

            let condition = translate_expression(*condition, instructions, tva);

            instructions.push(Instruction::JumpIfZero {
                condition,
                target: String::from(format!("ternary_otherwise{}", label_count)),
            });

            let e1 = translate_expression(*then, instructions, tva);
            instructions.push(Instruction::Copy {
                src: e1,
                dst: dst.clone(),
            });

            instructions.push(Instruction::Jump {
                target: String::from(format!("ternary_end{}", label_count)),
            });

            instructions.push(Instruction::Label(String::from(format!(
                "ternary_otherwise{}",
                label_count
            ))));

            let e2 = translate_expression(*otherwise, instructions, tva);
            instructions.push(Instruction::Copy {
                src: e2,
                dst: dst.clone(),
            });

            instructions.push(Instruction::Label(String::from(format!(
                "ternary_end{}",
                label_count
            ))));
            dst
        }
    }
}
