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

    // println!("{:?}", tokens);
    let equation = Equation::from(tokens);

    // println!("{:?}", &equation);
    eval_helper(&equation)
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
