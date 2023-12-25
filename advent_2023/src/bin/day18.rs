use microbench::{self, Options};
use utils::*;

type Parsed<'a> = Vec<(char, i32, &'a str)>;

fn parse(input: &str) -> Parsed {
    input
        .split_terminator('\n')
        .map(|line| {
            let mut it = line.split(' ');
            let dir = it.next().unwrap().chars().next().unwrap();
            let dist = it.next().unwrap().parse().unwrap();
            let col = it.next().unwrap();
            let col = &col[2..(col.len() - 1)];
            (dir, dist, col)
        })
        .collect()
}

fn solve<I: Iterator<Item = (char, i32)>>(it: I) -> i64 {
    let mut length = 0;
    let mut area = 0;
    let mut y_coord = 0;
    for (c, len) in it {
        let mult = match c {
            'L' => 1,
            'R' => -1,
            'U' => {
                y_coord += len as i64;
                0
            }
            'D' => {
                y_coord -= len as i64;
                0
            }
            _ => unreachable!(),
        };
        area += mult * (len as i64) * y_coord;
        length += len as i64;
    }
    let area = area.abs();
    let i = area - (length / 2) + 1; // number of internal points
    i + length
}

fn part1(data: &Parsed) -> i64 {
    solve(data.iter().map(|&e| (e.0, e.1)))
}

fn part2(data: &Parsed) -> i64 {
    solve(data.iter().map(|e| {
        let len = i64::from_str_radix(&e.2[..e.2.len() - 1], 16).unwrap() as i32;
        let dir = match e.2.as_bytes().last().unwrap() {
            b'0' => 'R',
            b'1' => 'D',
            b'2' => 'L',
            b'3' => 'U',
            _ => unreachable!(),
        };
        (dir, len)
    }
    ))
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)   
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(62, part1(&parse(TEST_INPUT)));
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
    benchmark(&s);
}
