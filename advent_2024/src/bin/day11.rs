use utils::*;
use microbench::{self, Options};

type Parsed = Vec<i64>;

fn with_cache(stone: i64, blinks: i16, cache: &mut HashMap<(i64, i16), i64>) -> i64 {
    if let Some(cached_result) = cache.get(&(stone, blinks)) {
        return *cached_result;
    }
    let result = compute_stones(stone, blinks, cache);
    cache.insert((stone, blinks), result);
    result
    // *cache.entry((stone, blinks)).or_insert_with(|| compute_stones(stone, blinks, cache))
}

fn compute_stones(stone: i64, blinks: i16, cache: &mut HashMap<(i64, i16), i64>) -> i64
{
    if blinks == 0 {
        return 1;
    }
    if stone == 0 {
        return with_cache(1, blinks-1, cache);
    }
    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
        return with_cache(stone / (10i64.pow(num_digits/2)), blinks-1, cache) + with_cache(stone % (10i64.pow(num_digits/2)), blinks-1, cache)
    }
    return with_cache(stone*2024, blinks-1, cache);
}

fn parse(input: &str) -> Parsed {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()

}

fn do_it(data: &Parsed, blinks: i16) -> i64 {
    let mut cache = HashMap::new();
    data.iter().fold(0, |acc, i| acc + compute_stones(*i, blinks, &mut cache))
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "125 17";
    use crate::*;
    #[test]
    fn test_part1() {
        let mut cache = HashMap::new();
        assert_eq!(1, compute_stones(125, 1, &mut cache));
        assert_eq!(2, compute_stones(125, 2, &mut cache));
        assert_eq!(4, do_it(&parse(TEST_INPUT), 2));
        assert_eq!(22, do_it(&parse(TEST_INPUT), 6));
        assert_eq!(55312, do_it(&parse(TEST_INPUT), 25));
    }
}

fn benchmark(s: &str) {
    let options = Options::default();
    microbench::bench(&options, "parsing", || {
        parse(&s);
    });
    let data = parse(&s);
    microbench::bench(&options, "part1", || {
        do_it(&data, 25);
    });
    microbench::bench(&options, "part2", || {
        do_it(&data, 75);
    });
}

fn main() {
    // let s = read_aoc!();
    let s = String::from("8793800 1629 65 5 960 0 138983 85629");
    let data = parse(&s);
    println!("{:?}", do_it(&data, 25));
    println!("{:?}", do_it(&data, 75));
    benchmark(&s);
}
