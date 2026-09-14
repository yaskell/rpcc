use crate::lexer::Token;

pub fn parse(tokens: &mut Vec<Token>) -> Program {
    let mut functions: Vec<FunctionDeclaration> = vec![];
    while !tokens.is_empty() {
        functions.push(FunctionDeclaration::parse(tokens));
    }

    let result = Program { functions };
    if !tokens.is_empty() {
        panic!("Error: unexpected token '{:?} at end of program'", tokens)
    }
    result
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionDeclaration>,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub params: Vec<Identifier>,
    pub body: Option<Block>,
}

impl FunctionDeclaration {
    fn parse(tokens: &mut Vec<Token>) -> Self {
        consume(Token::Int, tokens);
        let identifier = parse_identifier(tokens);
        consume(Token::OpenParan, tokens);
        let params = Self::parse_parameters(tokens);
        consume(Token::CloseParan, tokens);
        let block = if tokens.first() == Some(&Token::Semicolon) {
            consume(Token::Semicolon, tokens);
            None
        } else {
            Some(Block::parse(tokens))
        };

        Self {
            name: identifier,
            params,
            body: block,
        }
    }

    fn parse_parameters(tokens: &mut Vec<Token>) -> Vec<Identifier> {
        if tokens.first() == Some(&Token::Void) {
            consume(Token::Void, tokens);
            return vec![];
        }
        let mut parameters: Vec<Identifier> = vec![];
        consume(Token::Int, tokens);
        parameters.push(parse_identifier(tokens));
        while tokens.first() == Some(&Token::Comma) {
            consume(Token::Comma, tokens);
            consume(Token::Int, tokens);
            parameters.push(parse_identifier(tokens));
        }
        parameters
    }
}

#[derive(Debug)]
pub struct Block(pub Vec<BlockItem>);

impl Block {
    fn parse(tokens: &mut Vec<Token>) -> Self {
        consume(Token::OpenBrace, tokens);
        let mut block_items = Vec::new();
        while tokens.first() != Some(&Token::CloseBrace) {
            block_items.push(BlockItem::parse(tokens));
        }
        consume(Token::CloseBrace, tokens);

        Self(block_items)
    }
}

#[derive(Debug)]
pub enum BlockItem {
    S(Statement),
    D(Declaration),
}

impl BlockItem {
    fn parse(tokens: &mut Vec<Token>) -> Self {
        match tokens.first().unwrap() {
            Token::Int => Self::D(Declaration::parse(tokens)),
            _ => Self::S(Statement::parse(tokens)),
        }
    }
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub init: Option<Expression>,
}

impl VariableDeclaration {
    fn parse(tokens: &mut Vec<Token>) -> Self {
        match &tokens[2] {
            Token::Equal => Self::parse_with_expression(tokens),
            Token::Semicolon => Self::parse_without_expression(tokens),
            other => panic!("Malformed variable declaration: {other:?}"),
        }
    }

    fn parse_with_expression(tokens: &mut Vec<Token>) -> Self {
        consume(Token::Int, tokens);
        let identifier = parse_identifier(tokens);
        consume(Token::Equal, tokens);
        let expression = Expression::parse(tokens, 0);
        consume(Token::Semicolon, tokens);
        Self {
            name: identifier,
            init: Some(expression),
        }
    }

    fn parse_without_expression(tokens: &mut Vec<Token>) -> Self {
        consume(Token::Int, tokens);
        let identifier = parse_identifier(tokens);
        consume(Token::Semicolon, tokens);
        Self {
            name: identifier,
            init: None,
        }
    }
}

#[derive(Debug)]
pub enum Declaration {
    FunDecl(FunctionDeclaration),
    VarDecl(VariableDeclaration),
}

impl Declaration {
    fn parse(tokens: &mut Vec<Token>) -> Self {
        match &tokens[2] {
            Token::Equal => {
                Self::VarDecl(VariableDeclaration::parse_with_expression(tokens))
            }
            Token::Semicolon => {
                Self::VarDecl(VariableDeclaration::parse_without_expression(tokens))
            }
            Token::OpenParan => Self::FunDecl(FunctionDeclaration::parse(tokens)),
            other => panic!("Malformed declaration: {other:?}"),
        }
    }
}

pub type Identifier = String;

fn parse_identifier(tokens: &mut Vec<Token>) -> Identifier {
    match tokens.remove(0) {
        Token::Identifier(x) => x,
        t => panic!("Syntax error: expected <Identifier>, found: '{t:?}'"),
    }
}

