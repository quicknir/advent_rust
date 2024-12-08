use utils::*;

type Parsed = (Grid<bool, 2>, Coord<2>);

fn parse(input: &str) -> Parsed {
    let width = input.find('\n').unwrap() as i64;
    let height = (input.len() as i64) / (width + 1);
    let mut grid = Grid::new(false, &[width, height]);
    let mut loc = None;

    for (row_index, row) in input.split_terminator('\n').enumerate() {
        for (col_index, ch) in row.chars().enumerate() {
            match ch {
                '#' => grid[[col_index as i64, row_index as i64]] = true,
                '^' => loc = Some(Coord::from([col_index as i64, row_index as i64])),
                _ => (),
            }
        }
    }

    (grid, loc.unwrap())
}

fn traverse_lab(start: Point<i64, 2>, occupied: &Grid<bool, 2>) -> impl Iterator<Item = (Coord<2>, Coord<2>)> {
    let mut cur_pos = start;
    let mut dir = Coord::from([0, -1]);
    std::iter::from_fn(move || {

        loop {
            let new_pos = cur_pos + dir;
            match occupied.get(new_pos) {
                Some(true) => dir = Coord::from([-1 * dir[1], dir[0]]),
                Some(false) => {
                    cur_pos = new_pos;
                    return Some((cur_pos, dir));
                }
                None => return None,
            }
        }
    })
}

fn part1(data: &Parsed) -> usize {
    let (occupied, start) = data;
    let mut visited = Grid::new(false, &occupied.get_dims());
    visited[*start] = true;

    traverse_lab(*start, occupied).for_each(|(c, _d)| visited[c] = true);

    visited.get_data().iter().filter(|x| **x).count()
}

fn part2(data: &mut Parsed) -> i64 {
    let (occupied, start) = data;
    let mut num_ways = 0;
    let dims = occupied.get_dims();
    for col in 0..dims[0] {
        for row in 0..dims[1] {
            if occupied[[col, row]] {
                continue;
            }
            occupied[[col, row]] = true;

            let mut visited = HashSet::new();

            for (c, d) in traverse_lab(*start, occupied) {
                if visited.contains(&(c, d)) {
                    num_ways += 1;
                    break;
                }
                visited.insert((c, d));
            }

            occupied[[col, row]] = false;
        }
    }
    num_ways
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(41, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(6, part2(&mut parse(TEST_INPUT)));
    }
}

fn main() {
    let s = read_aoc!();
    let mut data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&mut data));
}
