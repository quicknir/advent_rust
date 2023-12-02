use utils::*;

fn process_line1(line: &str) -> usize {
    let mut it = line.chars().filter_map(|x| x.to_digit(10));
    let first = it.next().unwrap();
    let last = it.last().unwrap_or(first);
    (10 * first + last) as usize
}

fn part1(input: &str) -> usize {
    input
        .split_terminator("\n")
        .map(|x| process_line1(x.as_ref()))
        .sum()
}

const WORD_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process_line2(mut line: &str) -> usize {
    let mut first = None;
    let mut last = None;
    while !line.is_empty() {
        if let Some(x) = line.chars().next().unwrap().to_digit(10) {
            let x = x as usize;
            first.get_or_insert(x);
            last = Some(x);
        } else if let Some(i) = WORD_DIGITS.iter().position(|x| line.starts_with(x)) {
            first.get_or_insert(i + 1);
            last = Some(i + 1);
        }
        line = &line[1..];
    }
    10 * first.unwrap() + last.unwrap()
}

fn part2(input: &str) -> usize {
    input
        .split_terminator("\n")
        .map(|x| process_line2(x.as_ref()))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_part1() {
        let test_input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        assert_eq!(part1(test_input), 142);
    }
    #[test]
    fn test_part2() {
        let test_input = "\
            two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen\n\
        ";
        assert_eq!(part2(test_input), 281);
    }
}

fn main() {
    let s = read_aoc!();
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&s));
}
