use core::num;
use std::mem::replace;

use microbench::{self, Options};
use utils::*;

type Parsed = (HashMap<char, Vec<Coord<2>>>, [i64; 2]);

fn parse(input: &str) -> Parsed {
    let width = input.find('\n').unwrap() as i64;
    let height = (input.len() as i64) / (width + 1);

    let mut antennas: HashMap<char, Vec<Coord<2>>> = HashMap::new();

    for (row_index, line) in input.split_terminator('\n').enumerate() {
        for (col_index, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_default()
                    .push(Coord::from([col_index as i64, row_index as i64]));
            }
        }
    }

    (antennas, [width, height])
}

fn coord_groups<'a>(
    antennas: &'a HashMap<char, Vec<Coord<2>>>,
) -> impl Iterator<Item = (Coord<2>, Coord<2>)> + 'a {
    antennas
        .values()
        .flat_map(|v| (0..v.len()).flat_map(move |i| ((i + 1)..v.len()).map(move |j| (v[i], v[j]))))
}

fn part1(data: &Parsed) -> i64 {
    let (antennas, dims) = data;
    let mut antinodes = Grid::new(false, dims);
    let mut num_antinodes = 0;
    for (first, second) in coord_groups(antennas) {
        let antinode1 = (first - second) + first;
        let antinode2 = (second - first) + second;

        for a in [antinode1, antinode2] {
            if let Some(x) = antinodes.get_mut(a) {
                if !replace(x, true) { 
                    num_antinodes += 1;
                }
            }
        }
    }
    num_antinodes
}

fn part2(data: &Parsed) -> i64 {
    let (antennas, dims) = data;
    let mut antinode = Grid::new(false, dims);
    let mut num_antinodes = 0;
    for (first, second) in coord_groups(antennas) {
        let dir = second - first;

        let mut index = 0;

        loop {
            let a = first + (dir * index);

            if let Some(x) = antinode.get_mut(a) {
                if *x == false {
                    num_antinodes += 1;
                }
                *x = true;
                index += 1;
                continue;
            }
            break;
        }

        let mut index = -1;
        loop {
            let a = first + (dir * index);
            if let Some(x) = antinode.get_mut(a) {
                if *x == false {
                    num_antinodes += 1;
                }
                *x = true;
                index -= 1;
                continue;
            }
            break;
        }
    }
    num_antinodes
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}
