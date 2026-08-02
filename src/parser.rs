use crate::lexer::Token;

pub fn parse(tokens: &mut Vec<Token>) -> Program {
    let function = Function::parse(tokens);
    let result = Program { function };
    if !tokens.is_empty() {
        panic!("Error: unexpected token '{:?} at end of program'", tokens)
    }
    result
}

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub body: Block,
}

impl Function {
    fn parse(tokens: &mut Vec<Token>) -> Function {
        consume(Token::Int, tokens);
        let identifier = parse_identifier(tokens);
        consume(Token::OpenParan, tokens);
        consume(Token::Void, tokens);
        consume(Token::CloseParan, tokens);
        let block = Block::parse(tokens);

        Function {
            name: identifier,
            body: block,
        }
    }
}

#[derive(Debug)]
pub struct Block(pub Vec<BlockItem>);

impl Block {
    fn parse(tokens: &mut Vec<Token>) -> Block {
        consume(Token::OpenBrace, tokens);
        let mut block_items = Vec::new();
        while tokens.first().is_some_and(|x| *x != Token::CloseBrace) {
            block_items.push(BlockItem::parse(tokens))
        }
        consume(Token::CloseBrace, tokens);

        Block(block_items)
    }
}

#[derive(Debug)]
pub enum BlockItem {
    S(Statement),
    D(Declaration),
}

