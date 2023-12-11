use microbench::{self, Options};
use std::cmp::min;
use utils::*;

fn count_matches(line: &str, set: &mut HashSet<i32>) -> usize {
    set.clear();
    let (_card, line) = line.split_once(": ").unwrap();
    let (winners, have) = line.split_once(" | ").unwrap();
    set.extend(
        winners
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap()),
    );
    have.split_whitespace()
        .map(|x| x.parse().unwrap())
        .filter(|x| set.contains(x))
        .count()
}

fn parse(input: &str) -> Vec<usize> {
    let mut set = HashSet::new();
    input
        .split_terminator('\n')
        .map(|line| count_matches(line, &mut set))
        .collect()
}

fn part1(matches: &[usize]) -> usize {
    matches
        .iter()
        .map(|m| {
            if *m == 0 {
                0
            } else {
                2usize.pow(*m as u32 - 1)
            }
        })
        .sum()
}

fn part2(matches: &[usize]) -> usize {
    let mut copies = vec![0; matches.len()];
    let mut total_cards = 0;
    for (i, &m) in matches.iter().enumerate() {
        let cur_copies = copies[i] + 1;
        total_cards += cur_copies;
        for i in (i + 1)..min(i + 1 + m, copies.len()) {
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
        assert_eq!(13, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(30, part2(&parse(TEST_INPUT)));
    }
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
