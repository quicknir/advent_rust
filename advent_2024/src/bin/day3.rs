use utils::*;


fn part1(mut input: &str) -> i64 {
    let mut sum = 0;
    while let Some(i) = input.find("mul(") {
        input = &input[(i + 4)..];
        let Some(end_paren) = input.find(")") else {
            continue;
        };
        let Some((first, second)) = input[..end_paren].split_once(',') else {
            continue;
        };
        let (Ok(first), Ok(second)) = (first.parse::<i64>(), second.parse::<i64>()) else {
            continue;
        };

        sum += first * second;
    }
    sum
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    let mut mult_enabled = true;
    for i in 0..input.len() {
        if input[i..].starts_with("do()") {
            mult_enabled = true;
            continue;
        }
        if input[i..].starts_with("don't()") {
            mult_enabled = false;
            continue;
        }
        if !input[i..].starts_with("mul(") || !mult_enabled {
            continue;
        }
        let s = &input[(i + 4)..];
        let Some(end_paren) = s.find(")") else {
            continue;
        };
        let Some((first, second)) = s[..end_paren].split_once(',') else {
            continue;
        };
        let (Ok(first), Ok(second)) = (first.parse::<i64>(), second.parse::<i64>()) else {
            continue;
        };

        sum += first * second;
    }
    sum
}

fn main() {
    let s = read_aoc!();
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&s));
}
