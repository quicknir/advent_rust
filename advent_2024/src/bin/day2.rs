use microbench::{self, Options};
use utils::*;
use std::iter::once;

fn parse(line: &str, storage: &mut Vec<i64>) {
    storage.clear();
    storage.extend(line.split(' ').map(|l| l.parse::<i64>().unwrap()));
}

fn adj_i(i: usize, skip_level: Option<usize>) -> usize {
    i + ((skip_level == Some(i)) as usize)
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
            for skip in once(None).chain((1..v.len()).map(|x| Some(x))) {
                if are_levels_safe(&v, skip) {
                    return true;
                }
            }
            false
        })
        .count()
}


fn part_two_line(levels: &[i64]) -> bool {
    if levels.len() <= 3 {
        todo!()
    }
    let mut change_counts = [0; 3];
    for i in 0..(levels.len() - 1) {
        let index = (levels[i + 1] - levels[i]).signum() + 1;
        change_counts[index as usize] += 1
    }
    let defects = change_counts[1] + min(change_counts[0], change_counts[2]);
    if defects == 0 {
        return (0..(levels.len() - 1)).all(|i| (levels[i] - levels[i + 1]).abs() <= 3);
    }
    if defects > 1 {
        return false;
    }
    let defect_val = change_counts
        .iter()
        .enumerate()
        .find(|x| *x.1 == 1)
        .unwrap()
        .0 as i64;
    let defect_index = (0..levels.len() - 1)
        .find(|&i| (levels[i + 1] - levels[i]).signum() == (defect_val - 1))
        .unwrap();

    (0..(levels.len() - 2)).all(|i| {
        let first = i + ((i >= defect_index) as usize);
        let second = i + 1 + (((i + 1) >= defect_index) as usize);
        (levels[first] - levels[second]).abs() <= 3
    })
}

#[cfg(test)]
mod tests {
    use crate::*;
    // #[test]
    // fn test_part1() {
    //     assert_eq!(0, part1(&parse(TEST_INPUT)));
    // }
    #[test]
    fn test_part2() {
        assert_eq!(part_two_line(&[7, 6, 4, 2, 1]), true);
        assert_eq!(part_two_line(&[1, 2, 7, 8, 9]), false);
        assert_eq!(part_two_line(&[9, 7, 6, 2, 1]), false);
        assert_eq!(part_two_line(&[1, 3, 2, 4, 5]), true);
        assert_eq!(part_two_line(&[8, 6, 4, 4, 1]), true);
        assert_eq!(part_two_line(&[1, 3, 6, 7, 9]), true);
    }
}

fn main() {
    let s = read_aoc!();
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&s));
}
