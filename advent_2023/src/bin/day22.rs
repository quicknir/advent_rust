use std::cmp::max;

use microbench::{self, Options};
use utils::*;

type Parsed = Vec<(Coord<3>, Coord<3>)>;

fn make_coord(s: &str) -> Coord<3> {
    let mut it = s.split(',');
    std::array::from_fn(|_| it.next().unwrap().parse().unwrap()).into()
}

fn parse(input: &str) -> Parsed {
    let mut bricks = input
        .split_terminator('\n')
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            (make_coord(start), make_coord(end))
        })
        .to_vec();

    bricks.sort_unstable_by_key(|x| x.1[2]);
    bricks
}

fn start_to_end(start: Coord<2>, end: Coord<2>) -> impl Iterator<Item = Coord<2>> + Clone {
    let delta = end - start;
    let magnitude = delta.as_ref().iter().map(|x| x.abs()).sum::<i64>();
    let unit: Coord<2> = if magnitude != 0 {
        std::array::from_fn(|i| delta[i] / magnitude).into()
    } else {
        [0, 0].into()
    };
    (0..(magnitude + 1)).map(move |i| start + [unit[0] * i, unit[1] * i])
}

trait Solver {
    fn accept_supports<I: Iterator<Item=i16>>(&mut self, cur_id: usize, first_support: i16, all_supports: I);
}

fn solve(bricks: &Parsed, s: &mut impl Solver) {
    let max_coords: [i64; 2] =
        std::array::from_fn(|i| bricks.iter().map(|b| max(b.0[i], b.1[i])).max().unwrap() + 1);

    let mut z_free = Grid::new((-1i16, 1), &max_coords);

    for (id, b) in bricks.iter().enumerate() {
        let it = start_to_end([b.0[0], b.0[1]].into(), [b.1[0], b.1[1]].into());
        let (largest, first_support) = it
            .clone()
            .map(|h| (z_free[h].1, z_free[h].0))
            .max()
            .unwrap();
        let new_z = largest + (b.0[2] - b.1[2]).abs() + 1;
        let support_ids = it.filter_map(|h| {
            let z_old = z_free[h];
            z_free[h] = (id as i16, new_z);
            (z_old.1 == largest).then_some(z_old.0)
        });
        s.accept_supports(id, first_support, support_ids);
    }
}

struct Part1Solver {
    safe_to_disintegrate: Vec<bool>,
}

impl Solver for Part1Solver {
    fn accept_supports<I: Iterator<Item=i16>>(&mut self, _cur_id: usize, first_support: i16, all_supports: I) {
        let mut one_support = true;

        for support in all_supports {
            if support != first_support {
                one_support = false;
            }
        }

        if one_support && first_support != -1 {
            self.safe_to_disintegrate[first_support as usize] = false;
        }
    }
}

fn part1(bricks: &Parsed) -> usize {
    let mut solver = Part1Solver{safe_to_disintegrate: vec![true; bricks.len()]};
    solve(&bricks, &mut solver);

    solver.safe_to_disintegrate.iter().filter(|x| **x).count()
}

fn lca(parents: &[i16], mut first: i16, mut second: i16) -> i16 {
    loop {
        if first == -1 || second == -1 {
            return -1;
        }
        if first == second {
            return first;
        } else if first < second {
            second = parents[second as usize];
        } else {
            first = parents[first as usize];
        }
    }
}

struct Part2Solver {
    parents: Vec<i16>,
}

impl Solver for Part2Solver {
    fn accept_supports<I: Iterator<Item=i16>>(&mut self, cur_id: usize, first_support: i16, all_supports: I) {
        let mut parent = first_support;
        for support in all_supports {
            parent = lca(&self.parents, parent, support);
        }
        self.parents[cur_id] = parent;
    }
}

fn part2(bricks: &Parsed) -> usize {
    let mut solver = Part2Solver{parents: vec![-1i16; bricks.len()]};
    solve(&bricks, &mut solver);

    let mut num_fallen = vec![0; bricks.len()];
    let mut sum_fallen = 0;
    for i in (0..bricks.len()).rev() {
        if solver.parents[i] != -1 {
            num_fallen[solver.parents[i] as usize] += 1 + num_fallen[i];
        }
        sum_fallen += num_fallen[i];
    }

    sum_fallen
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(5, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(7, part2(&parse(TEST_INPUT)));
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
