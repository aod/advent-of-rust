use std::unreachable;

use super::{
    syntax::{Equation, Operator},
    tokenizer::{tokenize, Token},
};

pub fn eval(s: &str) -> u64 {
    let tokens = tokenize(s)
        .into_iter()
        .map(|token| match token {
            Token::LParen => Token::RParen,
            Token::RParen => Token::LParen,
            _ => token,
        })
        .rev()
        .collect::<Vec<_>>();
    let equation = Equation::from(tokens);

    eval_helper(&equation)
}

pub fn eval2(s: &str) -> u64 {
    let mut equation = Equation::from(tokenize(s));
    eval_adds(&mut equation);

    match equation {
        Equation::Number(result) => result,
        _ => unreachable!(),
    }
}

fn eval_helper(eq: &Equation) -> u64 {
    match eq {
        Equation::Number(val) => *val,
        Equation::Expr(lhs, op, rhs) => {
            let lhs = eval_helper(&lhs);
            let rhs = eval_helper(&rhs);

            match op {
                Operator::Add => lhs + rhs,
                Operator::Multiply => lhs * rhs,
            }
        }
        Equation::Group(x) => eval_helper(x),
    }
}

fn eval_adds(eq: &mut Equation) {
    use Equation::*;
    use Operator::*;
    match eq {
        Expr(lhs, op, rhs) => {
            if let Equation::Group(_) = &**lhs {
                eval_adds(lhs);
            }
            if let Equation::Group(_) = &**rhs {
                eval_adds(rhs);
            }

            if *op == Multiply {
                eval_adds(lhs);
                eval_adds(rhs);
            }

            // NOTE: Hidden logic here...
            // If operator is multiplication the following match will always fall in the first case. This is because
            // multiplication is the last step after we eval'd it in the if above, forcing the result of lhs and rhs.
            // Only the two reamining numbers will be left. After this we will exit the function.
            match (&mut **lhs, &mut **rhs) {
                (Number(lval), Number(rval)) => {
                    *eq = match op {
                        Add => Number(*lval + *rval),
                        Multiply => Number(*lval * *rval),
                    };
                }

                // Opposite case doesn't occur in parse tree :). Expr can never be on the left side because of how the
                // parse tree is build. This also catched the (Expr, Expr) case, which again never occurrs.
                (Number(left), Expr(mid, op2, right)) => match &mut **mid {
                    Number(midval) => {
                        *eq = Equation::new_expr(Number(*left + *midval), *op2, *right.clone());
                        eval_adds(eq);
                    }
                    Group(_) => {
                        eval_adds(mid);
                        eval_adds(eq);
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        Group(inner) => {
            eval_adds(inner);
            *eq = *inner.clone();
        }
        Number(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::eval;

    #[test]
    fn eval_single_number() {
        assert_eq!(eval("5"), 5);
        assert_eq!(eval("8"), 8);
    }

    #[test]
    fn eval_simple_add() {
        assert_eq!(eval("5 + 5"), 10);
        assert_eq!(eval("5 + 8"), 13);
    }

    #[test]
    fn eval_simple_mul() {
        assert_eq!(eval("5 * 5"), 25);
        assert_eq!(eval("5 * 8"), 40);
    }

    #[test]
    fn eval_chained_ops() {
        assert_eq!(eval("5 + 5 + 5"), 15);
        assert_eq!(eval("5 + 5 + 8"), 18);
        assert_eq!(eval("1 + 2 * 3 + 4 * 5 + 6"), 71);
    }

    #[test]
    fn eval_simple_bracket() {
        assert_eq!(eval("(4 * 5)"), 20);
        assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
    }

    #[test]
    fn eval_nested_brackets() {
        assert_eq!(eval("((4 * 5))"), 20);
        assert_eq!(eval("(((4 * 5)))"), 20);
    }

    #[test]
    fn eval_rest_examples() {
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
}
