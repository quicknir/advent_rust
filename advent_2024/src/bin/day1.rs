use utils::*;

fn parse(input: &str) -> impl Iterator<Item=(i64, i64)> {
    input.split_terminator('\n').map(|line| {
        let (i1, i2) = line.split_once("   ").unwrap();
        (i1.parse().unwrap(), i2.parse().unwrap())
    })
}

fn part1(input: &str) -> i64 {

    let (mut v1, mut v2): (Vec<_>, Vec<_>) = parse(input).unzip(); 
    v1.sort();
    v2.sort();

    v1.iter().zip(v2.iter()).fold(0, |acc, (i1, i2)| {
        acc + (*i1 - *i2).abs()
    })
}

fn part2(input: &str) -> i64 {
    let mut v1 = vec![];
    let mut h2: HashMap<i64, i64> = HashMap::new();
    parse(input).for_each(|(i1, i2)| {
        v1.push(i1);
        *h2.entry(i2).or_default() += 1;
    });
    v1.iter().fold(0, |acc, e| {
        acc + e * h2.get(e).unwrap_or(&0)
    })
}

fn main() {
    let s = read_aoc!();
    println!("{}", part1(&s));
    println!("{}", part2(&s));
}