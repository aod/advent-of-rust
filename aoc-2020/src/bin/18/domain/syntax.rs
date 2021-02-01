use std::iter::Peekable;

use super::tokenizer::Token;

#[derive(Debug)]
pub(super) enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
pub(super) enum Equation {
    Number(u64),
    Expr(Box<Equation>, Operator, Box<Equation>),
    Group(Box<Equation>),
}

impl Equation {
    fn new_expr(lhs: Self, op: Operator, rhs: Self) -> Self {
        Self::Expr(Box::new(lhs), op, Box::new(rhs))
    }

    fn new_group(eq: Equation) -> Self {
        Self::Group(Box::new(eq))
    }
}

impl From<Vec<Token>> for Equation {
    fn from(tokens: Vec<Token>) -> Self {
        build_equation_helper(&mut tokens.into_iter().peekable())
    }
}

fn build_equation_helper(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Equation {
    match tokens.next() {
        Some(token) => match token {
            Token::Number(lhs) => {
                if tokens.peek().is_none() {
                    return Equation::Number(lhs);
                }
                let op = match tokens.next().unwrap() {
                    Token::Plus => Operator::Add,
                    Token::Minus => Operator::Multiply,
                    Token::RParen => return Equation::Number(lhs),
                    _ => unreachable!(),
                };

                Equation::new_expr(Equation::Number(lhs), op, build_equation_helper(tokens))
            }
            Token::LParen => {
                let lhs = Equation::new_group(build_equation_helper(tokens));
                if tokens.peek().is_none() {
                    return lhs;
                }
                let op = match tokens.next().unwrap() {
                    Token::Plus => Operator::Add,
                    Token::Minus => Operator::Multiply,
                    Token::RParen => return lhs,
                    _ => unreachable!(),
                };

                Equation::new_expr(lhs, op, build_equation_helper(tokens))
            }
            _ => unreachable!(),
        },
        None => panic!("invalid equation"),
    }
}
