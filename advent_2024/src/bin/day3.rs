use utils::*;

enum Token {
    Do,
    Dont,
    Mul(i64, i64),
}

fn parse(mut input: &str) -> Vec<Token> {
    let mut tokens = vec![];

    while !input.is_empty() {
        if input.starts_with("do()") {
            tokens.push(Token::Do);
            input = &input[4..];
            continue;
        }
        if input.starts_with("don't()") {
            tokens.push(Token::Dont);
            input = &input[7..];
            continue;
        }
        if !input.starts_with("mul(") {
            input = &input[1..];
            continue;
        }
        input = &input[4..];
        let Some(end_paren) = input.find(")") else {
            continue;
        };
        let Some((first, second)) = input[..end_paren].split_once(',') else {
            continue;
        };
        let (Ok(first), Ok(second)) = (first.parse::<i64>(), second.parse::<i64>()) else {
            continue;
        };
        tokens.push(Token::Mul(first, second));
        input = &input[end_paren..];
    }
    tokens
}

fn part1(tokens: &[Token]) -> i64 {
    tokens.iter().fold(0, |acc, e| {
        let x = if let Token::Mul(x, y) = e {
            x*y
        }
        else {
            0
        };
        acc + x
    })
}

fn part2(tokens: &[Token]) -> i64 {
    let mut mult_enabled = true;
    tokens.iter().fold(0, |acc, e| {
        let x = match e {
            Token::Do => {
                mult_enabled = true;
                0
            },
            Token::Dont => {
                mult_enabled = false;
                0
            },
            Token::Mul(x, y) if mult_enabled => {
                x * y
            },
            _ => 0,
        };
        acc + x
    })
}

fn main() {
    let s = read_aoc!();
    let parsed = parse(&s);
    println!("{:?}", part1(&parsed));
    println!("{:?}", part2(&parsed));
}
