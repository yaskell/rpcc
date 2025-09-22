#[derive(Debug)]
pub struct Program {
    pub function_definition: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub body: Statement
}

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub struct Int(pub i32);

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(Int),
}
