use utils::*;

type Parsed = Grid<u8, 2>;

fn parse(input: &str) -> Parsed {
    let width = input.find('\n').unwrap() as i64;
    let height = (input.len() as i64) / (width + 1);
    let mut grid = Grid::new(b'0', &[width, height]);

    for (row_index, row) in input.split_terminator('\n').enumerate() {
        for (col_index, &b) in row.as_bytes().iter().enumerate() {
            grid[[col_index as i64, row_index as i64]] = b
        }
    }
    grid
}

const NEIGHBORS: [[i64; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn part1(data: &Parsed) -> i64 {
    let mut grid_ids = Grid::new(-1, &data.get_dims());
    let mut id_sizes = vec![];
    let mut traversal_stack = vec![];

    for c in data.iter_coords() {
        if grid_ids[c] != -1 {
            continue;
        }
        let plant = data[c];
        traversal_stack.clear();
        traversal_stack.push(Coord::from(c));
        id_sizes.push(0);

        while let Some(c) = traversal_stack.pop() {
            if grid_ids.get(c) != Some(&-1) || data[c] != plant {
                continue;
            }
            grid_ids[c] = id_sizes.len() as i32 - 1;
            *id_sizes.last_mut().unwrap() += 1;
            for n in NEIGHBORS {
                traversal_stack.push(c + n);
            }
        }
    }
    let mut id_perims = vec![0; id_sizes.len()];
    for c in grid_ids.iter_coords() {
        let cur_id = grid_ids[c];
        for n in NEIGHBORS {
            if grid_ids.get(c + n) != Some(&cur_id) {
                id_perims[cur_id as usize] += 1;
            }
        }
    }

    id_sizes
        .iter()
        .zip(&id_perims)
        .fold(0, |acc, (s, p)| acc + s * p)
}

fn part2(data: &Parsed) -> i64 {
    let mut grid_ids = Grid::new(-1, &data.get_dims());
    let mut id_sizes = vec![];
    let mut traversal_stack = vec![];

    for col in 0..data.get_dims()[0] {
        for row in 0..data.get_dims()[1] {
            if grid_ids[[col, row]] != -1 {
                continue;
            }
            let plant = data[[col, row]];
            traversal_stack.clear();
            traversal_stack.push(Coord::from([col, row]));
            id_sizes.push(0);

            while let Some(c) = traversal_stack.pop() {
                if grid_ids.get(c) != Some(&-1) || data[c] != plant {
                    continue;
                }
                grid_ids[c] = id_sizes.len() as i32 - 1;
                *id_sizes.last_mut().unwrap() += 1;
                for n in NEIGHBORS {
                    traversal_stack.push(c + n);
                }
            }
        }
    }
    let mut id_sides = vec![0; id_sizes.len()];
    for col in 0..data.get_dims()[0] {
        let neighbors = [[-1, 0], [1, 0]];
        let mut last_side = [None; 2];
        for row in 0..data.get_dims()[1] {
            let c = Coord::from([col, row]);
            let cur_id = grid_ids[c];
            for (i, n) in neighbors.iter().enumerate() {
                if grid_ids.get(c + *n) != Some(&cur_id) {
                    if last_side[i] != Some(cur_id) {
                        id_sides[cur_id as usize] += 1;
                    }
                    last_side[i] = Some(cur_id);
                } else {
                    last_side[i] = None;
                }
            }
        }
    }

    for row in 0..data.get_dims()[1] {
        let neighbors = [[0, 1], [0, -1]];
        let mut last_side = [None; 2];
        for col in 0..data.get_dims()[0] {
            let c = Coord::from([col, row]);
            let cur_id = grid_ids[c];
            for (i, n) in neighbors.iter().enumerate() {
                if grid_ids.get(c + *n) != Some(&cur_id) {
                    if last_side[i] != Some(cur_id) {
                        id_sides[cur_id as usize] += 1;
                        if cur_id == 1 {}
                    }
                    last_side[i] = Some(cur_id);
                } else {
                    last_side[i] = None;
                }
            }
        }
    }

    id_sizes
        .iter()
        .zip(&id_sides)
        .fold(0, |acc, (s, p)| acc + s * p)
}

#[cfg(test)]
mod tests {
    const TEST_INPUT_1: &str = "\
AAAA
BBCD
BBCC
EEEC
";
    const TEST_INPUT_2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(140, part1(&parse(TEST_INPUT_1)));
        assert_eq!(772, part1(&parse(TEST_INPUT_2)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(80, part2(&parse(TEST_INPUT_1)));
    }
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}
