use microbench::{self, Options};
use utils::*;

#[derive(Debug, Clone, Copy)]
enum Entry {
    Empty,
    SplitterVert,
    SplitterHor,
    MirrorSlash,
    MirrorBackslash,
}

type Parsed = Grid<Entry, 2>;

fn parse(input: &str) -> Parsed {
    let input = input.as_bytes();
    let width = input.iter().position(|&x| x == b'\n').unwrap() as i64;
    let height = input.len() as i64 / (width + 1);

    let mut grid = Grid::new(Entry::Empty, &[width, height]);
    let mut col = 0;
    let mut row = 0;

    for &b in input.iter() {
        match b {
            b'\n' => {
                row += 1;
                col = 0;
                continue;
            }
            b'|' => grid[[col, row]] = Entry::SplitterVert,
            b'-' => grid[[col, row]] = Entry::SplitterHor,
            b'/' => grid[[col, row]] = Entry::MirrorSlash,
            b'\\' => grid[[col, row]] = Entry::MirrorBackslash,
            _ => (),
        }
        col += 1;
    }

    grid
}

fn num_energized(input: &Parsed, starting_coord: Coord<2>, starting_dir: Coord<2>) -> usize {
    let mut energized = Grid::new([false; 4], &input.get_dims());
    let mut beam_stack = vec![(starting_coord, starting_dir)];

    while let Some((mut beam_loc, mut beam_dir)) = beam_stack.pop() {
        loop {
            let Some(r) = energized.get_mut(beam_loc) else {
                break;
            };
            let dir_num = match beam_dir.as_ref() {
                [1, _] => 0,
                [-1, _] => 1,
                [_, 1] => 2,
                [_, -1] => 3,
                _ => unreachable!(),
            };
            if unsafe { *r.get_unchecked(dir_num) } {
                break;
            }
            unsafe { *r.get_unchecked_mut(dir_num) = true };
            match input[beam_loc] {
                Entry::Empty => (),
                Entry::SplitterVert => {
                    if beam_dir.as_ref()[0] != 0 {
                        beam_dir = [0, -1].into();
                        let beam_dir_split = [0, 1];
                        let split_loc = beam_loc + beam_dir_split;
                        beam_stack.push((split_loc, beam_dir_split.into()));
                    }
                }
                Entry::SplitterHor => {
                    if beam_dir.as_ref()[1] != 0 {
                        beam_dir = [-1, 0].into();
                        let beam_dir_split = [1, 0];
                        let split_loc = beam_loc + beam_dir_split;
                        beam_stack.push((split_loc, beam_dir_split.into()));
                    }
                }
                Entry::MirrorSlash => {
                    beam_dir = [-beam_dir.as_ref()[1], -beam_dir.as_ref()[0]].into();
                }
                Entry::MirrorBackslash => {
                    beam_dir = [beam_dir.as_ref()[1], beam_dir.as_ref()[0]].into();
                }
            }
            beam_loc += beam_dir;
        }
    }

    energized
        .get_data()
        .iter()
        .filter(|x| x.iter().any(|x| *x))
        .count()
}

fn part1(input: &Parsed) -> usize {
    num_energized(input, [0, 0].into(), [1, 0].into())
}

fn part2(data: &Parsed) -> usize {
    let [width, height] = data.get_dims();
    let left = (0..height).map(|row| ([0, row], [1, 0]));
    let right = (0..height).map(|row| ([width-1, row], [-1, 0]));
    let top = (0..width).map(|col| ([col, 0], [1, 0]));
    let bottom = (0..width).map(|col| ([col, height-1], [-1, 0]));

    left.chain(right)
        .chain(top)
        .chain(bottom)
        .map(|x| num_energized(data, x.0.into(), x.1.into()))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(46, part1(&parse(TEST_INPUT)));
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
