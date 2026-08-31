use std::collections::HashMap;

use crate::parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    FunType { param_count: u32 },
}

#[derive(Debug)]
pub struct Symbol {
    kind: Type,
    defined: bool,
}

pub fn check_types(program: &parser::Program) -> Symbols {
    let mut symbols: Symbols = HashMap::new();
    program
        .functions
        .iter()
        .for_each(|f| typecheck_function_declaration(f, &mut symbols));

    symbols
}

type Symbols = HashMap<String, Symbol>;

pub fn typecheck_function_declaration(fd: &parser::FunctionDeclaration, symbols: &mut Symbols) {
    let fun_type = Type::FunType {
        param_count: fd.params.len() as u32,
    };
    let has_body = fd.body.is_some();
    let mut already_defined = false;

    if let Some(previous_declaration) = symbols.get(&fd.name) {
        if previous_declaration.kind != fun_type {
            panic!(
                "Incompatible function declaration: {:?} and {:?}",
                previous_declaration, fun_type
            );
        }
        already_defined = previous_declaration.defined;
        if already_defined && has_body {
            panic!("Function '{:?}' is defined more than once", fd.name);
        }
    }

    symbols.insert(
        fd.name.clone(),
        Symbol {
            kind: fun_type,
            defined: (already_defined || has_body),
        },
    );

    if has_body {
        fd.params.iter().for_each(|p| {
            symbols.insert(
                p.clone(),
                Symbol {
                    kind: Type::Int,
                    defined: false,
                },
            );
        });
        fd.body.as_ref().map(|b| typecheck_block(&b, symbols));
    };
}

pub fn typecheck_block(block: &parser::Block, symbols: &mut Symbols) {
    block
        .0
        .iter()
        .for_each(|block_item| typecheck_block_item(block_item, symbols));
}

pub fn typecheck_block_item(block_item: &parser::BlockItem, symbols: &mut Symbols) {
    match block_item {
        parser::BlockItem::S(statement) => typecheck_statement(statement, symbols),
        parser::BlockItem::D(declaration) => typecheck_declaration(declaration, symbols),
    }
}

pub fn typecheck_statement(statement: &parser::Statement, symbols: &mut Symbols) {
    println!("{:?}", statement);
    match statement {
        parser::Statement::Compound(block) => typecheck_block(block, symbols),
        parser::Statement::Expression(exp) => typecheck_exp(exp, symbols),
        parser::Statement::Return(exp) => typecheck_exp(exp, symbols),
        parser::Statement::If {
            condition,
            then,
            otherwise,
        } => {
            typecheck_exp(condition, symbols);
            typecheck_statement(then, symbols);
            otherwise
                .as_ref()
                .map(|s| typecheck_statement(&*s, symbols));
        }
        parser::Statement::While { condition, body } => {
            typecheck_exp(condition, symbols);
            typecheck_statement(body, symbols);
        }
        parser::Statement::DoWhile { condition, body } => {
            typecheck_exp(condition, symbols);
            typecheck_statement(body, symbols);
        }
        parser::Statement::For {
            init,
            condition,
            post,
            body,
        } => {
            typecheck_for_init(init, symbols);
            condition.as_ref().map(|e| typecheck_exp(e, symbols));
            post.as_ref().map(|e| typecheck_exp(e, symbols));
            typecheck_statement(body, symbols);
        }
        _statement_without_identifiers @ (parser::Statement::Break
        | parser::Statement::Continue
        | parser::Statement::Null) => {}
    }
}

pub fn typecheck_for_init(for_init: &parser::ForInit, symbols: &mut Symbols) {
    match for_init {
        parser::ForInit::D(vd) => typecheck_variable_declaration(vd, symbols),
        parser::ForInit::E(exp) => {
            exp.as_ref().map(|e| typecheck_exp(e, symbols));
        }
    }
}

pub fn typecheck_declaration(declaration: &parser::Declaration, symbols: &mut Symbols) {
    match declaration {
        parser::Declaration::FunDecl(fd) => typecheck_function_declaration(fd, symbols),
        parser::Declaration::VarDecl(vd) => typecheck_variable_declaration(vd, symbols),
    }
}

pub fn typecheck_variable_declaration(vd: &parser::VariableDeclaration, symbols: &mut Symbols) {
    symbols.insert(
        vd.name.clone(),
        Symbol {
            kind: Type::Int,
            defined: false,
        },
    );
    vd.init.iter().for_each(|e| typecheck_exp(e, symbols));
}

pub fn typecheck_exp(exp: &parser::Expression, symbols: &mut Symbols) {
    match exp {
        parser::Expression::FunctionCall { identifier, args } => {
            match symbols.get(identifier).unwrap().kind {
                Type::Int => panic!("Variable '{:?}' used as function name", identifier),
                Type::FunType { param_count } => {
                    if param_count != args.len() as u32 {
                        panic!(
                            "Function '{:?}' called with wrong number of arguments",
                            identifier
                        );
                    }
                    args.into_iter().for_each(|a| typecheck_exp(a, symbols));
                }
            }
        }
        parser::Expression::Var(identifier) => {
            if symbols.get(identifier).unwrap().kind != Type::Int {
                panic!("Function name '{:?}' used as variable", identifier)
            }
        }
        parser::Expression::Unary { operand, .. } => {
            typecheck_exp(operand, symbols);
        }
        parser::Expression::Binary {
            left_expression,
            right_expression,
            ..
        } => {
            typecheck_exp(left_expression, symbols);
            typecheck_exp(right_expression, symbols);
        }
        parser::Expression::Assignment { lvalue, expression } => {
            typecheck_exp(lvalue, symbols);
            typecheck_exp(expression, symbols);
        }
        parser::Expression::Conditional {
            condition,
            then,
            otherwise,
        } => {
            typecheck_exp(condition, symbols);
            typecheck_exp(then, symbols);
            typecheck_exp(otherwise, symbols);
        }
        _expression_without_identifier @ (parser::Expression::Constant(_)) => {}
    }
}
