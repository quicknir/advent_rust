use microbench::{self, Options};
use utils::*;

type Num = i32;

fn parse(input: &str) -> Vec<Vec<Num>> {
    input
        .split_terminator('\n')
        .map(|line| line.split_whitespace().map(|x| x.parse().unwrap()).to_vec())
        .to_vec()
}

fn process_sequence<const LAST: bool>(x: &[Num]) -> Num {
    let mut v = x.to_vec();
    let mut sum = 0;
    let mut mult = 1;

    loop {
        if LAST {
            sum += v.last().unwrap();
        } else {
            sum += v.first().unwrap() * mult;
            mult *= -1;
        }
        let mut all_zero = true;
        for i in 0..(v.len() - 1) {
            let diff = v[i + 1] - v[i];
            if diff != 0 {
                all_zero = false;
            }
            v[i] = diff;
        }
        v.pop().unwrap();
        if all_zero {
            break;
        }
    }
    sum
}

fn part1(data: &[Vec<Num>]) -> Num {
    data.iter().map(|seq| process_sequence::<true>(&seq)).sum()
}

fn part2(data: &[Vec<Num>]) -> Num {
    data.iter().map(|seq| process_sequence::<false>(&seq)).sum()
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
