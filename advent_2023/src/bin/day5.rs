use utils::*;

use std::{
    cmp::{max, min},
    ops::Range,
};

use microbench::{self, Options};

#[derive(Debug, Clone, Copy)]
struct MapRange {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
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
            ranges.push(MapRange {
                source_start,
                dest_start,
                length,
            });
        }
        ranges.sort_by_key(|x| x.source_start);
        (Map { ranges }, more)
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

    fn map_ranges(&self, source: &[Range<u64>], dest: &mut Vec<Range<u64>>) {
        dest.clear();
        let mut i = 0;
        for e in source {
            let mut src = e.clone();
            loop {
                let Some(map_range) = self.ranges.get(i) else {
                    dest.push(src);
                    break;
                };
                if src.end <= map_range.source_start {
                    dest.push(src);
                    break;
                }
                let range_end = map_range.source_start + map_range.length;
                if range_end <= src.start {
                    i += 1;
                    continue;
                }
                let [inter_start, inter_end] = [
                    max(src.start, map_range.source_start),
                    min(src.end, range_end),
                ];
                if inter_start > src.start {
                    dest.push(src.start..inter_start);
                }
                let mapped_start = map_range.dest_start + inter_start - map_range.source_start;
                dest.push(mapped_start..(mapped_start + inter_end - inter_start));
                if inter_end < src.end {
                    src.start = inter_end;
                    i += 1;
                    continue;
                }
                break;
            }
        }
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<Map>) {
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
        let (map, more) = Map::new(&mut lines);
        maps.push(map);
        if !more {
            break;
        }
    }
    (seeds, maps)
}

fn map_seed(seed: u64, maps: &Vec<Map>) -> u64 {
    maps.iter().fold(seed, |acc, e| {
        let x = e.map(acc);
        x
    })
}

fn part1(data: &(Vec<u64>, Vec<Map>)) -> u64 {
    let (seeds, maps) = data;
    seeds.iter().map(|s| map_seed(*s, maps)).min().unwrap()
}

fn seeds_to_ranges(seeds: &[u64]) -> Vec<Range<u64>> {
    seeds.chunks_exact(2).map(|x| x[0]..(x[0] + x[1])).to_vec()
}

fn part2(data: &(Vec<u64>, Vec<Map>)) -> u64 {
    let (seeds, maps) = data;
    let mut input = seeds_to_ranges(&seeds);
    input.sort_by_key(|x| x.start);
    let mut output = vec![];
    for map in maps {
        map.map_ranges(&input, &mut output);
        output.sort_by_key(|x| x.start);
        input.clear();
        input.push(output.first().unwrap().clone());
        for r in &output[1..] {
            let cur_last = input.last_mut().unwrap();
            if cur_last.end >= r.start {
                cur_last.end = max(cur_last.end, r.end)
            } else {
                input.push(r.clone());
            }
        }
    }
    output.first().unwrap().start
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
    #[test]
    fn test_map_range() {
        let (seeds, maps) = parse(TEST_INPUT);
        let mut input = seeds_to_ranges(&seeds);
        input.sort_by_key(|x| x.start);
        let mut output = vec![];
        maps[0].map_ranges(&input, &mut output);
        output.sort_by_key(|x| x.start);
        assert_eq!(vec![57..70, 81..95], output);
    }
    #[test]
    fn test_light_to_temp() {
        let (_seeds, maps) = parse(TEST_INPUT);
        let mut output = vec![];
        maps[4].map_ranges(&[77..78], &mut output);
        assert_eq!(vec![45..46], output);
    }
}

fn benchmark(s: &str) {
    let options = Options::default();
    microbench::bench(&options, "parsing", || {
        let data = parse(&s);
    });
    let data = parse(&s);
    microbench::bench(&options, "part1", || {
        part1(&data);
    });
    microbench::bench(&options, "part2", || {
        part2(&data);
    });
    part2(&data);
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
    benchmark(&s);
}
