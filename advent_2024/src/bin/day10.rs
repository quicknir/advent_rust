use std::mem::replace;

use utils::*;

type Parsed = Grid<i8, 2>;

fn parse(input: &str) -> Parsed {
    let width = input.find('\n').unwrap() as i64;
    let length = (input.len() as i64) / (width+1);

    let mut grid = Grid::new(0i8, &[width, length]);

    for (col_index, row) in input.split_terminator('\n').enumerate() {
        for (row_index, b) in row.bytes().enumerate() {
            grid[[col_index as i64, row_index as i64]] = (b-48) as i8;
        } 
    }
    grid
}

const NEIGHBORS: [[i64; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn score_trailheads<F: FnMut(Coord<2>)->bool>(trailhead: Coord<2>, heights: &Grid<i8, 2>, mut f: F) -> i64 {
    if heights[trailhead] != 0 {
        return 0;
    }

    let mut v = vec![trailhead];
    let mut score = 0;

    while let Some(c) = v.pop() {
        if f(c) {
            continue;
        }

        let height = heights[c];
        if height == 9 {
            score += 1;
            continue;
        }

        for n in NEIGHBORS {
            let new_coord = c+n;
            if heights.get(new_coord) == Some(&(height+1)) {
                v.push(new_coord);
            }
        }
    }
    score
}

fn part1(data: &Parsed) -> i64 {
    let dims = data.get_dims();
    let mut total_score = 0;
    for i in 0..dims[0] {
        for j in 0..dims[1] {
            let mut visited = Grid::new(false, &data.get_dims());
            total_score += score_trailheads([i, j].into(), data, |c| {
                replace(&mut visited[c], true)
            });
        }
    }
    total_score
}

fn part2(data: &Parsed) -> i64 {
    let dims = data.get_dims();
    let mut total_score = 0;
    for i in 0..dims[0] {
        for j in 0..dims[1] {
            total_score += score_trailheads([i, j].into(), data, |_c| {
                false
            });
        }
    }
    total_score
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}
