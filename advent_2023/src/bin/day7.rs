use utils::*;
use microbench::{self, Options};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    cards: [u8; 5],
    bid: usize,
}

fn get_card(c: char) -> u8 {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .split_terminator('\n')
        .map(|line| {
            let mut it = line.split_whitespace();
            let cards = it.next().unwrap();
            let mut card_it = cards.chars();
            let cards = std::array::from_fn(|_| get_card(card_it.next().unwrap()));
            let bid = it.next().unwrap().parse().unwrap();
            Hand { cards, bid }
        })
        .collect()
}

fn hand_type(h: &Hand) -> u8 {
    let mut counts = [0u8; 14]; // 14 to allow for either jokers or jacks
    for c in h.cards.iter() {
        counts[(*c - 1) as usize] += 1;
    }
    let m = counts[1..].iter().max().unwrap() + counts[0]; // max excluding jokers + jokers
    match m {
        1 => return 0, // high card
        4 => return 5, // four of a kind
        5 => return 6, // five of a kind
        _ => (),
    };
    let s = counts[1..].iter().filter(|x| **x > 1).sum::<u8>() + counts[0];
    match (m, s) {
        (3, 5) => 4, // full house
        (3, _) => 3, // 3 of a kind
        (2, 4) => 2, // two pair
        (2, _) => 1, // pair
        _ => unreachable!(),
    }
}

fn part1(data: &[Hand]) -> usize {
    let mut v = data.iter().map(|h| (hand_type(h), h.clone())).to_vec();
    v.sort_unstable();
    v.iter().enumerate().map(|(i, h)| (i + 1) * h.1.bid).sum()
}

fn part2(data: &[Hand]) -> usize {
    let mut v = data
        .iter()
        .map(|h| {
            let mut h = h.clone();
            for c in h.cards.iter_mut() {
                if *c == 11 {
                    *c = 1;
                }
            }
            (hand_type(&h), h)
        })
        .to_vec();
    v.sort_unstable();
    v.iter().enumerate().map(|(i, h)| (i + 1) * h.1.bid).sum()
}

fn benchmark(s: &str) {
    let options = Options::default();
    microbench::bench(&options, "parsing", || {
        parse(&s);
    });
    let data = parse(&s);
    microbench::bench(&options, "part1", || {
        part1(&data);
    });
    microbench::bench(&options, "part2", || {
        part2(&data);
    });
    microbench::bench(&options, "combined", || {
        let data = parse(&s);
        part1(&data);
        part2(&data);
    });
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
    benchmark(&s);
}
