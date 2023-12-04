use utils::*;

#[derive(Debug, Clone, Copy)]
enum Entry {
    Nothing,
    Part,
    MaybeGear,
    Digit(i8),
}

fn is_part(grid: &Grid<Entry, 2>, coord: [i64; 2]) -> bool {
    let Some(x) = grid.get(coord) else {
        return false;
    };
    match x {
        Entry::Part | Entry::MaybeGear => true,
        _ => false,
    }
}

fn as_digit(grid: &Grid<Entry, 2>, coord: [i64; 2]) -> i64 {
    if let Entry::Digit(d) = grid[coord] {
        d as i64
    } else {
        unreachable!()
    }
}

fn parse_grid(input: &str) -> Grid<Entry, 2> {
    let width = input.find('\n').unwrap() as i64;
    let height = (input.len() as i64) / (width + 1);
    let mut grid = Grid::new(Entry::Nothing, &[width, height]);
    for (row_index, row) in input.split_terminator('\n').enumerate() {
        for (col_index, c) in row.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                grid[[col_index as i64, row_index as i64]] = Entry::Digit(d as i8);
            } else if c == '*' {
                grid[[col_index as i64, row_index as i64]] = Entry::MaybeGear;
            } else if c != '.' {
                grid[[col_index as i64, row_index as i64]] = Entry::Part;
            }
        }
    }
    grid
}

fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);
    let [width, height] = grid.get_dims();

    let mut result = 0;

    for row_index in 0..height as i64 {
        let mut col_index = 0;
        while let Some(start) =
            (col_index..width).find(|i| matches!(grid[[*i, row_index]], Entry::Digit(_)))
        {
            let start = start as i64;
            let end = (start..width)
                .find(|i| !matches!(grid[[*i, row_index]], Entry::Digit(_)))
                .unwrap_or(width);
            col_index = end + 1;

            let part_adjacent = is_part(&grid, [start - 1, row_index])
                || is_part(&grid, [end, row_index])
                || (start - 1..end + 1).any(|i| {
                    is_part(&grid, [i, row_index - 1]) || is_part(&grid, [i, row_index + 1])
                });
            if part_adjacent {
                result += (start..end).fold(0, |acc, i| 10 * acc + as_digit(&grid, [i, row_index]))
            }
        }
    }
    result
}

fn part2(input: &str) -> i64 {
    let grid = parse_grid(input);
    let [width, height] = grid.get_dims();
    let neighbor_dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
        [1, 1],
        [1, -1],
        [-1, -1],
        [-1, 1],
    ];
    let mut result = 0;
    for row_index in 0..height {
        for col_index in 0..width {
            for n in neighbor_dirs {

            }
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
        let result = part1(TEST_INPUT);
        assert_eq!(4361, result);
    }
    #[test]
    fn test_part2() {
        let test_input = "";
    }
}

fn main() {
    let s = read_aoc!();
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&s));
}
