type Identifier = String;
type Int = i32;
type Src = Val;
type Dst = Identifier; //Dst must always be a temporary Var

struct TackyProgram {
    tacky_function_definition: TackyFunctionDefinition,
}

struct TackyFunctionDefinition {
    identifier: Identifier,
    body: Vec<Instruction>,
}

enum Instruction {
    Return(Val),
    Unary(UnaryOperator, Src, Dst),
}

enum Val {
    Constant(Int),
    Var(Identifier),
}

enum UnaryOperator {
    Complement,
    Negate,
}
