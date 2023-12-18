use microbench::{self, Options};
use utils::*;

type Parsed = Grid<Entry, 2>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Entry {
    SPACE,
    ROUND_ROCK,
    CUBE_ROCK,
}

fn print(grid: &Grid<Entry, 2>) {
    for row in 0..grid.get_dims()[1] {
        let s: String = (0..grid.get_dims()[0])
            .map(|col| match grid[[col, row]] {
                Entry::SPACE => '.',
                Entry::ROUND_ROCK => 'O',
                Entry::CUBE_ROCK => '#',
            })
            .collect();
        println!("{s}");
    }
}

fn parse(input: &str) -> Parsed {
    let width = input.find('\n').unwrap() as i64;
    let height = input.len() as i64 / (width + 1);
    let mut grid = Grid::new(Entry::SPACE, &[width, height]);
    let mut row = 0;
    let mut col = 0;

    let mut num_round = 0;

    for c in input.as_bytes() {
        match *c {
            b'\n' => {
                row += 1;
                col = 0;
                continue;
            }
            b'O' => {
                grid[[col, row]] = Entry::ROUND_ROCK;
                num_round += 1;
            }
            b'#' => {
                grid[[col, row]] = Entry::CUBE_ROCK;
            }
            b'.' => (),
            _ => unreachable!(),
        }
        col += 1;
    }
    grid
}

fn part1(input: &str) -> usize {
    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);

    let mut distances = vec![height; width];
    let mut row = 0;
    let mut col = 0;
    let mut load = 0;

    for c in input.as_bytes() {
        match *c {
            b'\n' => {
                row += 1;
                col = 0;
                continue;
            }
            b'O' => {
                load += distances[col];
                distances[col] -= 1;
            }
            b'#' => {
                distances[col] = height - row - 1;
            }
            b'.' => (),
            _ => unreachable!(),
        }
        col += 1;
    }
    load
}

// SHIFT_DIR: 1 means shifting towards 0, -1 means shifting towards higher coordinates
// AXIS: 0 
// WIP
// fn tilt<const SHIFT_DIR: i64, const AXIS: usize>(
//     grid: &mut Grid<Entry, 2>,
//     points: impl Iterator<Item = Coord<2>>,
//     dests: &mut Vec<i64>,
// ) {
//     for p in points {
//         match grid[p] {
//             Entry::SPACE => (),
//             Entry::CUBE_ROCK => dests[p.as_ref()[AXIS] as usize] = p.as_ref()[1-AXIS] + SHIFT_DIR,
//             Entry::ROUND_ROCK => {
//                 grid[p] = Entry::SPACE;
//                 let p2 = p;
//                 dests[p.as_ref()[AXIS] as usize] += SHIFT_DIR;
//             }
//         }
//     }
// }

fn tilt_north(grid: &mut Grid<Entry, 2>) {
    let mut dests = vec![0; grid.get_dims()[0] as usize];

    for row in 0..grid.get_dims()[1] {
        for col in 0..grid.get_dims()[0] {
            let col_u = col as usize;
            match grid[[col, row]] {
                Entry::SPACE => (),
                Entry::CUBE_ROCK => dests[col_u] = row + 1,
                Entry::ROUND_ROCK => {
                    grid[[col, row]] = Entry::SPACE;
                    grid[[col, dests[col_u]]] = Entry::ROUND_ROCK;
                    dests[col_u] += 1;
                }
            }
        }
    }
}

fn tilt_south(grid: &mut Grid<Entry, 2>) {
    let mut dests = vec![grid.get_dims()[1] - 1; grid.get_dims()[0] as usize];

    for row in (0..grid.get_dims()[1]).rev() {
        for col in 0..grid.get_dims()[0] {
            let col_u = col as usize;
            match grid[[col, row]] {
                Entry::SPACE => (),
                Entry::CUBE_ROCK => dests[col_u] = row - 1,
                Entry::ROUND_ROCK => {
                    grid[[col, row]] = Entry::SPACE;
                    grid[[col, dests[col_u]]] = Entry::ROUND_ROCK;
                    dests[col_u] -= 1;
                }
            }
        }
    }
}

