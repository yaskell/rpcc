use core::panic;
use std::collections::HashMap;

use crate::parser;

pub struct VarAllocator {
    pub var_count: i32,
    pub map: HashMap<String, String>,
}

impl VarAllocator {
    fn new() -> VarAllocator {
        VarAllocator {
            var_count: 0,
            map: HashMap::<String, String>::new(),
        }
    }

    fn new_unique_name(&self, var: &str) -> String {
        format!("{}.{}", var, self.var_count)
    }

    fn new_var(&mut self, var: String) -> String {
        match self.map.get(&var) {
            Some(_) => panic!("Duplicate variable declaration: {var}"),
            None => {
                let name = VarAllocator::new_unique_name(self, var.as_str());
                self.map.insert(var.clone(), name.clone());
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
        body: body
            .into_iter()
            .map(|i| resolve_block_item(i, va))
            .collect(),
    }
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
            Some(var) => return parser::Expression::Var(var.clone()),
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
        other_without_subexpressions => other_without_subexpressions,
    }
}

pub fn resolve_statement(statement: parser::Statement, va: &mut VarAllocator) -> parser::Statement {
    match statement {
        parser::Statement::Return(exp) => parser::Statement::Return(resolve_exp(exp, va)),
        parser::Statement::Expression(exp) => parser::Statement::Expression(resolve_exp(exp, va)),
        parser::Statement::Null => parser::Statement::Null,
    }
}
