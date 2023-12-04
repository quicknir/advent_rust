use std::collections::VecDeque;
use utils::*;
use microbench::{self, Options};

fn count_matches(input: &str, set: &mut HashSet<i32>) -> usize {
    set.clear();
    let (_card, input) = input.split_once(": ").unwrap();
    let (winners, have) = input.split_once(" | ").unwrap();
    set.extend(winners
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
);
    have.split_whitespace()
        .map(|x| x.parse().unwrap())
        .filter(|x| set.contains(x))
        .count()
}

fn part1(input: &str) -> usize {
    let mut set = HashSet::new();
    input
        .split_terminator('\n')
        .map(|line| {
            let matches = count_matches(line, &mut set);
            if matches == 0 {
                0
            } else {
                2usize.pow(matches as u32 - 1)
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut set = HashSet::new();
    let mut copies = VecDeque::new();
    let mut total_cards = 0;
    for line in input.split_terminator('\n') {
        let cur_copies = copies.pop_front().unwrap_or(0) + 1;
        total_cards += cur_copies;
        let matches = count_matches(line, &mut set);
        if copies.len() < matches {
            copies.resize(matches, 0);
        }
        for i in 0..matches {
            copies[i] += cur_copies;
        }
    }
    total_cards
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(13, part1(TEST_INPUT));
    }
    #[test]
    fn test_part2() {
        assert_eq!(30, part2(TEST_INPUT));
    }
}

fn main() {
    let s = read_aoc!();
    let options = Options::default();
    microbench::bench(&options, "part1", || part1(&s));
    microbench::bench(&options, "part2", || part2(&s));
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&s));
}
