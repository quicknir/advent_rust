use utils::*;
use microbench::{self, Options};

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.split_terminator('\n');
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace();
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace();
    times
        .zip(distances)
        .map(|(x, y)| Race {
            time: x.parse().unwrap(),
            distance: y.parse().unwrap(),
        })
        .collect()
}

fn num_winners(r: Race) -> u64 {
    let pm = ((r.time * r.time - 4 * r.distance) as f64).sqrt() / 2.0;

    if r.time % 2 == 0 {
        1 + 2 * ((pm - 1.0).ceil() as u64)
    } else {
        2 * ((pm - 0.5).ceil()) as u64
    }
}

fn part1(data: &[Race]) -> u64 {
    data.iter().fold(1, |acc, r| acc * num_winners(*r))
}

fn combine(left: u64, right: u64) -> u64 {
    let right_digits = ((right as f64).log10()).floor() + 1.0;
    left * 10u64.pow(right_digits as u32) + right
}

fn part2(data: &[Race]) -> u64 {
    let time = data[1..]
        .iter()
        .fold(data[0].time, |acc, e| combine(acc, e.time));
    let distance = data[1..]
        .iter()
        .fold(data[0].distance, |acc, e| combine(acc, e.distance));
    num_winners(Race { time, distance })
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_num_winners() {
        assert_eq!(
            4,
            num_winners(Race {
                time: 7,
                distance: 9
            })
        );
        assert_eq!(
            8,
            num_winners(Race {
                time: 15,
                distance: 40
            })
        );
        assert_eq!(
            9,
            num_winners(Race {
                time: 30,
                distance: 200
            })
        );
    }
}

fn benchmark(s: &str) {
    let options = Options::default();
    microbench::bench(&options, "parsing", || {
        parse(&s);
    });
    // part1 seems too fast to bench on its own with default options
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
