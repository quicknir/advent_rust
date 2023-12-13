use microbench::{self, Options};
use utils::*;

#[derive(Debug, Clone, Copy)]
struct CharView<'a> {
    data: &'a str,
    width: usize,
    height: usize,
}

impl<'a> CharView<'a> {
    fn get_row(&self, row: usize) -> Option<&str> {
        if row >= self.height {
            return None;
        }
        let start = row * (self.width + 1);
        self.data.get(start..(start + self.width))
    }

    fn row_similarity<const TARGET: u8>(&self, row_idx1: usize, row_idx2: usize) -> Option<u8> {
        if row_idx1 >= self.height || row_idx2 >= self.height {
            return None;
        }
        let row1 = self.get_row(row_idx1).unwrap().as_bytes();
        let row2 = self.get_row(row_idx2).unwrap().as_bytes();
        let mut dissimilarity = 0;
        for col in 0..self.width {
            if unsafe { row1.get_unchecked(col) != row2.get_unchecked(col) } {
                dissimilarity += 1;
            }
            if dissimilarity > TARGET {
                return Some(TARGET + 1);
            }
        }
        Some(dissimilarity)
    }

    fn col_similarity<const TARGET: u8>(&self, col1: usize, col2: usize) -> Option<u8> {
        if col1 >= self.width || col2 >= self.width {
            return None;
        }
        let mut dissimilarity = 0;
        for row_index in 0..self.height {
            let row = self.get_row(row_index).unwrap();
            if row.as_bytes()[col1] != row.as_bytes()[col2] {
                dissimilarity += 1;
                if dissimilarity > TARGET {
                    return Some(TARGET + 1);
                }
            }
        }
        Some(dissimilarity)
    }
}

fn next_char_view(input: &str) -> (&str, CharView<'_>) {
    let width = input.find('\n').unwrap();
    let length = 1 + input.find("\n\n").unwrap_or(input.len() - 1);
    let height = length / (width + 1);

    (
        input.get((length + 1)..).unwrap_or(""),
        CharView {
            data: &input[..length],
            width,
            height,
        },
    )
}

fn reflection<const TARGET: u8>(
    range: usize,
    f: impl Fn(usize, usize) -> Option<u8>,
) -> Option<usize> {
    'outer: for start in 0..(range - 1) {
        let mut dissimilarity = 0;
        for dist in 0.. {
            let i = start.wrapping_sub(dist);
            let j = start + 1 + dist;
            match f(i, j) {
                None => break,
                Some(x) => dissimilarity += x,
            }
            if dissimilarity > TARGET {
                continue 'outer;
            }
        }
        if dissimilarity == TARGET {
            return Some(start + 1);
        }
    }
    None
}

fn vertical_reflection<const TARGET: u8>(cv: CharView) -> Option<usize> {
    reflection::<TARGET>(cv.width, |x, y| cv.col_similarity::<TARGET>(x, y))
}
fn horizontal_reflection<const TARGET: u8>(cv: CharView) -> Option<usize> {
    reflection::<TARGET>(cv.height, |x, y| cv.row_similarity::<TARGET>(x, y))
}

fn solve<const TARGET: u8>(input: &str) -> usize {
    let mut rest = input;
    let mut sum = 0;
    while rest != "" {
        let (r, cv) = next_char_view(rest);
        rest = r;
        if let Some(h) = horizontal_reflection::<TARGET>(cv) {
            sum += 100 * h;
            continue;
        }
        sum += vertical_reflection::<TARGET>(cv).unwrap();
    }
    sum
}

fn part1(input: &str) -> usize {
    solve::<0>(input)
}

fn part2(input: &str) -> usize {
    solve::<1>(input)
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    const TEST_INPUT2: &str = "\
...#.###...
###.##.##.#
#...#.##.#.
.####..####
##..###.#..
.#.#..#.#.#
#....####.#
#....####.#
.#.#..#.#.#
##..###.#..
.####..#.##
#...#.##.#.
###.##.##.#
...#.###...
...#.###...
";

    use crate::*;
    #[test]
    fn test_part1() {
        let (rest, first) = next_char_view(TEST_INPUT);
        let (rest, second) = next_char_view(rest);
        assert_eq!("", rest);

        assert_eq!(Some(5), vertical_reflection::<0>(first));
        assert_eq!(None, vertical_reflection::<0>(second));

        assert_eq!(None, horizontal_reflection::<0>(first));
        assert_eq!(Some(4), horizontal_reflection::<0>(second));
    }

    #[test]
    fn test_start_input() {
        let (rest, first) = next_char_view(TEST_INPUT2);
        assert_eq!("", rest);
        assert_eq!(Some(14), horizontal_reflection::<0>(first));
    }

    #[test]
    fn test_part2() {
        let (rest, first) = next_char_view(TEST_INPUT);
        let (_, second) = next_char_view(rest);


        assert_eq!(Some(3), horizontal_reflection::<1>(first));
        assert_eq!(None, vertical_reflection::<1>(first));

        assert_eq!(Some(1), horizontal_reflection::<1>(second));
        assert_eq!(None, vertical_reflection::<1>(second));
    }
}

fn benchmark(s: &str) {
    let options = Options::default();
    microbench::bench(&options, "part1", || {
        part1(&s);
    });
    microbench::bench(&options, "part2", || {
        part2(&s);
    });
}

fn main() {
    let s = read_aoc!();
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&s));
    benchmark(&s)
}
