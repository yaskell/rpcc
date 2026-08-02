use crate::parser;

type Identifier = String;

struct LoopLabeler {
    loop_count: i32,
}

impl LoopLabeler {
    fn new() -> Self {
        Self { loop_count: 0 }
    }

    fn next(&mut self) -> Identifier {
        let id = format!("loop.{}", self.loop_count);
        self.loop_count += 1;
        id
    }
}

pub fn label_loops(program: parser::Program) -> LabeledProgram {
    let mut labeler = LoopLabeler::new();
    LabeledProgram {
        function: label_function(program.function, None, &mut labeler),
    }
}

#[derive(Debug)]
pub struct LabeledProgram {
    pub function: LabeledFunction,
}

#[derive(Debug)]
pub struct LabeledFunction {
    pub name: Identifier,
    pub body: LabeledBlock,
}

#[derive(Debug)]
pub struct LabeledBlock(pub Vec<LabeledBlockItem>);

#[derive(Debug)]
pub enum LabeledBlockItem {
    S(LabeledStatement),
    D(parser::Declaration),
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

fn label_function(
    parser::Function { name, body }: parser::Function,
    current_label: Option<Identifier>,
    labeler: &mut LoopLabeler,
) -> LabeledFunction {
    LabeledFunction {
        name,
        body: label_block(body, current_label, labeler),
    }
}

fn label_block(
    block: parser::Block,
    current_label: Option<Identifier>,
    labeler: &mut LoopLabeler,
) -> LabeledBlock {
    LabeledBlock(
        block
            .0
            .into_iter()
            .map(|item| label_block_item(item, current_label.clone(), labeler))
            .collect(),
    )
}

fn label_block_item(
    item: parser::BlockItem,
    current_label: Option<Identifier>,
    labeler: &mut LoopLabeler,
) -> LabeledBlockItem {
    match item {
        parser::BlockItem::S(statement) => {
            LabeledBlockItem::S(label_statement(statement, current_label, labeler))
        }
        parser::BlockItem::D(d) => LabeledBlockItem::D(d),
    }
}

fn label_statement(
    statement: parser::Statement,
    current_label: Option<Identifier>,
    labeler: &mut LoopLabeler,
) -> LabeledStatement {
    match statement {
        parser::Statement::Compound(block) => {
            LabeledStatement::Compound(label_block(block, current_label, labeler))
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
            let label = labeler.next();
            LabeledStatement::While {
                condition,
                body: Box::new(label_statement(*body, Some(label.clone()), labeler)),
                label,
            }
        }

        parser::Statement::DoWhile { condition, body } => {
            let label = labeler.next();
            LabeledStatement::DoWhile {
                condition,
                body: Box::new(label_statement(*body, Some(label.clone()), labeler)),
                label,
            }
        }

        parser::Statement::For {
            init,
            condition,
            post,
            body,
        } => {
            let label = labeler.next();
            LabeledStatement::For {
                init,
                condition,
                post,
                body: Box::new(label_statement(*body, Some(label.clone()), labeler)),
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
            then: Box::new(label_statement(*then, current_label.clone(), labeler)),
            otherwise: otherwise.map(|s| Box::new(label_statement(*s, current_label, labeler))),
        },
    }
}
