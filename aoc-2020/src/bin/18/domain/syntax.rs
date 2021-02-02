use std::{fmt::Display, iter::Peekable};

use super::tokenizer::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Operator {
    Add,
    Multiply,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Multiply => write!(f, "*"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Equation {
    Number(u64),
    Expr(Box<Equation>, Operator, Box<Equation>),
    Group(Box<Equation>),
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Equation::Number(num) => write!(f, "{}", num),
            Equation::Expr(lhs, op, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Equation::Group(group) => write!(f, "({})", group),
        }
    }
}

impl Equation {
    pub(super) fn new_expr(lhs: Self, op: Operator, rhs: Self) -> Self {
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
