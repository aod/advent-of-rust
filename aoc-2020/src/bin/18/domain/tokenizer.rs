#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum Token {
    Number(u64),
    Plus,
    Minus,
    LParen,
    RParen,
}

pub(super) fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = s.chars().filter(|c| !c.is_whitespace()).peekable();

    while let Some(char) = chars.next() {
        let token = match char {
            '+' => Token::Plus,
            '*' => Token::Minus,
            '(' => Token::LParen,
            ')' => Token::RParen,
            n if n.is_digit(10) => {
                let mut s = n.to_digit(10).unwrap() as u64;
                loop {
                    match chars.peek() {
                        Some(n) if n.is_digit(10) => {
                            s *= 10;
                            s += chars.next().unwrap().to_digit(10).unwrap() as u64;
                        }
                        _ => break,
                    }
                }
                Token::Number(s)
            }
            _ => panic!("invalid token {}", char),
        };
        tokens.push(token);
    }

    tokens
}
