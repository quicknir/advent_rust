use rayon::prelude::*;
use utils::*;

#[derive(Debug)]
struct Range {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

#[derive(Debug)]
struct RangeMap {
    ranges: Vec<Range>,
}

impl RangeMap {
    fn new<'a>(it: &mut impl Iterator<Item = &'a str>) -> (Self, bool) {
        it.next().unwrap(); // skip first line
        let mut ranges = vec![];
        let mut more = false;
        while let Some(line) = it.next() {
            if line == "" {
                more = true;
                break;
            }
            let mut nums = line.split(' ');
            let [dest_start, source_start, length] =
                std::array::from_fn(|_| nums.next().unwrap().parse().unwrap());
            ranges.push(Range {
                source_start,
                dest_start,
                length,
            });
        }
        ranges.sort_by_key(|x| x.source_start);
        (RangeMap { ranges }, more)
    }
    fn map(&self, source: u64) -> u64 {
        let one_past_index = self
            .ranges
            .iter()
            .position(|x| x.source_start > source)
            .unwrap_or(self.ranges.len());
        let Some(range) = self.ranges.get(one_past_index.wrapping_sub(1)) else {
            return source;
        };
        let diff = source - range.source_start;
        if diff <= range.length {
            range.dest_start + diff
        } else {
            source
        }
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<RangeMap>) {
    let mut lines = input.split_terminator('\n');
    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    lines.next().unwrap();
    let mut maps = vec![];
    loop {
        let (map, more) = RangeMap::new(&mut lines);
        maps.push(map);
        if !more {
            break;
        }
    }
    (seeds, maps)
}

fn map_seed(seed: u64, maps: &Vec<RangeMap>) -> u64 {
    maps.iter().fold(seed, |acc, e| {
        let x = e.map(acc);
        // println!("{x}");
        x
    })
}

fn part1(data: &(Vec<u64>, Vec<RangeMap>)) -> u64 {
    let (seeds, maps) = data;
    seeds.iter().map(|s| map_seed(*s, maps)).min().unwrap()
}

fn part2(data: &(Vec<u64>, Vec<RangeMap>)) -> u64 {
    let (seeds, maps) = data;
    seeds
        .par_chunks_exact(2)
        .flat_map(|x| (x[0]..(x[0] + x[1])))
        .map(|s| map_seed(s, maps))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4   
";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(35, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(46, part2(&parse(TEST_INPUT)));
    }
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}
