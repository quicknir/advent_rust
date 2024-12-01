use utils::*;

fn part1(input: &str) -> i64 {
    let mut v1 = vec![];
    let mut v2 = vec![];

    for line in input.split_terminator("\n") {
        let (i1, i2) = line.split_once("   ").unwrap();
        v1.push(i1.parse::<i64>().unwrap());
        v2.push(i2.parse::<i64>().unwrap());
    }
    v1.sort();
    v2.sort();

    v1.iter().zip(v2.iter()).fold(0, |acc, (i1, i2)| {
        acc + (*i1 - *i2).abs()
    })
}

fn part2(input: &str) -> i64 {
    let mut v1 = vec![];
    let mut h2: HashMap<i64, i64> = HashMap::new();
    for line in input.split_terminator("\n") {
        let (i1, i2) = line.split_once("   ").unwrap();
        v1.push(i1.parse::<i64>().unwrap());
        *h2.entry(i2.parse().unwrap()).or_default() += 1;
    }
    v1.iter().fold(0, |acc, e| {
        acc + e * h2.get(e).unwrap_or(&0)
    })
}

fn main() {
    let s = read_aoc!();
    println!("{}", part1(&s));
    println!("{}", part2(&s));
}