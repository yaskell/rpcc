use std::collections::HashMap;

use crate::parser::{self, Declaration, Expression};

pub fn resolve_variables(program: parser::Program) -> parser::Program {
    let mut variables = HashMap::<String, String>::new();
    todo!()
}

pub fn resolve_declaration(
    parser::Declaration { mut name, mut init }: parser::Declaration,
    map: &mut HashMap<String, String>,
) -> Declaration {
    let unique_name = make_temporary();
    match map.get(&name) {
        Some(v) => panic!("Duplicate variable declaration: {v}"),
        None => map.insert(name, unique_name.clone()),
    };
    if let Some(_) = init {
        init = resolve_exp(init, map)
    }
    Declaration {
        name: unique_name,
        init,
    }
}

pub fn make_temporary() -> String {
    todo!()
}

pub fn resolve_exp(
    init: Option<Expression>,
    map: &mut HashMap<String, String>,
) -> Option<Expression> {
    todo!()
}

// Resolve_declaration(Declaration(name, init), variable_map):
// 1 if name is in variable_map:
// fail("Duplicate variable declaration!")
// unique_name = make_temporary()
// 2 variable_map.add(name, unique_name)
// 3 if init is not null:
// init = resolve_exp(init, variable_map)
// 4 return Declaration(unique_name, init)
