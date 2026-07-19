use crate::lexer::Token;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub body: Statement,
}

pub type Identifier = String;

pub type Int = i32;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Factor(Factor),
    Binary {
        operator: BinaryOp,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Factor {
    Constant(Int),
    Unary {
        operator: UnaryOp,
        operand: Box<Expression>,
    },
}

#[derive(Debug)]
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
}

impl BinaryOp {
    fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Remainder => 50,
            BinaryOp::Add | BinaryOp::Subtract => 45,
            BinaryOp::LessThan
            | BinaryOp::LessOrEqual
            | BinaryOp::GreaterThan
            | BinaryOp::GreaterOrEqual => 35,
            BinaryOp::Equal | BinaryOp::NotEqual => 30,
            BinaryOp::And => 10,
            BinaryOp::Or => 5,
        }
    }

    fn from_token(token: &Token) -> Option<Self> {
        Some(match token {
            Token::Plus => BinaryOp::Add,
            Token::Minus => BinaryOp::Subtract,
            Token::Asterisk => BinaryOp::Multiply,
            Token::ForwardSlash => BinaryOp::Divide,
            Token::Percent => BinaryOp::Remainder,
            Token::DoubleAmpersands => BinaryOp::And,
            Token::DoubleBar => BinaryOp::Or,
            Token::DoubleEqual => BinaryOp::Equal,
            Token::ExclamationEqual => BinaryOp::NotEqual,
            Token::LeftAngleBracket => BinaryOp::LessThan,
            Token::RightAngleBracket => BinaryOp::GreaterThan,
            Token::LeftAngleBracketEqual => BinaryOp::LessOrEqual,
            Token::RightAngleBracketEqual => BinaryOp::GreaterOrEqual,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
    Not,
}

impl UnaryOp {
    fn from_token(token: &Token) -> Option<Self> {
        Some(match token {
            Token::Tilde => UnaryOp::Complement,
            Token::Minus => UnaryOp::Negate,
            Token::Exclamation => UnaryOp::Not,
            _ => return None,
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

pub fn parse(tokens: &mut Vec<Token>) -> Program {
    let function = parse_function(tokens);
    let result = Program { function };
    if !tokens.is_empty() {
        panic!("Error: unexpected token '{:?} at end of program'", tokens)
    }
    result
}

fn parse_function(tokens: &mut Vec<Token>) -> Function {
    consume(Token::Int, tokens);
    let identifier = parse_identifier(tokens);
    consume(Token::OpenParan, tokens);
    consume(Token::Void, tokens);
    consume(Token::CloseParan, tokens);
    consume(Token::OpenBrace, tokens);
    let statement = parse_statement(tokens);
    consume(Token::CloseBrace, tokens);
    Function {
        name: identifier,
        body: statement,
    }
}

fn parse_identifier(tokens: &mut Vec<Token>) -> Identifier {
    match tokens.remove(0) {
        Token::Identifier(x) => x,
        value => panic!("Syntax error: expected <Identifier>, found: '{:?}'", value),
    }
}

fn parse_statement(tokens: &mut Vec<Token>) -> Statement {
    match tokens.remove(0) {
        Token::Return => {
            let expression = parse_expression(tokens, 0);
            consume(Token::Semicolon, tokens);
            return Statement::Return(expression);
        }
        value => panic!("Syntax error: expected <return>, found: '{:?}'", value),
    }
}

fn parse_expression(tokens: &mut Vec<Token>, min_prec: u8) -> Expression {
    let mut left = parse_factor(tokens);

    while let Some(binary_op) = tokens.first().and_then(BinaryOp::from_token) {
        let prec = BinaryOp::precedence(&binary_op);
        if prec < min_prec {
            break;
        }

        tokens.remove(0);

        let right = parse_expression(tokens, prec + 1);

        left = Expression::Binary {
            operator: binary_op,
            left_expression: Box::new(left),
            right_expression: Box::new(right),
        };
    }

    left
}

fn parse_factor(tokens: &mut Vec<Token>) -> Expression {
    match tokens.first().and_then(UnaryOp::from_token) {
        Some(unary_op) => {
            tokens.remove(0);
            Expression::Factor(Factor::Unary {
                operator: unary_op,
                operand: Box::new(parse_factor(tokens)),
            })
        }
        None => match tokens.remove(0) {
            Token::Constant(i) => Expression::Factor(Factor::Constant(parse_int(i))),
            Token::OpenParan => {
                let expr = parse_expression(tokens, 0);
                consume(Token::CloseParan, tokens);
                expr
            }
            token => panic!("Malformed factor: {:?}", token),
        },
    }
}

fn parse_int(int: i32) -> Int {
    int
}
