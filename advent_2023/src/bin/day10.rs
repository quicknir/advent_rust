use microbench::{self, Options};
use utils::*;

#[derive(Debug, Default, Clone)]
struct Entry {
    neighbors: [[i8; 2]; 2],
}

impl Entry {
    fn from_dir(&self, in_dir: [i8; 2]) -> Option<[i8; 2]> {
        let in_dir_rev = [in_dir[0] * -1, in_dir[1] * -1];
        if self.neighbors[0] == in_dir_rev {
            Some(self.neighbors[1].clone())
        } else if self.neighbors[1] == in_dir_rev {
            Some(self.neighbors[0].clone())
        } else {
            None
        }
    }
}

type Parsed = (Grid<Entry, 2>, Coord<2>);

fn neighbors(c: char) -> [[i8; 2]; 2] {
    match c {
        '.' => [[0, 0], [0, 0]],
        '|' => [[0, 1], [0, -1]],
        '-' => [[1, 0], [-1, 0]],
        'L' => [[1, 0], [0, -1]],
        'J' => [[-1, 0], [0, -1]],
        '7' => [[-1, 0], [0, 1]],
        'F' => [[1, 0], [0, 1]],
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Parsed {
    let width = input.chars().position(|c| c == '\n').unwrap() as i64;
    let height = input.len() as i64 / (width + 1);
    let mut grid = Grid::new(Entry::default(), &[width, height]);
    let mut start = None;
    for (j, row) in input.split_terminator('\n').enumerate() {
        for (i, c) in row.chars().enumerate() {
            let coord = [i as i64, j as i64];
            if c == 'S' {
                start = Some(coord.into());
                continue;
            }
            grid[coord] = Entry {
                neighbors: neighbors(c),
            };
        }
    }
    (grid, start.unwrap())
}

struct LoopInfo {
    max_distance: u64,
    visited: Grid<bool, 2>,
}

fn traverse_loop(grid: &mut Grid<Entry, 2>, start: Coord<2>, mut dir: [i8; 2]) -> Option<LoopInfo> {
    let start_dir = dir;
    let mut cur = start;
    let mut distance = 0;
    let mut visited = Grid::new(false, &grid.get_dims());
    loop {
        distance += 1;
        visited.get_mut(cur).map(|x| *x = true);
        cur = [
            cur.as_ref()[0] + dir[0] as i64,
            cur.as_ref()[1] + dir[1] as i64,
        ]
        .into();
        if cur == start {
            grid[start] = Entry{neighbors: [start_dir, [dir[0]*-1, dir[1]*-1]]};
            return Some(LoopInfo {
                max_distance: distance / 2,
                visited,
            });
        }
        let Some(new_dir) = grid.get(cur).and_then(|x| x.from_dir(dir)) else {
            return None;
        };
        dir = new_dir;
    }
}

fn part1((grid, start): &mut Parsed) -> LoopInfo {
    let neighbors = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    neighbors
        .iter()
        .filter_map(|n| traverse_loop(grid, *start, *n))
        .next()
        .unwrap()
}

fn part2(grid: &Grid<Entry, 2>, visited: Grid<bool, 2>) -> u64 {
    let mut total_area = 0;

    for row in 0..visited.get_dims()[1] {
        let mut boundary_crosses = 0;
        let mut boundary_dir = None;
        for col in 0..visited.get_dims()[0] {
            if !visited[[col, row]] {
                if boundary_crosses % 2 == 1 {
                    total_area += 1;
                }
                continue
            }
            let entry = grid[[col, row]].clone();
            if entry.from_dir([0, 1]) == Some([0, 1]) {
                boundary_crosses += 1;
                continue
            }
            if let Some(d) = entry.from_dir([-1, 0]) {
                if d[1] != 0 {
                    boundary_dir = Some(d[1]);
                }
            }
            if let Some(d) = entry.from_dir([1, 0]) {
                if d[1] != 0 {
                    if d[1] != boundary_dir.unwrap() {
                        boundary_crosses += 1;
                    }
                    boundary_dir = None;
                }
            }
        }
    }

    total_area
}

#[cfg(test)]
mod tests {
    const TEST_INPUT_ONE: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const TEST_INPUT_TWO: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
    use crate::*;
    #[test]
    fn test_input_one() {
        let mut parsed = parse(&TEST_INPUT_ONE);
        let part1_result = part1(&mut parsed);
        assert_eq!(4, part2(&parsed.0, part1_result.visited));
    }
    #[test]
    fn test_input_two() {
        let mut parsed = parse(&TEST_INPUT_TWO);
        let part1_result = part1(&mut parsed);
        assert_eq!(8, part2(&parsed.0, part1_result.visited));
    }
}

fn benchmark(s: &str) {
    let options = Options::default();
    microbench::bench(&options, "parsing", || {
        parse(&s);
    });
    microbench::bench(&options, "combined", || {
        let mut data = parse(&s);
        let part1_result = part1(&mut data);
        part2(&data.0, part1_result.visited);
    });
}

fn main() {
    let s = read_aoc!();
    let mut data = parse(&s);
    let part1_result = part1(&mut data);
    println!("{:?}", part1_result.max_distance);
    println!("{:?}", part2(&data.0, part1_result.visited));
    benchmark(&s);
}
