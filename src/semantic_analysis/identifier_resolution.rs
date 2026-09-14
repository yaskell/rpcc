use std::collections::HashMap;

use crate::parser;

pub fn resolve_identifiers(program: parser::Program) -> parser::Program {
    let mut allocator = IdentifierAllocator::new();

    parser::Program {
        functions: program
            .functions
            .into_iter()
            .map(|f| resolve_function_declaration(f, &mut allocator))
            .collect(),
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierAllocator {
    pub identifier_count: i32,
    pub map: HashMap<String, IdentifierData>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Linkage {
    ExternalLinkage,
    NoLinkage,
}

#[derive(Debug, Clone)]
pub struct IdentifierData {
    pub name: String,
    pub from_current_scope: bool,
    pub linkage: Linkage,
}

impl IdentifierAllocator {
    fn new() -> Self {
        Self {
            identifier_count: 0,
            map: HashMap::<String, IdentifierData>::new(),
        }
    }

    fn new_name(&self, var: &str) -> String {
        format!("{}.{}", var, self.identifier_count)
    }

    fn allocate_identifier(&mut self, identifier: String, linkage: Linkage) -> String {
        match self.map.get(&identifier) {
            Some(v)
                if v.from_current_scope
                    && (v.linkage != Linkage::ExternalLinkage
                        || linkage != Linkage::ExternalLinkage) =>
            {
                panic!("Duplicate declaration: {identifier}")
            }
            _not_in_map => {
                let identifier_name = match linkage {
                    Linkage::ExternalLinkage => identifier.clone(),
                    Linkage::NoLinkage => self.new_name(&identifier.clone()),
                };

                let identifier_data = IdentifierData {
                    name: identifier_name.clone(),
                    from_current_scope: true,
                    linkage,
                };

                self.map.insert(identifier, identifier_data);
                self.identifier_count += 1;

                identifier_name
            }
        }
    }

    fn new_scope(&mut self) -> Self {
        let mut new_allocator = self.clone();
        for v in new_allocator.map.values_mut() {
            v.from_current_scope = false;
        }
        new_allocator
    }
}

pub fn resolve_block(block: parser::Block, allocator: &mut IdentifierAllocator) -> parser::Block {
    parser::Block(
        block
            .0
            .into_iter()
            .map(|i| resolve_block_item(i, allocator))
            .collect(),
    )
}

pub fn resolve_block_item(
    item: parser::BlockItem,
    allocator: &mut IdentifierAllocator,
) -> parser::BlockItem {
    match item {
        parser::BlockItem::S(statement) => {
            parser::BlockItem::S(resolve_statement(statement, allocator))
        }
        parser::BlockItem::D(declaration) => {
            parser::BlockItem::D(resolve_declaration(declaration, allocator))
        }
    }
}

pub fn resolve_declaration(
    declaration: parser::Declaration,
    allocator: &mut IdentifierAllocator,
) -> parser::Declaration {
    match declaration {
        parser::Declaration::FunDecl(fd) => {
            parser::Declaration::FunDecl(resolve_local_function_declaration(fd, allocator))
        }
        parser::Declaration::VarDecl(vd) => {
            parser::Declaration::VarDecl(resolve_variable_declaration(vd, allocator))
        }
    }
}

pub fn resolve_local_function_declaration(
    function_declaration: parser::FunctionDeclaration,
    allocator: &mut IdentifierAllocator,
) -> parser::FunctionDeclaration {
    if function_declaration.body.is_some() {
        panic!(
            "Cannot redefine function in inner scope: {:?}",
            function_declaration.name
        );
    }
    resolve_function_declaration(function_declaration, allocator)
}

pub fn resolve_function_declaration(
    parser::FunctionDeclaration { name, params, body }: parser::FunctionDeclaration,
    allocator: &mut IdentifierAllocator,
) -> parser::FunctionDeclaration {
    let resolved_name = allocator.allocate_identifier(name, Linkage::ExternalLinkage);

    let mut new_allocator = allocator.new_scope();

    let resolved_params = params
        .into_iter()
        .map(|p| new_allocator.allocate_identifier(p, Linkage::NoLinkage))
        .collect();

    let resolved_body = body.map(|b| resolve_block(b, &mut new_allocator));

    parser::FunctionDeclaration {
        name: resolved_name,
        params: resolved_params,
        body: resolved_body,
    }
}

pub fn resolve_variable_declaration(
    parser::VariableDeclaration { name, init }: parser::VariableDeclaration,
    allocator: &mut IdentifierAllocator,
) -> parser::VariableDeclaration {
    let resolved_var = allocator.allocate_identifier(name, Linkage::NoLinkage);
    let resolved_init = init.map(|e| resolve_exp(e, allocator));

    parser::VariableDeclaration {
        name: resolved_var,
        init: resolved_init,
    }
}

pub fn resolve_exp(
    exp: parser::Expression,
    allocator: &mut IdentifierAllocator,
) -> parser::Expression {
    match exp {
        parser::Expression::Assignment { lvalue, expression } => {
            if let parser::Expression::Var(_) = *lvalue {
                return parser::Expression::Assignment {
                    lvalue: Box::new(resolve_exp(*lvalue, allocator)),
                    expression: Box::new(resolve_exp(*expression, allocator)),
                };
            }
            panic!("Invalid lvalue: `{lvalue:?}`")
        }
        parser::Expression::Var(v) => match allocator.map.get(&v) {
            Some(IdentifierData { name: new_name, .. }) => {
                parser::Expression::Var(new_name.clone())
            }
            None => panic!("Undeclared variable: `{v}`"),
        },
        parser::Expression::Unary { operator, operand } => parser::Expression::Unary {
            operator,
            operand: Box::new(resolve_exp(*operand, allocator)),
        },
        parser::Expression::Binary {
            operator,
            left_expression,
            right_expression,
        } => parser::Expression::Binary {
            operator,
            left_expression: Box::new(resolve_exp(*left_expression, allocator)),
            right_expression: Box::new(resolve_exp(*right_expression, allocator)),
        },
        parser::Expression::Conditional {
            condition,
            then,
            otherwise,
        } => parser::Expression::Conditional {
            condition: Box::new(resolve_exp(*condition, allocator)),
            then: Box::new(resolve_exp(*then, allocator)),
            otherwise: Box::new(resolve_exp(*otherwise, allocator)),
        },
        parser::Expression::FunctionCall { identifier, args } => {
            let new_name = match allocator.map.get(&identifier) {
                Some(IdentifierData { name: new_name, .. }) => new_name.clone(),
                None => panic!("Undeclared function: `{identifier}`"),
            };

            let new_args = args
                .into_iter()
                .map(|a| resolve_exp(a, allocator))
                .collect();

            parser::Expression::FunctionCall {
                identifier: new_name,
                args: new_args,
            }
        }
        exp_without_subexp @ parser::Expression::Constant(_) => exp_without_subexp,
    }
}

pub fn resolve_statement(
    statement: parser::Statement,
    allocator: &mut IdentifierAllocator,
) -> parser::Statement {
    match statement {
        parser::Statement::Return(exp) => parser::Statement::Return(resolve_exp(exp, allocator)),
        parser::Statement::Expression(exp) => {
            parser::Statement::Expression(resolve_exp(exp, allocator))
        }
        parser::Statement::If {
            condition,
            then,
            otherwise,
        } => parser::Statement::If {
            condition: resolve_exp(condition, allocator),
            then: Box::new(resolve_statement(*then, allocator)),
            otherwise: otherwise
                .map(|statement| Box::new(resolve_statement(*statement, allocator))),
        },
        parser::Statement::Compound(block) => {
            let mut new_scope_allocator = allocator.new_scope();
            parser::Statement::Compound(resolve_block(block, &mut new_scope_allocator))
        }
        parser::Statement::While { condition, body } => parser::Statement::While {
            condition: resolve_exp(condition, allocator),
            body: Box::new(resolve_statement(*body, allocator)),
        },
        parser::Statement::DoWhile { condition, body } => parser::Statement::DoWhile {
            condition: resolve_exp(condition, allocator),
            body: Box::new(resolve_statement(*body, allocator)),
        },
        parser::Statement::For {
            init,
            condition,
            post,
            body,
        } => {
            let mut new_scope_allocator = allocator.new_scope();
            parser::Statement::For {
                init: resolve_for_init(init, &mut new_scope_allocator),
                condition: resolve_optional_exp(condition, &mut new_scope_allocator),
                post: resolve_optional_exp(post, &mut new_scope_allocator),
                body: Box::new(resolve_statement(*body, &mut new_scope_allocator)),
            }
        }
        statement_without_substatement_or_subexpression @ (parser::Statement::Null
        | parser::Statement::Break
        | parser::Statement::Continue) => statement_without_substatement_or_subexpression,
    }
}

fn resolve_for_init(init: parser::ForInit, allocator: &mut IdentifierAllocator) -> parser::ForInit {
    match init {
        parser::ForInit::D(dec) => parser::ForInit::D(resolve_variable_declaration(dec, allocator)),
        parser::ForInit::E(exp) => parser::ForInit::E(resolve_optional_exp(exp, allocator)),
    }
}

fn resolve_optional_exp(
    exp: Option<parser::Expression>,
    allocator: &mut IdentifierAllocator,
) -> Option<parser::Expression> {
    exp.map(|e| resolve_exp(e, allocator))
}
