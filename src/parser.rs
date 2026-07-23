use crate::lexer::Token;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub body: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    S(Statement),
    D(Declaration),
}

#[derive(Debug)]
pub struct Declaration {
    name: Identifier,
    init: Option<Expression>,
}

pub type Identifier = String;

pub type Int = i32;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    Null,
}

#[derive(Debug)]
pub enum Expression {
    Constant(Int),
    Var(Identifier),
    Unary {
        operator: UnaryOp,
        operand: Box<Expression>,
    },
    Binary {
        operator: BinaryOp,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    Assignment {
        lvalue: Box<Expression>,
        expression: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
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
            BinaryOp::Assignment => 1,
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
            Token::Equal => BinaryOp::Assignment,
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

    let mut block_items = Vec::new();
    while tokens.first().is_some_and(|x| *x != Token::CloseBrace) {
        block_items.push(parse_block_items(tokens))
    }

    consume(Token::CloseBrace, tokens);
    Function {
        name: identifier,
        body: block_items,
    }
}

fn parse_block_items(tokens: &mut Vec<Token>) -> BlockItem {
    match tokens.first().unwrap() {
        Token::Int => BlockItem::D(parse_declaration(tokens)),
        _ => BlockItem::S(parse_statement(tokens)),
    }
}

fn parse_identifier(tokens: &mut Vec<Token>) -> Identifier {
    match tokens.remove(0) {
        Token::Identifier(x) => x,
        value => panic!("Syntax error: expected <Identifier>, found: '{:?}'", value),
    }
}

fn parse_declaration(tokens: &mut Vec<Token>) -> Declaration {
    consume(Token::Int, tokens);
    let identifier = parse_identifier(tokens);
    let expression = if tokens.first() == Some(&Token::Equal) {
        consume(Token::Equal, tokens);
        Some(parse_expression(tokens, 0))
    } else {
        None
    };
    consume(Token::Semicolon, tokens);
    Declaration {
        name: identifier,
        init: expression,
    }
}

fn parse_statement(tokens: &mut Vec<Token>) -> Statement {
    match tokens.first().unwrap() {
        Token::Return => {
            consume(Token::Return, tokens);
            let expression = parse_expression(tokens, 0);
            consume(Token::Semicolon, tokens);
            Statement::Return(expression)
        }
        Token::Semicolon => {
            consume(Token::Semicolon, tokens);
            Statement::Null
        }
        _ => {
            let expression = parse_expression(tokens, 0);
            consume(Token::Semicolon, tokens);
            Statement::Expression(expression)
        }
    }
}

fn parse_expression(tokens: &mut Vec<Token>, min_prec: u8) -> Expression {
    let mut left = parse_factor(tokens);

    while let Some(binary_op) = tokens.first().and_then(BinaryOp::from_token) {
        let prec = binary_op.precedence();

        if prec < min_prec {
            break;
        }

        match binary_op {
            BinaryOp::Assignment => {
                consume(Token::Equal, tokens);
                let right = parse_expression(tokens, prec);
                left = Expression::Assignment {
                    lvalue: Box::new(left),
                    expression: Box::new(right),
                };
            }
            _ => {
                tokens.remove(0);
                let right = parse_expression(tokens, prec + 1);
                left = Expression::Binary {
                    operator: binary_op,
                    left_expression: Box::new(left),
                    right_expression: Box::new(right),
                };
            }
        }
    }

    left
}

fn parse_factor(tokens: &mut Vec<Token>) -> Expression {
    match tokens.first().and_then(UnaryOp::from_token) {
        Some(unary_op) => {
            tokens.remove(0);
            Expression::Unary {
                operator: unary_op,
                operand: Box::new(parse_factor(tokens)),
            }
        }
        None => match tokens.remove(0) {
            Token::Constant(i) => Expression::Constant(parse_int(i)),
            Token::OpenParan => {
                let expr = parse_expression(tokens, 0);
                consume(Token::CloseParan, tokens);
                expr
            }
            Token::Identifier(identifier) => Expression::Var(identifier),
            token => panic!("Malformed factor: {:?}", token),
        },
    }
}

fn parse_int(int: i32) -> Int {
    int
}
