#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum Token {
    Number(u64),
    Plus,
    Minus,
    LParen,
    RParen,
}

pub(super) fn tokenize(s: &str) -> Vec<Token> {
    let mut parsed = vec![];
    let tokens = s.chars().filter(|c| !c.is_whitespace());

    for token in tokens {
        let token = match token {
            '+' => Token::Plus,
            '*' => Token::Minus,
            '(' => Token::LParen,
            ')' => Token::RParen,
            n => Token::Number(n.to_digit(10).unwrap() as u64),
        };
        parsed.push(token);
    }

    parsed
}
