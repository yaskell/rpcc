use std::panic;

use crate::token::Token;
use crate::ast::*;

pub fn parse(tokens: &mut Vec<Token>) -> Program {
    parse_program(tokens)
}

fn parse_program(tokens: &mut Vec<Token>) -> Program {
    let function_val = parse_function(tokens);
    Program { function_definition: function_val }
}

fn expect(expected: Token, tokens: &mut Vec<Token>) {
    let actual = tokens.remove(0);
    if actual != expected {
        panic!("Syntax error: expected token '{:?}' did not match actual token '{:?}'", expected, actual)
    }
}

fn parse_statement(tokens: &mut Vec<Token>) -> Statement {
    expect(Token::Return, tokens);
    let return_val = parse_expression(tokens);
    expect(Token::Semicolon, tokens);
    Statement::Return(return_val)
}

fn parse_expression(tokens: &mut Vec<Token>) -> Expression {
    match tokens.remove(0) {
        Token::Constant(x) => Expression::Constant(parse_int(x)),
        value => panic!("Syntax error: expected <constant>, found '{:?}'", value)
    }
}

fn parse_identifier(tokens: &mut Vec<Token>) -> Identifier {
    match tokens.remove(0) {
        Token::Identifier(x) => Identifier(x),
        value => panic!("Syntax error: expected <Identifier>, found: '{:?}'", value)
    }
}

fn parse_int(int: i32) -> Int {
    Int(int)
}

fn parse_function(tokens: &mut Vec<Token>) -> FunctionDefinition {
    expect(Token::Int, tokens);
    let identifier_val = parse_identifier(tokens);
    expect(Token::OpenParan, tokens);
    expect(Token::Void, tokens);
    expect(Token::CloseParan, tokens);
    expect(Token::OpenBrace, tokens);
    let statement_val = parse_statement(tokens);
    expect(Token::CloseBrace, tokens);
    FunctionDefinition {
        name: identifier_val,
        body: statement_val,
    }
}
