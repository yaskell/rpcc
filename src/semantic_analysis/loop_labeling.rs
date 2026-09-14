use crate::parser;

type Identifier = String;

struct LoopLabelGenerator {
    next_loop_id: i32,
}

impl LoopLabelGenerator {
    const fn new() -> Self {
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
                Self::LabeledFuncDecl(LabeledFunctionDeclaration {
                    name,
                    params,
                    body: body.map(|b| LabeledBlock::from(b, current_label, label_generator)),
                })
            }
            parser::Declaration::VarDecl(vd) => Self::LabeledVarDecl(vd),
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
    ) -> Self {
        Self {
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
    ) -> Self {
        Self(
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
    ) -> Self {
        match item {
            parser::BlockItem::S(statement) => {
                Self::S(LabeledStatement::from(statement, current_label, labeler))
            }
            parser::BlockItem::D(d) => {
                Self::D(LabeledDeclaration::from(d, current_label, labeler))
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
        then: Box<Self>,
        otherwise: Option<Box<Self>>,
    },
    Compound(LabeledBlock),
    Break(Identifier),
    Continue(Identifier),
    While {
        condition: parser::Expression,
        body: Box<Self>,
        label: Identifier,
    },
    DoWhile {
        condition: parser::Expression,
        body: Box<Self>,
        label: Identifier,
    },
    For {
        init: parser::ForInit,
        condition: Option<parser::Expression>,
        post: Option<parser::Expression>,
        body: Box<Self>,
        label: Identifier,
    },
    Null,
}

impl LabeledStatement {
    fn from(
        statement: parser::Statement,
        current_label: Option<Identifier>,
        labeler: &mut LoopLabelGenerator,
    ) -> Self {
        match statement {
            parser::Statement::Compound(block) => {
                Self::Compound(LabeledBlock::from(block, current_label, labeler))
            }

            parser::Statement::Break => match current_label {
                Some(label) => Self::Break(label),
                None => panic!("Break statement outside of loop"),
            },

            parser::Statement::Continue => match current_label {
                Some(label) => Self::Continue(label),
                None => panic!("Continue statement outside of loop"),
            },

            parser::Statement::While { condition, body } => {
                let label = labeler.new_label();
                Self::While {
                    condition,
                    body: Box::new(Self::from(*body, Some(label.clone()), labeler)),
                    label,
                }
            }

            parser::Statement::DoWhile { condition, body } => {
                let label = labeler.new_label();
                Self::DoWhile {
                    condition,
                    body: Box::new(Self::from(*body, Some(label.clone()), labeler)),
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
                Self::For {
                    init,
                    condition,
                    post,
                    body: Box::new(Self::from(*body, Some(label.clone()), labeler)),
                    label,
                }
            }

            parser::Statement::Null => Self::Null,
            parser::Statement::Return(expression) => Self::Return(expression),
            parser::Statement::Expression(expression) => Self::Expression(expression),

            parser::Statement::If {
                condition,
                then,
                otherwise,
            } => Self::If {
                condition,
                then: Box::new(Self::from(
                    *then,
                    current_label.clone(),
                    labeler,
                )),
                otherwise: otherwise
                    .map(|s| Box::new(Self::from(*s, current_label, labeler))),
            },
        }
    }
}
