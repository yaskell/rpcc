use crate::parser;

type Identifier = String;

struct LoopLabelGenerator {
    next_loop_id: i32,
}

impl LoopLabelGenerator {
    fn new() -> Self {
        Self { next_loop_id: 0 }
    }

    fn new_label(&mut self) -> Identifier {
        let id = format!("loop.{}", self.next_loop_id);
        self.next_loop_id += 1;
        id
    }
}

pub fn label_loops(program: parser::Program) -> LabeledProgram {
    let mut labeler = LoopLabelGenerator::new();
    LabeledProgram {
        functions: program
            .functions
            .into_iter()
            .map(|f| LabeledFunctionDeclaration::from(f, None, &mut labeler))
            .collect(),
    }
}

#[derive(Debug)]
pub struct LabeledProgram {
    pub functions: Vec<LabeledFunctionDeclaration>,
}

#[derive(Debug)]
pub enum LabeledDeclaration {
    LabeledFuncDecl(LabeledFunctionDeclaration),
    LabeledVarDecl(parser::VariableDeclaration),
}

impl LabeledDeclaration {
    fn from(
        declaration: parser::Declaration,
        current_label: Option<Identifier>,
        label_generator: &mut LoopLabelGenerator,
    ) -> Self {
        match declaration {
            parser::Declaration::FunDecl(parser::FunctionDeclaration { name, params, body }) => {
                LabeledDeclaration::LabeledFuncDecl(LabeledFunctionDeclaration {
                    name,
                    params,
                    body: body.map(|b| LabeledBlock::from(b, current_label, label_generator)),
                })
            }
            parser::Declaration::VarDecl(vd) => LabeledDeclaration::LabeledVarDecl(vd),
        }
    }
}

#[derive(Debug)]
pub struct LabeledFunctionDeclaration {
    pub name: Identifier,
    pub params: Vec<Identifier>,
    pub body: Option<LabeledBlock>,
}

impl LabeledFunctionDeclaration {
    fn from(
        parser::FunctionDeclaration { name, params, body }: parser::FunctionDeclaration,
        current_label: Option<Identifier>,
        label_generator: &mut LoopLabelGenerator,
    ) -> LabeledFunctionDeclaration {
        LabeledFunctionDeclaration {
            name,
            params,
            body: body.map(|b| LabeledBlock::from(b, current_label, label_generator)),
        }
    }
}

#[derive(Debug)]
pub struct LabeledBlock(pub Vec<LabeledBlockItem>);

impl LabeledBlock {
    fn from(
        block: parser::Block,
        current_label: Option<Identifier>,
        labeler: &mut LoopLabelGenerator,
    ) -> LabeledBlock {
        LabeledBlock(
            block
                .0
                .into_iter()
                .map(|item| LabeledBlockItem::from(item, current_label.clone(), labeler))
                .collect(),
        )
    }
}

#[derive(Debug)]
pub enum LabeledBlockItem {
    S(LabeledStatement),
    D(LabeledDeclaration),
}

impl LabeledBlockItem {
    fn from(
        item: parser::BlockItem,
        current_label: Option<Identifier>,
        labeler: &mut LoopLabelGenerator,
    ) -> LabeledBlockItem {
        match item {
            parser::BlockItem::S(statement) => {
                LabeledBlockItem::S(LabeledStatement::from(statement, current_label, labeler))
            }
            parser::BlockItem::D(d) => {
                LabeledBlockItem::D(LabeledDeclaration::from(d, current_label, labeler))
            }
        }
    }
}

#[derive(Debug)]
pub enum LabeledStatement {
    Return(parser::Expression),
    Expression(parser::Expression),
    If {
        condition: parser::Expression,
        then: Box<LabeledStatement>,
        otherwise: Option<Box<LabeledStatement>>,
    },
    Compound(LabeledBlock),
    Break(Identifier),
    Continue(Identifier),
    While {
        condition: parser::Expression,
        body: Box<LabeledStatement>,
        label: Identifier,
    },
    DoWhile {
        condition: parser::Expression,
        body: Box<LabeledStatement>,
        label: Identifier,
    },
    For {
        init: parser::ForInit,
        condition: Option<parser::Expression>,
        post: Option<parser::Expression>,
        body: Box<LabeledStatement>,
        label: Identifier,
    },
    Null,
}

impl LabeledStatement {
    fn from(
        statement: parser::Statement,
        current_label: Option<Identifier>,
        labeler: &mut LoopLabelGenerator,
    ) -> LabeledStatement {
        match statement {
            parser::Statement::Compound(block) => {
                LabeledStatement::Compound(LabeledBlock::from(block, current_label, labeler))
            }

            parser::Statement::Break => match current_label {
                Some(label) => LabeledStatement::Break(label),
                None => panic!("Break statement outside of loop"),
            },

            parser::Statement::Continue => match current_label {
                Some(label) => LabeledStatement::Continue(label),
                None => panic!("Continue statement outside of loop"),
            },

            parser::Statement::While { condition, body } => {
                let label = labeler.new_label();
                LabeledStatement::While {
                    condition,
                    body: Box::new(LabeledStatement::from(*body, Some(label.clone()), labeler)),
                    label,
                }
            }

            parser::Statement::DoWhile { condition, body } => {
                let label = labeler.new_label();
                LabeledStatement::DoWhile {
                    condition,
                    body: Box::new(LabeledStatement::from(*body, Some(label.clone()), labeler)),
                    label,
                }
            }

            parser::Statement::For {
                init,
                condition,
                post,
                body,
            } => {
                let label = labeler.new_label();
                LabeledStatement::For {
                    init,
                    condition,
                    post,
                    body: Box::new(LabeledStatement::from(*body, Some(label.clone()), labeler)),
                    label,
                }
            }

            parser::Statement::Null => LabeledStatement::Null,
            parser::Statement::Return(expression) => LabeledStatement::Return(expression),
            parser::Statement::Expression(expression) => LabeledStatement::Expression(expression),

            parser::Statement::If {
                condition,
                then,
                otherwise,
            } => LabeledStatement::If {
                condition,
                then: Box::new(LabeledStatement::from(
                    *then,
                    current_label.clone(),
                    labeler,
                )),
                otherwise: otherwise
                    .map(|s| Box::new(LabeledStatement::from(*s, current_label, labeler))),
            },
        }
    }
}
