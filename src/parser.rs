use std::panic;

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
}

impl BinaryOp {
    fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Remainder => 50,
            BinaryOp::Add | BinaryOp::Subtract => 45,
        }
    }

    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Subtract),
            Token::Asterisk => Some(BinaryOp::Multiply),
            Token::ForwardSlash => Some(BinaryOp::Divide),
            Token::Percent => Some(BinaryOp::Remainder),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
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

    while let Some(op) = peek_binary_op(tokens) {
        let prec = BinaryOp::precedence(&op);
        if prec < min_prec {
            break;
        }

        parse_binary_op(tokens);

        let right = parse_expression(tokens, prec + 1);

        left = Expression::Binary {
            operator: op,
            left_expression: Box::new(left),
            right_expression: Box::new(right),
        };
    }

    left
}

fn peek_binary_op(tokens: &Vec<Token>) -> Option<BinaryOp> {
    tokens.first().and_then(BinaryOp::from_token)
}

fn parse_binary_op(tokens: &mut Vec<Token>) -> BinaryOp {
    let token = tokens.remove(0);
    BinaryOp::from_token(&token)
        .unwrap_or_else(|| panic!("Expected binary operator, found '{:?}'", token))
}

fn parse_factor(tokens: &mut Vec<Token>) -> Expression {
    match tokens.remove(0) {
        Token::Constant(i) => Expression::Factor(Factor::Constant(parse_int(i))),
        Token::Minus => Expression::Factor(Factor::Unary {
            operator: UnaryOp::Negate,
            operand: Box::new(parse_factor(tokens)),
        }),
        Token::Tilde => Expression::Factor(Factor::Unary {
            operator: UnaryOp::Complement,
            operand: Box::new(parse_factor(tokens)),
        }),
        Token::OpenParan => {
            let expr = parse_expression(tokens, 0);
            consume(Token::CloseParan, tokens);
            expr
        }
        token => panic!("Malformed factor: {:?}", token),
    }
}

fn parse_int(int: i32) -> Int {
    int
}
