use crate::parser;

type Identifier = String;
type Int = i32;

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
    TackyTranslator::new().translate_program(program)
}

pub struct TackyTranslator {
    var_count: i32,
    label_count: i32,
}

impl TackyTranslator {
    fn new() -> TackyTranslator {
        TackyTranslator {
            var_count: 0,
            label_count: 0,
        }
    }

    fn translate_program(&mut self, program: parser::Program) -> Program {
        Program {
            function: self.translate_function(program.function),
        }
    }

    fn translate_function(&mut self, function: parser::Function) -> Function {
        Function {
            identifier: self.translate_identifier(function.name),
            body: function
                .body
                .0
                .into_iter()
                .flat_map(|block_item| self.translate_block_item(block_item))
                .chain(std::iter::once(Instruction::Return(Val::Constant(0))))
                .collect(),
        }
    }

    fn translate_identifier(&self, identifier: Identifier) -> String {
        identifier
    }

    fn translate_block_item(&mut self, block_item: parser::BlockItem) -> Vec<Instruction> {
        match block_item {
            parser::BlockItem::S(statement) => self.translate_statement(statement),
            parser::BlockItem::D(declaration) => self.translate_declaration(declaration),
        }
    }

    fn translate_declaration(
        &mut self,
        parser::Declaration { name, init }: parser::Declaration,
    ) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        match init {
            Some(e) => {
                let var = parser::Expression::Assignment {
                    lvalue: Box::new(parser::Expression::Var(name)),
                    expression: Box::new(e),
                };
                self.translate_expression(var, &mut instructions);
                instructions
            }
            None => instructions,
        }
    }

    fn translate_statement(&mut self, statement: parser::Statement) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        match statement {
            parser::Statement::Return(expression) => {
                let value = self.translate_expression(expression, &mut instructions);
                instructions.push(Instruction::Return(value))
            }
            parser::Statement::Expression(expression) => {
                self.translate_expression(expression, &mut instructions);
            }
            parser::Statement::Null => {}
            parser::Statement::If {
                condition,
                then,
                otherwise,
            } => {
                let label_count = self.label_count;
                self.label_count += 1;

                let c = self.translate_expression(condition, &mut instructions);
                match otherwise {
                    Some(otherwise) => {
                        instructions.push(Instruction::JumpIfZero {
                            condition: c,
                            target: String::from(format!("if_else{}", label_count)),
                        });
                        instructions.extend(self.translate_statement(*then));

                        instructions.push(Instruction::Jump {
                            target: String::from(format!("if_end{}", label_count)),
                        });
                        instructions.push(Instruction::Label(String::from(format!(
                            "if_else{}",
                            label_count
                        ))));
                        instructions.extend(self.translate_statement(*otherwise));
                    }
                    None => {
                        instructions.push(Instruction::JumpIfZero {
                            condition: c,
                            target: String::from(format!("if_end{}", label_count)),
                        });
                        instructions.extend(self.translate_statement(*then));
                    }
                }
                instructions.push(Instruction::Label(String::from(format!(
                    "if_end{}",
                    label_count
                ))));
            }
            parser::Statement::Compound(block) => instructions.extend(
                block
                    .0
                    .into_iter()
                    .flat_map(|block_item| self.translate_block_item(block_item)),
            ),
        };
        instructions
    }

    fn translate_expression(
        &mut self,
        expression: parser::Expression,
        instructions: &mut Vec<Instruction>,
    ) -> Val {
        match expression {
            parser::Expression::Var(v) => Val::Var(v),
            parser::Expression::Assignment { lvalue, expression } => {
                let right = self.translate_expression(*expression, instructions);
                let left = self.translate_expression(*lvalue, instructions);
                instructions.push(Instruction::Copy {
                    src: right,
                    dst: left.clone(),
                });
                left
            }
            parser::Expression::Constant(int) => Val::Constant(int),
            parser::Expression::Unary { operator, operand } => {
                let dst = Val::Var(format!("tmp.{}", self.var_count).to_string());
                let src = {
                    self.var_count += 1;
                    self.translate_expression(*operand, instructions)
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
                    let dst = Val::Var(format!("tmp.{}", self.var_count).to_string());
                    self.var_count += 1;

                    let label_count = self.label_count;
                    self.label_count += 1;

                    let left = self.translate_expression(*left_expression, instructions);
                    instructions.push(Instruction::JumpIfZero {
                        condition: left,
                        target: String::from(format!("and_false{}", label_count)),
                    });

                    let right = self.translate_expression(*right_expression, instructions);
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
                    let dst = Val::Var(format!("tmp.{}", self.var_count).to_string());
                    self.var_count += 1;

                    let label_count = self.label_count;
                    self.label_count += 1;

                    let left = self.translate_expression(*left_expression, instructions);
                    instructions.push(Instruction::JumpIfNotZero {
                        condition: left,
                        target: String::from(format!("or_false{}", label_count)),
                    });

                    let right = self.translate_expression(*right_expression, instructions);
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
                    let dst = Val::Var(format!("tmp.{}", self.var_count).to_string());
                    self.var_count += 1;
                    let left = self.translate_expression(*left_expression, instructions);
                    let right = self.translate_expression(*right_expression, instructions);
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
                        parser::BinaryOp::And
                        | parser::BinaryOp::Or
                        | parser::BinaryOp::Assignment => {
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
                let dst = Val::Var(format!("tmp.{}", self.var_count).to_string());
                self.var_count += 1;

                let label_count = self.label_count;
                self.label_count += 1;

                let condition = self.translate_expression(*condition, instructions);

                instructions.push(Instruction::JumpIfZero {
                    condition,
                    target: String::from(format!("ternary_otherwise{}", label_count)),
                });

                let e1 = self.translate_expression(*then, instructions);
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

                let e2 = self.translate_expression(*otherwise, instructions);
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
}