pub type Int = i32;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    If {
        condition: Expression,
        then: Box<Self>,
        otherwise: Option<Box<Self>>,
    },
    Compound(Block),
    Break,
    Continue,
    While {
        condition: Expression,
        body: Box<Self>,
    },
    DoWhile {
        condition: Expression,
        body: Box<Self>,
    },
    For {
        init: ForInit,
        condition: Option<Expression>,
        post: Option<Expression>,
        body: Box<Self>,
    },
    Null,
}

impl Statement {
    fn parse(tokens: &mut Vec<Token>) -> Self {
        match tokens.first().unwrap() {
            Token::Return => {
                consume(Token::Return, tokens);
                let expression = Expression::parse(tokens, 0);
                consume(Token::Semicolon, tokens);
                Self::Return(expression)
            }
            Token::Semicolon => {
                consume(Token::Semicolon, tokens);
                Self::Null
            }
            Token::If => {
                consume(Token::If, tokens);
                consume(Token::OpenParan, tokens);
                let condition = Expression::parse(tokens, 0);
                consume(Token::CloseParan, tokens);
                let then = Self::parse(tokens);
                let mut otherwise = None;
                if tokens.first() == Some(&Token::Else) {
                    consume(Token::Else, tokens);
                    otherwise = Some(Self::parse(tokens));
                }
                Self::If {
                    condition,
                    then: Box::new(then),
                    otherwise: otherwise.map(Box::new),
                }
            }
            Token::Break => {
                consume(Token::Break, tokens);
                consume(Token::Semicolon, tokens);
                Self::Break
            }
            Token::Continue => {
                consume(Token::Continue, tokens);
                consume(Token::Semicolon, tokens);
                Self::Continue
            }
            Token::While => {
                consume(Token::While, tokens);
                consume(Token::OpenParan, tokens);
                let condition = Expression::parse(tokens, 0);
                consume(Token::CloseParan, tokens);
                let body = Box::new(Self::parse(tokens));
                Self::While { condition, body }
            }
            Token::Do => {
                consume(Token::Do, tokens);
                let body = Box::new(Self::parse(tokens));
                consume(Token::While, tokens);
                consume(Token::OpenParan, tokens);
                let condition = Expression::parse(tokens, 0);
                consume(Token::CloseParan, tokens);
                consume(Token::Semicolon, tokens);
                Self::DoWhile { condition, body }
            }
            Token::For => {
                consume(Token::For, tokens);
                consume(Token::OpenParan, tokens);
                let init = ForInit::parse(tokens);
                let condition = Expression::parse_optional(tokens, Token::Semicolon);
                consume(Token::Semicolon, tokens);
                let post = Expression::parse_optional(tokens, Token::CloseParan);
                consume(Token::CloseParan, tokens);
                let body = Box::new(Self::parse(tokens));
                Self::For {
                    init,
                    condition,
                    post,
                    body,
                }
            }
            Token::OpenBrace => Self::Compound(Block::parse(tokens)),
            _exp => {
                let expression = Expression::parse(tokens, 0);
                consume(Token::Semicolon, tokens);
                Self::Expression(expression)
            }
        }
    }
}

#[derive(Debug)]
pub enum ForInit {
    D(VariableDeclaration),
    E(Option<Expression>),
}

