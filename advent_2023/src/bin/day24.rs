use std::ptr::null_mut;

use microbench::{self, Options};
use utils::*;

type HailStone = ([i64; 3], [i64; 3]);
type Parsed = Vec<HailStone>;

fn make_triple(s: &str) -> [i64; 3] {
    let mut it = s.split(", ");
    std::array::from_fn(|_| it.next().unwrap().parse().unwrap())
}

fn parse(input: &str) -> Parsed {
    input
        .split_terminator('\n')
        .map(|line| {
            let (p_str, v_str) = line.split_once(" @ ").unwrap();
            (make_triple(p_str), make_triple(v_str))
        })
        .collect()
}

fn intersect_stones_2d(first: &HailStone, second: &HailStone) -> Option<[f64; 2]> {
    let m1 = (first.1[1] as f64) / (first.1[0] as f64);
    let m2 = (second.1[1] as f64) / (second.1[0] as f64);

    if m1 == m2 {
        return None;
    }

    // y2 - y1 + m1x1 - m2x2 / (m1 - m2)
    let x = ((second.0[1] - first.0[1]) as f64 + m1 * (first.0[0] as f64)
        - m2 * (second.0[0] as f64))
        / (m1 - m2);

    // before figuring out y, we first determine if this in the past
    if (x - first.0[0] as f64).signum() != (first.1[0] as f64).signum()
        || (x - second.0[0] as f64).signum() != (second.1[0] as f64).signum()
    {
        return None;
    }

    // y = y1 + m1(x-x1)
    let y = first.0[1] as f64 + m1 * (x - first.0[0] as f64);
    Some([x, y])
}

fn part1(data: &Parsed) -> u64 {
    let search_area = 200000000000000.0..=400000000000000.0;
    let mut num_intersections = 0;

    for i in 0..data.len() {
        for j in 0..i {
            if let Some([x, y]) = intersect_stones_2d(&data[i], &data[j]) {
                if search_area.contains(&x) && search_area.contains(&y) {
                    num_intersections += 1;
                }
            }
        }
    }

    num_intersections
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
        // intersect
        println!(
            "{:?}",
            intersect_stones_2d(&([19, 13, 30], [-2, 1, -2]), &([18, 19, 22], [-1, -1, -2]))
        );
        println!(
            "{:?}",
            intersect_stones_2d(&([19, 13, 30], [-2, 1, -2]), &([20, 25, 34], [-2, -2, -4]))
        );
        println!(
            "{:?}",
            intersect_stones_2d(&([19, 13, 30], [-2, 1, -2]), &([12, 31, 28], [-1, -2, -1]))
        );

        // past intersect
        println!(
            "Past {:?}",
            intersect_stones_2d(&([19, 13, 30], [-2, 1, -2]), &([20, 19, 15], [1, -5, -3]))
        );

        // parallel
        println!(
            "{:?}",
            intersect_stones_2d(&([18, 19, 22], [-1, -1, -2]), &([20, 25, 34], [-2, -2, -4]))
        );
        println!(
            "{:?}",
            intersect_stones_2d(&([18, 19, 22], [-1, -3, -2]), &([20, 25, 34], [-2, -6, -4]))
        );
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
        part1(&data)
    });
    // microbench::bench(&options, "part2", || {
    //     part2(&data);
    // });
    // microbench::bench(&options, "combined", || {
    //     let data = parse(&s);
    //     part1(&data);
    //     part2(&data);
    // });
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
    benchmark(&s);
}
