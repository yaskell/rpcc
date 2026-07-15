use std::panic;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Identifier,
    pub body: Statement,
}

pub type Identifier = String;

pub type Int = i32;

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(Int),
    Unary(UnaryOp, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Complement,
    Negate,
}

pub fn parse(tokens: &mut Vec<Token>) -> Program {
    parse_program(tokens)
}

fn parse_program(tokens: &mut Vec<Token>) -> Program {
    let function_val = parse_function(tokens);
    let result = Program {
        function: function_val,
    };
    if !tokens.is_empty() {
        panic!("Error: unexpected token '{:?} at end of program'", tokens)
    }
    result
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

fn parse_statement(tokens: &mut Vec<Token>) -> Statement {
    consume(Token::Return, tokens);
    let return_val = parse_expression(tokens);
    consume(Token::Semicolon, tokens);
    Statement::Return(return_val)
}

fn parse_expression(tokens: &mut Vec<Token>) -> Expression {
    match tokens.remove(0) {
        Token::Constant(x) => Expression::Constant(parse_int(x)),
        Token::Tilde => Expression::Unary(UnaryOp::Complement, Box::new(parse_expression(tokens))),
        Token::Minus => Expression::Unary(UnaryOp::Negate, Box::new(parse_expression(tokens))),
        Token::OpenParan => {
            let inner_expression = parse_expression(tokens);
            consume(Token::CloseParan, tokens);
            return inner_expression;
        }
        value => panic!("Malformed expression: '{:?}'", value),
    }
}

fn parse_identifier(tokens: &mut Vec<Token>) -> Identifier {
    match tokens.remove(0) {
        Token::Identifier(x) => x,
        value => panic!("Syntax error: expected <Identifier>, found: '{:?}'", value),
    }
}

fn parse_int(int: i32) -> Int {
    int
}

fn parse_function(tokens: &mut Vec<Token>) -> Function {
    consume(Token::Int, tokens);
    let identifier_val = parse_identifier(tokens);
    consume(Token::OpenParan, tokens);
    consume(Token::Void, tokens);
    consume(Token::CloseParan, tokens);
    consume(Token::OpenBrace, tokens);
    let statement_val = parse_statement(tokens);
    consume(Token::CloseBrace, tokens);
    Function {
        name: identifier_val,
        body: statement_val,
    }
}
