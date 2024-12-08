use utils::*;

type Parsed = Vec<(i64, Vec<i64>)>;

fn could_be_true<const THIRD_OPERATOR: bool>(test_value: i64, numbers: &[i64]) -> bool {
    let mut v = vec![(numbers[0], 1)];

    while let Some((value, index)) = v.pop() {
        if index == numbers.len() {
            if value == test_value {
                return true;
            } else {
                continue;
            }
        }
        v.push((value + numbers[index], index + 1));
        v.push((value * numbers[index], index + 1));
        if THIRD_OPERATOR {
            let num_digits = ((numbers[index] as f64).log10().floor() as u32) + 1;
            v.push((value * 10i64.pow(num_digits) + numbers[index], index+1));
        }
    }

    false
}

fn parse(input: &str) -> Parsed {
    input
        .split_terminator('\n')
        .map(|line| {
            let (first, second) = line.split_once(": ").unwrap();
            (
                first.parse().unwrap(),
                second.split(' ').map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn do_problem<const THIRD_OPERATOR: bool>(data: &Parsed) -> i64 {
    data.iter().fold(0, |acc, (test_value, numbers)| {
        acc + if could_be_true::<THIRD_OPERATOR>(*test_value, numbers) {
            *test_value
        } else {
            0
        }
    })

}

fn part1(data: &Parsed) -> i64 {
    do_problem::<false>(data)
}

fn part2(data: &Parsed) -> i64 {
    do_problem::<true>(data)
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    use crate::*;
    #[test]
    fn test_could_be_true() {
        assert_eq!(true, could_be_true::<false>(190, &[10, 19]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(3749, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(11387, part2(&parse(TEST_INPUT)));
    }
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}