impl BlockItem {
    fn parse(tokens: &mut Vec<Token>) -> BlockItem {
        match tokens.first().unwrap() {
            Token::Int => BlockItem::D(Declaration::parse(tokens)),
            _ => BlockItem::S(Statement::parse(tokens)),
        }
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub name: Identifier,
    pub init: Option<Expression>,
}

impl Declaration {
    fn parse(tokens: &mut Vec<Token>) -> Declaration {
        consume(Token::Int, tokens);
        let identifier = parse_identifier(tokens);
        let expression = if tokens.first() == Some(&Token::Equal) {
            consume(Token::Equal, tokens);
            Some(Expression::parse(tokens, 0))
        } else {
            None
        };
        consume(Token::Semicolon, tokens);
        Declaration {
            name: identifier,
            init: expression,
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
        then: Box<Statement>,
        otherwise: Option<Box<Statement>>,
    },
    Compound(Block),
    Break,
    Continue,
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    DoWhile {
        condition: Expression,
        body: Box<Statement>,
    },
    For {
        init: ForInit,
        condition: Option<Expression>,
        post: Option<Expression>,
        body: Box<Statement>,
    },
    Null,
}

impl Statement {
    fn parse(tokens: &mut Vec<Token>) -> Statement {
        match tokens.first().unwrap() {
            Token::Return => {
                consume(Token::Return, tokens);
                let expression = Expression::parse(tokens, 0);
                consume(Token::Semicolon, tokens);
                Statement::Return(expression)
            }
            Token::Semicolon => {
                consume(Token::Semicolon, tokens);
                Statement::Null
            }
            Token::If => {
                consume(Token::If, tokens);
                consume(Token::OpenParan, tokens);
                let condition = Expression::parse(tokens, 0);
                consume(Token::CloseParan, tokens);
                let then = Statement::parse(tokens);
                let mut otherwise = None;
                if tokens.first() == Some(&Token::Else) {
                    consume(Token::Else, tokens);
                    otherwise = Some(Statement::parse(tokens));
                }
                Statement::If {
                    condition,
                    then: Box::new(then),
                    otherwise: otherwise.map(Box::new),
                }
            }
            Token::Break => {
                consume(Token::Break, tokens);
                consume(Token::Semicolon, tokens);
                Statement::Break
            }
            Token::Continue => {
                consume(Token::Continue, tokens);
                consume(Token::Semicolon, tokens);
                Statement::Continue
            }
            Token::While => {
                consume(Token::While, tokens);
                consume(Token::OpenParan, tokens);
                let condition = Expression::parse(tokens, 0);
                consume(Token::CloseParan, tokens);
                let body = Box::new(Statement::parse(tokens));
                Statement::While { condition, body }
            }
            Token::Do => {
                consume(Token::Do, tokens);
                let body = Box::new(Statement::parse(tokens));
                consume(Token::While, tokens);
                consume(Token::OpenParan, tokens);
                let condition = Expression::parse(tokens, 0);
                consume(Token::CloseParan, tokens);
                consume(Token::Semicolon, tokens);
                Statement::DoWhile { condition, body }
            }
            Token::For => {
                consume(Token::For, tokens);
                consume(Token::OpenParan, tokens);
                let init = ForInit::parse(tokens);
                let condition = Expression::parse_optional(tokens, Token::Semicolon);
                consume(Token::Semicolon, tokens);
                let post = Expression::parse_optional(tokens, Token::CloseParan);
                consume(Token::CloseParan, tokens);
                let body = Box::new(Statement::parse(tokens));
                Statement::For {
                    init,
                    condition,
                    post,
                    body,
                }
            }
            Token::OpenBrace => Statement::Compound(Block::parse(tokens)),
            _exp => {
                let expression = Expression::parse(tokens, 0);
                consume(Token::Semicolon, tokens);
                Statement::Expression(expression)
            }
        }
    }
}

#[derive(Debug)]
pub enum ForInit {
    D(Declaration),
    E(Option<Expression>),
}

impl ForInit {
    fn parse(tokens: &mut Vec<Token>) -> ForInit {
        if tokens.first() == Some(&Token::Int) {
            return ForInit::D(Declaration::parse(tokens));
        }

        if let Some(e) = Expression::parse_optional(tokens, Token::Semicolon) {
            consume(Token::Semicolon, tokens);
            ForInit::E(Some(e))
        } else {
            consume(Token::Semicolon, tokens);
            ForInit::E(None)
        }
    }
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
    Conditional {
        condition: Box<Expression>,
        then: Box<Expression>,
        otherwise: Box<Expression>,
    },
}

impl Expression {
    fn parse(tokens: &mut Vec<Token>, min_prec: u8) -> Expression {
        let mut left = Expression::parse_factor(tokens);

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
                    let right = Expression::parse(tokens, prec);
                    left = Expression::Assignment {
                        lvalue: Box::new(left),
                        expression: Box::new(right),
                    };
                }
                BinaryOp::Ternary => {
                    let middle = {
                        consume(Token::QuestionMark, tokens);
                        let middle = Expression::parse(tokens, 0);
                        consume(Token::Colon, tokens);
                        middle
                    };
                    let right = Expression::parse(tokens, prec);
                    left = Expression::Conditional {
                        condition: Box::new(left),
                        then: Box::new(middle),
                        otherwise: Box::new(right),
                    };
                }
                _ => {
                    tokens.remove(0);
                    let right = Expression::parse(tokens, prec + 1);
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

    fn parse_optional(tokens: &mut Vec<Token>, end_token: Token) -> Option<Expression> {
        if tokens.first() == Some(&end_token) {
            None
        } else {
            Some(Expression::parse(tokens, 0))
        }
    }

    fn parse_factor(tokens: &mut Vec<Token>) -> Expression {
        let token = tokens.remove(0);
        match UnaryOp::parse(&token) {
            Ok(unary_op) => Expression::Unary {
                operator: unary_op,
                operand: Box::new(Expression::parse_factor(tokens)),
            },
            Err(_) => match token {
                Token::Constant(i) => Expression::Constant(i),
                Token::OpenParan => {
                    let expr = Expression::parse(tokens, 0);
                    consume(Token::CloseParan, tokens);
                    expr
                }
                Token::Identifier(i) => Expression::Var(i),
                t => panic!("Malformed factor: {:?}", t),
            },
        }
    }
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
    Ternary,
}

impl BinaryOp {
    fn parse(token: &Token) -> Result<BinaryOp, &'static str> {
        Ok(match token {
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
            Token::QuestionMark => BinaryOp::Ternary,
            _ => return Err("Token does not correspond to binary operator"),
        })
    }

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
            BinaryOp::Ternary => 3,
            BinaryOp::Assignment => 1,
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
            Token::Tilde => UnaryOp::Complement,
            Token::Minus => UnaryOp::Negate,
            Token::Exclamation => UnaryOp::Not,
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
