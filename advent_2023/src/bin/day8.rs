use microbench::{self, Options};
use utils::*;

type Index = u16;

#[derive(Debug, Default)]
struct Node {
    neighbors: [Index; 2],
}

struct Parsed {
    dirs: Vec<u8>,
    nodes: Vec<Node>,
    start: Index,
    end: Index,
    starts: Vec<Index>,
    ends: Vec<Index>,
}

fn parse(input: &str) -> Parsed {
    let mut it = input.split_terminator('\n');
    let dirs = it
        .next()
        .unwrap()
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .to_vec();
    it.next();

    let mut name_lookup: HashMap<&str, Index> = HashMap::new();
    let mut nodes = vec![];
    let mut starts = vec![];
    let mut ends = vec![];

    for line in it {
        let source = *name_lookup.entry(&line[0..3]).or_insert_with(|| {
            nodes.push(Node::default());
            (nodes.len() - 1) as Index
        });
        let left = *name_lookup.entry(&line[7..10]).or_insert_with(|| {
            nodes.push(Node::default());
            (nodes.len() - 1) as Index
        });
        let right = *name_lookup.entry(&line[12..15]).or_insert_with(|| {
            nodes.push(Node::default());
            (nodes.len() - 1) as Index
        });
        nodes[source as usize].neighbors[0] = left;
        nodes[source as usize].neighbors[1] = right;
        match line.chars().nth(2).unwrap() {
            'A' => starts.push(source),
            'Z' => ends.push(source),
            _ => (),
        }
    }
    Parsed {
        dirs,
        nodes,
        // start: *name_lookup.get("AAA").unwrap(),
        // end: *name_lookup.get("ZZZ").unwrap(),
        start: *name_lookup.get("MSA").unwrap(),
        end: *name_lookup.get("XHZ").unwrap(),
        starts,
        ends,
    }
}

fn part1(data: &Parsed) -> u64 {
    let mut total_steps = 0;
    let mut dir_index = 0;
    let mut cur_index = data.start;
    while cur_index != data.end {
        cur_index = data.nodes[cur_index as usize].neighbors[data.dirs[dir_index] as usize];
        total_steps += 1;
        dir_index += 1;
        if dir_index == data.dirs.len() {
            dir_index = 0;
        }
    }
    total_steps
}

fn find_end(start: Index, nodes: &[Node], dirs: &[u8], ends: &[bool]) -> u64 {
    let mut total_steps: u64 = 0;
    let mut cur_index = start;
    let mut dir_index = 0;
    loop {
        if unsafe { *ends.get_unchecked(cur_index as usize) } {
            return total_steps;
        }
        cur_index = unsafe {
            *nodes
                .get_unchecked(cur_index as usize)
                .neighbors
                .get_unchecked(*dirs.get_unchecked(dir_index) as usize)
        };

        total_steps += 1;
        dir_index += 1;
        if dir_index == dirs.len() {
            dir_index = 0;
        }
    }
}

fn part2(data: &Parsed) -> u64 {
    let mut ends_bits = vec![false; data.nodes.len()];
    for e in &data.ends {
        ends_bits[*e as usize] = true;
    }
    (data.dirs.len() as u64)
        * data
            .starts
            .iter()
            .map(|s| find_end(*s, &data.nodes, &data.dirs, &ends_bits))
            .fold(1, |acc, e| acc * (e / data.dirs.len() as u64))
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