impl ForInit {
    fn parse(tokens: &mut Vec<Token>) -> Self {
        if tokens.first() == Some(&Token::Int) {
            return Self::D(VariableDeclaration::parse(tokens));
        }

        if let Some(e) = Expression::parse_optional(tokens, Token::Semicolon) {
            consume(Token::Semicolon, tokens);
            Self::E(Some(e))
        } else {
            consume(Token::Semicolon, tokens);
            Self::E(None)
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Constant(Int),
    Var(Identifier),
    Unary {
        operator: UnaryOp,
        operand: Box<Self>,
    },
    Binary {
        operator: BinaryOp,
        left_expression: Box<Self>,
        right_expression: Box<Self>,
    },
    Assignment {
        lvalue: Box<Self>,
        expression: Box<Self>,
    },
    Conditional {
        condition: Box<Self>,
        then: Box<Self>,
        otherwise: Box<Self>,
    },
    FunctionCall {
        identifier: Identifier,
        args: Vec<Self>,
    },
}

impl Expression {
    fn parse(tokens: &mut Vec<Token>, min_prec: u8) -> Self {
        let mut left = Self::parse_factor(tokens);

        #[allow(clippy::while_let_loop)]
        loop {
            let Some(next) = tokens.first() else { break };
            let Ok(binary_op) = BinaryOp::parse(next) else {
                break;
            };
            let prec = binary_op.precedence();

            if prec < min_prec {
                break;
            }

            match binary_op {
                BinaryOp::Assignment => {
                    consume(Token::Equal, tokens);
                    let right = Self::parse(tokens, prec);
                    left = Self::Assignment {
                        lvalue: Box::new(left),
                        expression: Box::new(right),
                    };
                }
                BinaryOp::Ternary => {
                    let middle = {
                        consume(Token::QuestionMark, tokens);
                        let middle = Self::parse(tokens, 0);
                        consume(Token::Colon, tokens);
                        middle
                    };
                    let right = Self::parse(tokens, prec);
                    left = Self::Conditional {
                        condition: Box::new(left),
                        then: Box::new(middle),
                        otherwise: Box::new(right),
                    };
                }
                _ => {
                    tokens.remove(0);
                    let right = Self::parse(tokens, prec + 1);
                    left = Self::Binary {
                        operator: binary_op,
                        left_expression: Box::new(left),
                        right_expression: Box::new(right),
                    };
                }
            }
        }

        left
    }

    fn parse_optional(tokens: &mut Vec<Token>, end_token: Token) -> Option<Self> {
        if tokens.first() == Some(&end_token) {
            None
        } else {
            Some(Self::parse(tokens, 0))
        }
    }

    fn parse_factor(tokens: &mut Vec<Token>) -> Self {
        let token = tokens.remove(0);
        match UnaryOp::parse(&token) {
            Ok(unary_op) => Self::Unary {
                operator: unary_op,
                operand: Box::new(Self::parse_factor(tokens)),
            },
            Err(_) => match token {
                Token::Constant(i) => Self::Constant(i),
                Token::OpenParan => {
                    let expr = Self::parse(tokens, 0);
                    consume(Token::CloseParan, tokens);
                    expr
                }
                Token::Identifier(i) => {
                    if tokens.first() == Some(&Token::OpenParan) {
                        consume(Token::OpenParan, tokens);
                        let mut args: Vec<Self> = vec![];
                        if tokens.first() != Some(&Token::CloseParan) {
                            args.extend(Self::parse_argument_list(tokens));
                        }
                        consume(Token::CloseParan, tokens);
                        return Self::FunctionCall {
                            identifier: i,
                            args,
                        };
                    }
                    Self::Var(i)
                }
                t => panic!("Malformed factor: {t:?}"),
            },
        }
    }

    fn parse_argument_list(tokens: &mut Vec<Token>) -> Vec<Self> {
        let mut args: Vec<Self> = vec![];
        args.push(Self::parse(tokens, 0));
        while tokens.first() == Some(&Token::Comma) {
            consume(Token::Comma, tokens);
            args.push(Self::parse(tokens, 0));
        }
        args
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
    Assignment,
    Ternary,
}

impl BinaryOp {
    fn parse(token: &Token) -> Result<BinaryOp, &'static str> {
        Ok(match token {
            Token::Plus => Self::Add,
            Token::Minus => Self::Subtract,
            Token::Asterisk => Self::Multiply,
            Token::ForwardSlash => Self::Divide,
            Token::Percent => Self::Remainder,
            Token::DoubleAmpersands => Self::And,
            Token::DoubleBar => Self::Or,
            Token::DoubleEqual => Self::Equal,
            Token::ExclamationEqual => Self::NotEqual,
            Token::LeftAngleBracket => Self::LessThan,
            Token::RightAngleBracket => Self::GreaterThan,
            Token::LeftAngleBracketEqual => Self::LessOrEqual,
            Token::RightAngleBracketEqual => Self::GreaterOrEqual,
            Token::Equal => Self::Assignment,
            Token::QuestionMark => Self::Ternary,
            _ => return Err("Token does not correspond to binary operator"),
        })
    }

    fn precedence(&self) -> u8 {
        match self {
            Self::Multiply | Self::Divide | Self::Remainder => 50,
            Self::Add | Self::Subtract => 45,
            Self::LessThan
            | Self::LessOrEqual
            | Self::GreaterThan
            | Self::GreaterOrEqual => 35,
            Self::Equal | Self::NotEqual => 30,
            Self::And => 10,
            Self::Or => 5,
            Self::Ternary => 3,
            Self::Assignment => 1,
        }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
    Not,
}

impl UnaryOp {
    fn parse(token: &Token) -> Result<Self, &'static str> {
        Ok(match token {
            Token::Tilde => Self::Complement,
            Token::Minus => Self::Negate,
            Token::Exclamation => Self::Not,
            _ => return Err("Token does not correspond to unary operator"),
        })
    }
}

fn consume(expected: Token, tokens: &mut Vec<Token>) {
    let actual = tokens.remove(0);
    if actual != expected {
        panic!(
            "Syntax error: expected token '{:?}' did not match actual token '{:?}'",
            expected, actual
        )
    }
}
