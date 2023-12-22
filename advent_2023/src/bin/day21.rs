use microbench::{self, Options};
use utils::*;
use std::collections::VecDeque;

type Parsed = (Grid<bool, 2>, Coord<2>);

fn parse(input: &str) -> Parsed {
    let width = input.find('\n').unwrap() as i64;
    let height = (input.len() as i64) / (width + 1);
    let mut grid = Grid::new(false, &[width, height]);
    let mut start = None;

    for (row_index, mut row) in input.split_terminator('\n').enumerate() {
        for (col, mut c) in row.chars().enumerate() {
            match c {
                'S' => start = Some([col as i64, row_index as i64]),
                '#' => grid[[col as i64, row_index as i64]] = true,
                _ => (),
            }
        }
    }

    (grid, start.unwrap().into())
}

const NEIGHBORS: [[i64; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn part1(data: &Parsed, max_steps: u32) -> usize {
    let (is_stone, start) = data;
    let mut visited = Grid::new(3u8, &is_stone.get_dims());
    let mut stack = VecDeque::from([(start.to_owned(), 0)]);

    while let Some((coord, steps)) = stack.pop_front() {
        if visited.get(coord) != Some(&3) || is_stone[coord] || steps > max_steps {
            continue;
        }
        visited[coord] = (steps % 2) as u8;
        if visited[coord] == 0 {
        }
        stack.extend(NEIGHBORS.iter().map(|x| (coord + *x, steps + 1)));
    }

    // for row in 0..visited.get_dims()[1] {
    //     let s: String = (0..visited.get_dims()[0]).map(|col| 
    //         if is_stone[[col, row]] {
    //             '#'
    //         }
    //         else if visited[[col, row]] == 0 {
    //             'O'
    //         }
    //         else if visited[[col, row]] == 3 {
    //             '!'
    //         }
    //         else {
    //             '.'
    //         }
    //     ).collect();
    //     println!("{}", s);

    // }


    visited.get_data().iter().filter(|x| **x == 0).count()
}

fn part2(data: &Parsed) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
 ...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........   
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(16, part1(&parse(TEST_INPUT), 6));
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(0, part1(&parse(TEST_INPUT)));
    // }
}

fn benchmark(s: &str) {
    let options = Options::default();
    microbench::bench(&options, "parsing", || {
        parse(&s);
    });
    let data = parse(&s);
    microbench::bench(&options, "part1", || {
        part1(&data, 64);
    });
    // microbench::bench(&options, "part2", || {
    //     part2(&data);
    // });
    // microbench::bench(&options, "combined", || {
    //     let data = parse(&s);
    //     part1(&data, 64);
    //     part2(&data);
    // });
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data, 64));
    println!("{:?}", part2(&data));
    benchmark(&s);
}
