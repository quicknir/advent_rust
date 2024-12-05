use utils::*;

fn parse(input: &str) -> Grid<char, 2> {
    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);
    let mut grid = Grid::new('\0', &[width as i64, height as i64]);

    for (row, line) in input.split_terminator('\n').enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[[col as i64, row as i64]] = c;
        }
    }

    grid
}

const DIRS: [[i64; 2]; 8] = [
    [1, 0],
    [-1, 0],
    [0, 1],
    [0, -1],
    [1, 1],
    [1, -1],
    [-1, 1],
    [-1, -1],
];

fn part1(grid: &Grid<char, 2>) -> usize {
    let dims = grid.get_dims();
    (0..dims[0])
        .flat_map(|col| (0..dims[1]).map(move |row| Coord::from([col, row])))
        .flat_map(|c| DIRS.iter().map(move |&d| (c, Coord::from(d))))
        .filter(|&(coord, d)| {
            "XMAS"
                .char_indices()
                .all(|(i, ch)| grid.get(coord + d * (i as i64)) == Some(&ch))
        })
        .count()
}

fn is_ms(c: Option<&char>) -> bool {
    c == Some(&'M') || c == Some(&'S')
}
fn is_mas(c1: Option<&char>, c2: Option<&char>) -> bool {
    is_ms(c1) && is_ms(c2) && c1 != c2
}

fn part2(grid: &Grid<char, 2>) -> usize {
    let dims = grid.get_dims();
    (0..dims[0])
        .flat_map(|col| (0..dims[1]).map(move |row| Coord::from([col, row])))
        .filter(|&coord| {
            if grid[coord] != 'A' {
                return false;
            }
            is_mas(grid.get(coord + [-1, -1]), grid.get(coord + [1, 1]))
                && is_mas(grid.get(coord + [1, -1]), grid.get(coord + [-1, 1]))
        })
        .count()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(18, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(&parse(TEST_INPUT)));
    }
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}
