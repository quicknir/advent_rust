use utils::*;
use microbench::{self, Options};

type Parsed = ();

fn parse(input: &str) -> Parsed {

}

fn part1(data: &Parsed) -> i64 {
    0
}

fn part2(data: &Parsed) -> i64 {
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
}
