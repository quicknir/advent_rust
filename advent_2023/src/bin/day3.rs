use utils::*;
use microbench::{self, Options};

#[derive(Debug, Clone, Copy)]
enum Entry {
    Nothing,
    Part,
    MaybeGear,
    Number(usize),
}

fn parse(input: &str) -> (Grid<Entry, 2>, Vec<i64>) {
    let width = input.find('\n').unwrap() as i64;
    let height = (input.len() as i64) / (width + 1);
    let mut grid = Grid::new(Entry::Nothing, &[width, height]);
    let mut values = vec![];
    let mut id_num = 0;
    for (row_index, mut row) in input.split_terminator('\n').enumerate() {
        let mut col_index = 0;
        while !row.is_empty() {
            if row.chars().next().unwrap().is_digit(10) {
                let non_digit = row.find(|x: char| !x.is_digit(10)).unwrap_or(row.len());
                values.push(row[..non_digit].parse().unwrap());
                row = &row[non_digit..];
                let non_digit = non_digit as i64;
                for i in col_index..(col_index + non_digit) {
                    grid[[i, row_index as i64]] = Entry::Number(id_num);
                }
                col_index += non_digit;
                id_num += 1;
                continue;
            }
            if row.starts_with('*') {
                grid[[col_index, row_index as i64]] = Entry::MaybeGear;
            } else if row.chars().next().unwrap() != '.' {
                grid[[col_index, row_index as i64]] = Entry::Part;
            }
            col_index += 1;
            row = &row[1..];
        }
    }
    (grid, values)
}

// In this order to simplify part 2; this order of neighbors results in non-decreasing id's of Numbers
const NEIGHBORS: [[i64; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

fn part1((grid, values): &(Grid<Entry, 2>, Vec<i64>)) -> i64 {
    let [width, height] = grid.get_dims();

    let mut numbers = HashSet::new();

    for row_index in 0..height {
        for col_index in 0..width {
            let c: Coord<2> = [col_index, row_index].into();
            if !matches!(grid[c], Entry::MaybeGear | Entry::Part) {
                continue;
            }
            for n in NEIGHBORS {
                if let Some(Entry::Number(id)) = grid.get(c + n) {
                    numbers.insert(*id);
                }
            }
        }
    }
    numbers.iter().map(|id| values[*id]).sum()
}

fn part2((grid, values): &(Grid<Entry, 2>, Vec<i64>)) -> i64 {
    let [width, height] = grid.get_dims();
    let mut numbers = Vec::with_capacity(NEIGHBORS.len());
    let mut result = 0;

    for row_index in 0..height {
        for col_index in 0..width {
            let c: Coord<2> = [col_index, row_index].into();
            if !matches!(grid[c], Entry::MaybeGear) {
                continue;
            }
            for n in NEIGHBORS {
                if let Some(Entry::Number(id)) = grid.get(c + n) {
                    if Some(id) != numbers.last() {
                        numbers.push(*id);
                    }
                }
            }
            if numbers.len() == 2 {
                result += values[numbers[0]] * values[numbers[1]];
            }
            numbers.clear();
        }
    }
    result
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    use crate::*;
    #[test]
    fn test_part1() {
        let result = part1(&parse(TEST_INPUT));
        assert_eq!(4361, result);
    }
    #[test]
    fn test_part2() {
        assert_eq!(467835, part2(&parse(TEST_INPUT)));
    }
}

fn benchmark(s: &str) {
    let options = Options::default();
    microbench::bench(&options, "part1", || {
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
