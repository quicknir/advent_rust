use std::cmp::Ordering;

use utils::*;

type Parsed = (HashMap<(i64, i64), bool>, Vec<Vec<i64>>);

fn parse(input: &str) -> Parsed {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut rules = HashMap::new();
    for line in first.split('\n') {
        let (before, after) = line.split_once('|').unwrap();
        let (before, after) = (before.parse().unwrap(), after.parse().unwrap());
        rules.insert((before, after), true);
        rules.insert((after, before), false);
    }

    let updates = second.split_terminator('\n').map(|line| {
        line.split(',').map(|page| page.parse().unwrap()).collect()
    }).collect();

    (rules, updates)
}

fn is_update_correct(update: &[i64], rules: &HashMap<(i64, i64), bool>) -> bool {

    for (i, page) in update.iter().enumerate() {
        for earlier in &update[..i] {
            if rules.get(&(*page, *earlier)) == Some(&true) {
                return false;
            }
        }
    }
    true
}

fn part1(data: &Parsed) -> i64 {
    let (rules, updates) = data;

    updates.iter().fold(0, |acc, update| {
        acc + if is_update_correct(update, &rules) {
            update[update.len() / 2]
        }
        else {
            0
        }
    })
}

fn part2(data: &mut Parsed) -> i64 {
    let (rules, updates) = data;
    updates.iter_mut().fold(0, |acc, update| {
        if is_update_correct(update, &rules) {
            return acc;
        }
        loop {
            let mut made_change = false;

            for i in 0..(update.len()-1) {
                if rules.get(&(update[i+1], update[i])) == Some(&true) {
                    made_change = true;
                    update.swap(i, i+1);
                }
            }

            if !made_change {
                break;
            }
        }
        acc + update[update.len() / 2]
    })
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(0, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(0, part1(&parse(TEST_INPUT)));
    }
}

fn main() {
    let s = read_aoc!();
    let mut data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&mut data));
}
