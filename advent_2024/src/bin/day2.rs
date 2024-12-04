use microbench::{self, Options};
use utils::*;
use std::iter::once;

fn parse(line: &str, storage: &mut Vec<i64>) {
    storage.clear();
    storage.extend(line.split(' ').map(|l| l.parse::<i64>().unwrap()));
}

fn adj_i(i: usize, skip_level: Option<usize>) -> usize {
    i + (skip_level.map_or(false, |s| s <= i) as usize)
}

fn are_levels_safe(levels: &[i64], skip_level: Option<usize>) -> bool {
    let mut dir = None;
    (0..levels.len()-1-(skip_level.is_some() as usize)).all(|i| {
        let diff = levels[adj_i(i+1, skip_level)] - levels[adj_i(i, skip_level)];
        (1..=3).contains(&diff.abs()) && dir.get_or_insert(diff.signum()) == &diff.signum()

    })
}

fn part1(input: &str) -> usize {
    let mut v = vec![];
    input
        .split_terminator('\n')
        .filter(|l| {
            parse(*l, &mut v);
            are_levels_safe(&v, None)
        })
        .count()
}

fn part2(input: &str) -> usize {
    let mut v = vec![];
    input
        .split_terminator('\n')
        .filter(|l| {
            parse(*l, &mut v);
            for skip in once(None).chain((0..v.len()).map(|x| Some(x))) {
                if are_levels_safe(&v, skip) {
                    return true;
                }
            }
            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "\
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}

fn main() {
    let s = read_aoc!();
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&s));
}
