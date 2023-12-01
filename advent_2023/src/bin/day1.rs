use utils::*;


fn process_line1(line: &str) -> usize {
    let mut it = line.chars().filter_map(|x| x.to_digit(10));
    let first = it.next().unwrap();
    let last = it.last().unwrap_or(first);
    (10 * first + last) as usize
}

fn part1(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    input.map(|x| process_line1(x.as_ref())).sum()
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
        }
        else if let Some(i) = WORD_DIGITS.iter().position(|x| line.starts_with(x)) {
            first.get_or_insert(i+1);
            last = Some(i+1);
            line = line.strip_prefix(WORD_DIGITS[i]).unwrap();
            continue;
        }
        line = &line[1..];
    }
    10 * first.unwrap() + last.unwrap()
}

fn part2(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    input.map(|x| process_line2(x.as_ref())).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_part1() {
        let test_input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(part1(test_input.iter()), 142);
    }
    #[test]
    fn test_part2() {
        let test_input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        assert_eq!(part2(test_input.iter()), 281);
    }
}

fn main() {
    println!("{:?}", part1(read_aoc_lines!()));
    println!("{:?}", part2(read_aoc_lines!()));
}
