use utils::*;

fn parse(input: &str) -> () {

}

fn part1(data: &()) -> i64 {
    0
}

fn part2(data: &()) -> i64 {
    0
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
    let data = parse(&s)
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}
