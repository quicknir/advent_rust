use microbench::{self, Options};
use utils::*;

fn hash(s: &[u8]) -> u32 {
    s.iter().fold(0, |mut acc, e| {
        acc += *e as u32;
        acc *= 17;
        acc % 256
    })
}

fn part1(input: &str) -> u32 {
    input[..input.len()-1].split(',').map(|s| hash(s.as_bytes())).sum()
}

fn part2(input: &str) -> usize {
    let mut boxes: [Vec<(&str, u32)>; 256] = std::array::from_fn(|_| Vec::with_capacity(10));

    for s in input[..input.len() - 1].split(',') {
        if s.ends_with("-") {
            let label = &s[..s.len() - 1];
            let cur_box = &mut boxes[hash(label.as_bytes()) as usize];
            let found = cur_box.iter().position(|x| x.0 == label);
            found.map(|x| cur_box.remove(x));
            continue;
        }
        let (label, focal) = s.split_once('=').unwrap();
        let cur_box = &mut boxes[hash(label.as_bytes()) as usize];
        let focal = focal.parse().unwrap();
        if let Some(found) = cur_box.iter().position(|x| x.0 == label) {
            cur_box[found] = (label, focal);
        } else {
            cur_box.push((label, focal));
        }

    }
    let mut focusing_power = 0;
    for (box_num, cur_box) in boxes.iter().enumerate() {
        for (i, lens) in cur_box.iter().enumerate() {
            focusing_power += (box_num+1) * (i+1) * lens.1 as usize;
        }
    }
    focusing_power
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    use crate::*;
    #[test]
    fn test_hash() {
        let results = [30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];
        for (s, r) in TEST_INPUT.split(',').zip(results) {
            assert_eq!(r, hash(s.as_bytes()));
        }
    }
    #[test]
    fn test_part2() {
        assert_eq!(145, part2(TEST_INPUT));
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
    microbench::bench(&options, "combined", || {
        part1(&s);
        part2(&s);
    });
}

fn main() {
    let s = read_aoc!();
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&s));
    benchmark(&s);
}
