use microbench::{self, Options};
use utils::*;

#[derive(Debug)]
struct Parsed {
    galaxies: Vec<[usize; 2]>,
    empty_rows: Vec<bool>,
    empty_cols: Vec<bool>,
}

fn parse(input: &str) -> Parsed {
    let width = input.chars().position(|x| x == '\n').unwrap();
    let height = input.len() / (width + 1);
    let mut empty_rows = vec![true; height];
    let mut empty_cols = vec![true; width];
    let mut galaxies = vec![];

    let (mut row, mut col) = (0, 0);

    for c in input.chars() {
        if c == '\n' {
            col = 0;
            row += 1;
            continue;
        }
        if c == '#' {
            empty_rows[row] = false;
            empty_cols[col] = false;
            galaxies.push([col, row]);
        }

        col += 1;
    }
    Parsed {
        galaxies,
        empty_rows,
        empty_cols,
    }
}

fn do_it<const MULT: usize>(data: &Parsed) -> usize {
    let mut sum_distances = 0;
    let galaxies_adjusted = data
        .galaxies
        .iter()
        .map(|&[col, row]| {
            [
                col + MULT * data.empty_cols[..col].iter().filter(|x| **x).count(),
                row + MULT * data.empty_rows[..row].iter().filter(|x| **x).count(),
            ]
        })
        .to_vec();

    for i in 0..galaxies_adjusted.len() {
        let g1 = galaxies_adjusted[i];
        for j in 0..i {
            let g2 = galaxies_adjusted[j];
            sum_distances += g1[0].abs_diff(g2[0]) + g1[1].abs_diff(g2[1]);
        }
    }
    sum_distances
}

fn part1(data: &Parsed) -> usize {
    do_it::<1>(data)
}

fn part2(data: &Parsed) -> usize {
    do_it::<999999>(data)
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(374, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(1030, do_it::<9>(&parse(TEST_INPUT)));
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