fn tilt_west(grid: &mut Grid<Entry, 2>) {
    let mut dests = vec![0; grid.get_dims()[1] as usize];

    for col in 0..grid.get_dims()[0] {
        for row in 0..grid.get_dims()[1] {
            let row_u = row as usize;
            match grid[[col, row]] {
                Entry::SPACE => (),
                Entry::CUBE_ROCK => dests[row_u] = col + 1,
                Entry::ROUND_ROCK => {
                    grid[[col, row]] = Entry::SPACE;
                    grid[[dests[row_u], row]] = Entry::ROUND_ROCK;
                    dests[row_u] += 1;
                }
            }
        }
    }
}

fn tilt_east(grid: &mut Grid<Entry, 2>) {
    let mut dests = vec![grid.get_dims()[0] - 1; grid.get_dims()[1] as usize];

    for col in (0..grid.get_dims()[0]).rev() {
        for row in 0..grid.get_dims()[1] {
            let row_u = row as usize;
            match grid[[col, row]] {
                Entry::SPACE => (),
                Entry::CUBE_ROCK => dests[row_u] = col - 1,
                Entry::ROUND_ROCK => {
                    grid[[col, row]] = Entry::SPACE;
                    grid[[dests[row_u], row]] = Entry::ROUND_ROCK;
                    dests[row_u] -= 1;
                }
            }
        }
    }
}

fn part2(grid: &Parsed) -> i64 {
    let mut grid = grid.clone();
    let mut seen = HashMap::new();
    let mut cycle_counter = 0;
    let (cycle_start, cycle_size) = loop {
        // let mut bits = vec![0u8; (grid.get_data().len()/8) + 1];
        // for (i, e) in grid.get_data().iter().enumerate() {
        //     if let Entry::ROUND_ROCK = e {
        //         bits[i/8] |= 1 << (i%8);
        //     }
        // }
        // if let Some(past) = seen.get(&bits) {
        //     break (*past, cycle_counter - past);
        // }
        if let Some(past) = seen.get(grid.get_data()) {
            break (*past, cycle_counter - past);
        }

        seen.insert(grid.get_data().clone(), cycle_counter);
        // seen.insert(bits, cycle_counter);

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
        cycle_counter += 1;
    };
    let rem = (1000000000 - cycle_start) % cycle_size;
    let grid_data = seen
        .into_iter()
        .find(|x| x.1 == rem + cycle_start)
        .unwrap()
        .0;
    let final_grid = Grid::from_data(grid_data, &grid.get_dims());

    let mut total_load = 0;

    for row in 0..final_grid.get_dims()[1] {
        for col in 0..final_grid.get_dims()[0] {
            if let Entry::ROUND_ROCK = final_grid[[col, row]] {
                total_load += final_grid.get_dims()[1] - row;
            }
        }
    }
    total_load
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    const ONE_CYCLE: &str = "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
";

    const TWO_CYCLE: &str = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
";

    const THREE_CYCLE: &str = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
";

    use crate::*;
    #[test]
    fn test_part1() {
        let mut grid = parse(TEST_INPUT);
        // print(&grid);
        println!("\n tilted north \n");
        tilt_north(&mut grid);
        // print(&grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
        println!("\n one cycle \n");
        // print(&grid);
    }
    #[test]
    fn test_part2() {
        let cycles = [parse(ONE_CYCLE), parse(TWO_CYCLE), parse(THREE_CYCLE)];
        let mut grid = parse(TEST_INPUT);
        for soln in cycles.iter() {
            tilt_north(&mut grid);
            tilt_west(&mut grid);
            tilt_south(&mut grid);
            tilt_east(&mut grid);
            assert_eq!(soln, &grid);
        }
    }
    #[test]
    fn test_part2_2() {
        let data = parse(&TEST_INPUT);
        assert_eq!(64, part2(&data));
    }
}

fn benchmark(s: &str) {
    let options = Options::default();
    microbench::bench(&options, "parsing", || {
        parse(&s);
    });
    let data = parse(&s);
    microbench::bench(&options, "part1", || {
        part1(&s);
    });
    microbench::bench(&options, "part2", || {
        part2(&data);
    });
    microbench::bench(&options, "combined", || {
        let data = parse(&s);
        part1(&s);
        part2(&data);
    });
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&data));
    benchmark(&s);
}
