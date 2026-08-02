use std::collections::HashMap;

use crate::parser::{self, ForInit};

#[derive(Debug, Clone)]
pub struct VarAllocator {
    pub var_count: i32,
    pub map: HashMap<String, MapEntry>,
}

#[derive(Debug, Clone)]
pub struct MapEntry {
    pub new_name: String,
    pub from_current_block: bool,
}

impl VarAllocator {
    fn new() -> VarAllocator {
        VarAllocator {
            var_count: 0,
            map: HashMap::<String, MapEntry>::new(),
        }
    }

    fn new_unique_name(&self, var: &str) -> String {
        format!("{}.{}", var, self.var_count)
    }

    fn new_var(&mut self, var: String) -> String {
        match self.map.get(&var) {
            Some(v) if v.from_current_block => {
                panic!("Duplicate variable declaration: {var}")
            }
            _ => {
                let name = VarAllocator::new_unique_name(self, var.as_str());
                self.map.insert(
                    var.clone(),
                    MapEntry {
                        new_name: name.clone(),
                        from_current_block: true,
                    },
                );
                self.var_count += 1;
                name
            }
        }
    }
}

pub fn resolve_variables(program: parser::Program) -> parser::Program {
    let mut va = VarAllocator::new();
    parser::Program {
        function: resolve_function(program.function, &mut va),
    }
}

pub fn resolve_function(
    parser::Function { name, body }: parser::Function,
    va: &mut VarAllocator,
) -> parser::Function {
    parser::Function {
        name,
        body: resolve_block(body, va),
    }
}

pub fn resolve_block(block: parser::Block, va: &mut VarAllocator) -> parser::Block {
    parser::Block(
        block
            .0
            .into_iter()
            .map(|i| resolve_block_item(i, va))
            .collect(),
    )
}

pub fn resolve_block_item(item: parser::BlockItem, va: &mut VarAllocator) -> parser::BlockItem {
    match item {
        parser::BlockItem::S(statement) => parser::BlockItem::S(resolve_statement(statement, va)),
        parser::BlockItem::D(declaration) => {
            parser::BlockItem::D(resolve_declaration(declaration, va))
        }
    }
}

pub fn resolve_declaration(
    parser::Declaration { name, mut init }: parser::Declaration,
    va: &mut VarAllocator,
) -> parser::Declaration {
    let new_var = va.new_var(name);
    if let Some(exp) = init {
        init = Some(resolve_exp(exp, va))
    }
    parser::Declaration {
        name: new_var,
        init,
    }
}

pub fn resolve_exp(exp: parser::Expression, va: &mut VarAllocator) -> parser::Expression {
    match exp {
        parser::Expression::Assignment { lvalue, expression } => {
            if let parser::Expression::Var(_) = *lvalue {
                return parser::Expression::Assignment {
                    lvalue: Box::new(resolve_exp(*lvalue, va)),
                    expression: Box::new(resolve_exp(*expression, va)),
                };
            }
            panic!("Invalid lvalue: `{:?}`", lvalue)
        }
        parser::Expression::Var(v) => match va.map.get(&v) {
            Some(MapEntry {
                new_name,
                from_current_block: _,
            }) => parser::Expression::Var(new_name.clone()),
            None => panic!("Undeclared variable: `{}`", v),
        },
        parser::Expression::Unary { operator, operand } => parser::Expression::Unary {
            operator,
            operand: Box::new(resolve_exp(*operand, va)),
        },
        parser::Expression::Binary {
            operator,
            left_expression,
            right_expression,
        } => parser::Expression::Binary {
            operator,
            left_expression: Box::new(resolve_exp(*left_expression, va)),
            right_expression: Box::new(resolve_exp(*right_expression, va)),
        },
        parser::Expression::Conditional {
            condition,
            then,
            otherwise,
        } => parser::Expression::Conditional {
            condition: Box::new(resolve_exp(*condition, va)),
            then: Box::new(resolve_exp(*then, va)),
            otherwise: Box::new(resolve_exp(*otherwise, va)),
        },
        exp_without_subexp => exp_without_subexp,
    }
}

pub fn resolve_statement(statement: parser::Statement, va: &mut VarAllocator) -> parser::Statement {
    match statement {
        parser::Statement::Return(exp) => parser::Statement::Return(resolve_exp(exp, va)),
        parser::Statement::Expression(exp) => parser::Statement::Expression(resolve_exp(exp, va)),
        parser::Statement::If {
            condition,
            then,
            otherwise,
        } => parser::Statement::If {
            condition: resolve_exp(condition, va),
            then: Box::new(resolve_statement(*then, va)),
            otherwise: otherwise.map(|statement| Box::new(resolve_statement(*statement, va))),
        },
        parser::Statement::Compound(block) => {
            let mut var_allocator = va.clone();
            var_allocator.map = copy_variable_map(var_allocator.map);
            parser::Statement::Compound(resolve_block(block, &mut var_allocator))
        }
        parser::Statement::While { condition, body } => parser::Statement::While {
            condition: resolve_exp(condition, va),
            body: Box::new(resolve_statement(*body, va)),
        },
        parser::Statement::DoWhile { condition, body } => parser::Statement::DoWhile {
            condition: resolve_exp(condition, va),
            body: Box::new(resolve_statement(*body, va)),
        },
        parser::Statement::For {
            init,
            condition,
            post,
            body,
        } => {
            let mut new_va = va.clone();
            new_va.map = copy_variable_map(new_va.map);
            parser::Statement::For {
                init: resolve_for_init(init, &mut new_va),
                condition: resolve_optional_exp(condition, &mut new_va),
                post: resolve_optional_exp(post, &mut new_va),
                body: Box::new(resolve_statement(*body, &mut new_va)),
            }
        }
        statement_without_substatement_or_subexpression @ (parser::Statement::Null
        | parser::Statement::Break
        | parser::Statement::Continue) => statement_without_substatement_or_subexpression,
    }
}

fn resolve_for_init(init: ForInit, va: &mut VarAllocator) -> ForInit {
    match init {
        ForInit::D(dec) => ForInit::D(resolve_declaration(dec, va)),
        ForInit::E(exp) => ForInit::E(resolve_optional_exp(exp, va)),
    }
}

fn resolve_optional_exp(
    exp: Option<parser::Expression>,
    va: &mut VarAllocator,
) -> Option<parser::Expression> {
    exp.map(|e| resolve_exp(e, va))
}

pub fn copy_variable_map(map: HashMap<String, MapEntry>) -> HashMap<String, MapEntry> {
    let mut new_map = map.clone();
    for v in new_map.values_mut() {
        v.from_current_block = false
    }
    new_map
}
