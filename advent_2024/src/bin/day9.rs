use std::{cmp::Reverse, collections::BinaryHeap};
use utils::*;

type Parsed = Vec<i64>;

fn parse(input: &str) -> Parsed {
    input[..(input.len() - 1)]
        .bytes()
        .map(|b| (b - 48) as i64)
        .collect()
}

fn reverse_blocks<'a>(data: &'a [i64]) -> impl Iterator<Item = usize> + 'a {
    (0..data.len())
        .rev()
        .step_by(2)
        .flat_map(|i| std::iter::repeat_n(i, data[i] as usize))
}

fn part1(data: &Parsed) -> i64 {
    let mut id_count = -1;
    let ids: Vec<_> = (0..data.len())
        .map(|x| {
            if x % 2 == 0 {
                id_count += 1;
                id_count
            } else {
                -1
            }
        })
        .collect();

    let mut forward_index = 0;
    let mut checksum = 0;
    let mut position_index = 0;
    let mut back_it = reverse_blocks(data);
    let mut back_index = data.len();

    'blub: loop {
        if forward_index % 2 == 0 && forward_index < back_index {
            for _ in 0..data[forward_index] {
                checksum += ids[forward_index] * position_index;
                position_index += 1;
            }
            forward_index += 1;
            continue;
        }
        for _ in 0..data[forward_index] {
            back_index = back_it.next().unwrap();
            if back_index < forward_index {
                break 'blub;
            }
            checksum += ids[back_index] * position_index;
            position_index += 1;
        }
        forward_index += 1;
    }
    checksum
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Dense {
    start: i64,
    num_blocks: i64,
}

fn part2(data: &Parsed) -> i64 {
    let mut files = vec![];
    let mut cur_pos = 0;
    let mut spaces: [BinaryHeap<Reverse<Dense>>; 9] = Default::default();
    for (i, &size) in data.iter().enumerate() {
        if size == 0 {
            continue;
        }
        let d = Dense {
            start: cur_pos,
            num_blocks: size,
        };
        cur_pos += size;
        if i % 2 == 0 {
            files.push(d);
        } else {
            spaces[size as usize - 1].push(Reverse(d));
        }
    }

    let mut checksum = 0;

    for (id, file) in files.iter().enumerate().rev() {
        let array_start = file.num_blocks as usize - 1;
        let found_space = spaces[array_start..]
            .iter_mut()
            .enumerate()
            .filter_map(|(i, h)| h.peek_mut().map(|p| (p.0.start, i)))
            .min();
        let Dense{mut start, num_blocks} = *file;

        if let Some((_, space_offset)) = found_space {
            println!("Array start {array_start}, space offset {space_offset}");
            let Reverse(space) = spaces[array_start+space_offset].pop().unwrap();
            start = space.start;
            if num_blocks < space.num_blocks {
                let leftover_space = Dense{start:start+num_blocks, num_blocks:space.num_blocks-num_blocks};
                spaces[leftover_space.num_blocks as usize - 1].push(Reverse(leftover_space));
            }
        }

        for pos in start..(start+num_blocks) {
            checksum += (id as i64) *pos;
        }

    }

    checksum
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "2333133121414131402\n";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(1928, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(2858, part2(&parse(TEST_INPUT)));
    }
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}
