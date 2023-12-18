use std::{cmp::Reverse, collections::BinaryHeap};

use microbench::{self, Options};
use utils::*;

type Parsed = Grid<u8, 2>;

fn parse(input: &str) -> Parsed {
    let input = input.as_bytes();
    let width = input.iter().position(|&x| x == b'\n').unwrap() as i64;
    let height = input.len() as i64 / (width + 1);

    let mut grid = Grid::new(0, &[width, height]);
    let mut col = 0;
    let mut row = 0;

    for &b in input.iter() {
        if b == b'\n' {
            row += 1;
            col = 0;
            continue;
        }
        grid[[col, row]] = b - b'0';
        col += 1;
    }
    grid
}

fn part1(data: &Parsed) -> u32 {
    let mut heap = BinaryHeap::from([
        Reverse((data[[0, 1]] as u32, [0, 1], [0, 1], 0)),
        Reverse((data[[1, 0]] as u32, [1, 0], [1, 0], 0)),
    ]);
    let mut seen = HashSet::new();

    loop {
        let Reverse((heat_so_far, coord, dir, mom)) = heap.pop().unwrap();
        if coord == [data.get_dims()[0]-1, data.get_dims()[1]-1] {
            return heat_so_far;
        }
        if seen.contains(&(coord, dir, mom)) {
            continue;
        }
        seen.insert((coord, dir, mom));
        // same dir
        if mom <= 1 {
            let mut new_coord: Coord<2> = coord.into();
            new_coord += dir;
            if let Some(&added_heat) = data.get(new_coord) {
                heap.push(Reverse((
                    heat_so_far + added_heat as u32,
                    *new_coord.as_ref(),
                    dir,
                    mom + 1,
                )));
            }
        }
        // rotations
        let left_dir = [dir[1], -dir[0]];
        let right_dir = [-dir[1], dir[0]];
        for new_dir in [left_dir, right_dir] {
            let mut new_coord: Coord<2> = coord.into();
            new_coord += new_dir;
            if let Some(&added_heat) = data.get(new_coord) {
                heap.push(Reverse((
                    heat_so_far + added_heat as u32,
                    *new_coord.as_ref(),
                    new_dir,
                    0,
                )));
            }
        }
    }
}

fn part2(data: &Parsed) -> u32 {
    let mut heap = BinaryHeap::from([
        Reverse((data[[0, 1]] as u32, [0, 1], [0, 1], 0)),
        Reverse((data[[1, 0]] as u32, [1, 0], [1, 0], 0)),
    ]);
    let mut seen = HashSet::new();

    loop {
        let Reverse((heat_so_far, coord, dir, mom)) = heap.pop().unwrap();
        if coord == [data.get_dims()[0]-1, data.get_dims()[1]-1] {
            return heat_so_far;
        }
        if seen.contains(&(coord, dir, mom)) {
            continue;
        }
        seen.insert((coord, dir, mom));
        // same dir
        if mom <= 8 {
            let mut new_coord: Coord<2> = coord.into();
            new_coord += dir;
            if let Some(&added_heat) = data.get(new_coord) {
                heap.push(Reverse((
                    heat_so_far + added_heat as u32,
                    *new_coord.as_ref(),
                    dir,
                    mom + 1,
                )));
            }
            if mom <= 2 {
                continue;
            }
        }
        // rotations
        let left_dir = [dir[1], -dir[0]];
        let right_dir = [-dir[1], dir[0]];
        for new_dir in [left_dir, right_dir] {
            let mut new_coord: Coord<2> = coord.into();
            new_coord += new_dir;
            if let Some(&added_heat) = data.get(new_coord) {
                heap.push(Reverse((
                    heat_so_far + added_heat as u32,
                    *new_coord.as_ref(),
                    new_dir,
                    0,
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(102, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(94, part2(&parse(TEST_INPUT)));
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
