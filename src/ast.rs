pub struct AST {
    pub program: FunctionDefinition
}

pub struct FunctionDefinition {
    pub name: String,
    pub body: Statement,
}

pub enum Expression {
    Constant(i32),
}

pub enum Statement {
    Return(Expression),
}
