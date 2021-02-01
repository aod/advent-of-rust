use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Token {
    Number(u64),
    Add,
    Mul,
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut parsed = vec![];
    let mut tokens = s.chars().filter(|c| !c.is_whitespace());

    while let Some(token) = tokens.next() {
        match token {
            '+' => parsed.push(Token::Add),
            '*' => parsed.push(Token::Mul),
            '(' => parsed.push(Token::LParen),
            ')' => parsed.push(Token::RParen),
            n => parsed.push(Token::Number(n.to_digit(10).unwrap() as u64)),
        }
    }

    parsed
}

pub fn dumb_eval(s: &str) -> u64 {
    let tokens = tokenize(s);

    eval_helper(
        &mut tokens
            .into_iter()
            .map(|t| match t {
                Token::LParen => Token::RParen,
                Token::RParen => Token::LParen,
                _ => t,
            })
            .rev()
            .peekable(),
    )
}

fn eval_helper(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> u64 {
    let token = match tokens.next() {
        Some(token) => token,
        None => return 0,
    };

    match token {
        Token::Number(val) => {
            if tokens.peek().is_none() {
                return val;
            }

            let op = tokens.next().unwrap();
            if Token::RParen == op {
                return val;
            }

            let rhs = eval_helper(tokens);
            match op {
                Token::Add => return val + rhs,
                Token::Mul => return val * rhs,
                _ => unreachable!(),
            }
        }

        Token::LParen => {
            let lhs = eval_helper(tokens);
            if let Some(op) = tokens.next() {
                match op {
                    Token::Add => return lhs + eval_helper(tokens),
                    Token::Mul => return lhs * eval_helper(tokens),
                    Token::LParen => eval_helper(tokens),
                    Token::RParen => return lhs,
                    _ => unreachable!(),
                }
            } else {
                return lhs;
            }
        }

        token => panic!("unexpected token: {:?}", token),
    }
}

#[cfg(test)]
mod tests {
    use super::dumb_eval;

    #[test]
    fn eval_single_number() {
        assert_eq!(dumb_eval("5"), 5);
        assert_eq!(dumb_eval("8"), 8);
    }

    #[test]
    fn eval_simple_add() {
        assert_eq!(dumb_eval("5 + 5"), 10);
        assert_eq!(dumb_eval("5 + 8"), 13);
    }

    #[test]
    fn eval_simple_mul() {
        assert_eq!(dumb_eval("5 * 5"), 25);
        assert_eq!(dumb_eval("5 * 8"), 40);
    }

    #[test]
    fn eval_chained_ops() {
        assert_eq!(dumb_eval("5 + 5 + 5"), 15);
        assert_eq!(dumb_eval("5 + 5 + 8"), 18);
        assert_eq!(dumb_eval("1 + 2 * 3 + 4 * 5 + 6"), 71);
    }

    #[test]
    fn eval_simple_bracket() {
        assert_eq!(dumb_eval("(4 * 5)"), 20);
        assert_eq!(dumb_eval("2 * 3 + (4 * 5)"), 26);
    }

    #[test]
    fn eval_nested_brackets() {
        assert_eq!(dumb_eval("((4 * 5))"), 20);
        assert_eq!(dumb_eval("(((4 * 5)))"), 20);
    }

    #[test]
    fn eval_rest_examples() {
        assert_eq!(dumb_eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            dumb_eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            dumb_eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
}
