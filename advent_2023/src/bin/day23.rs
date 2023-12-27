use microbench::{self, Options};
use std::collections::hash_map;
use utils::*;

#[derive(Debug, Clone, Copy)]
enum Entry {
    Path,
    Forest,
    Slope([i8; 2]),
}

type Parsed = Grid<Entry, 2>;

fn parse(input: &str) -> Parsed {
    let width = input.find('\n').unwrap() as i64;
    let height = input.len() as i64 / (width + 1);
    let mut grid = Grid::new(Entry::Forest, &[width, height]);

    for (row_index, row) in input.split_terminator('\n').enumerate() {
        for (col_index, c) in row.chars().enumerate() {
            let coord = [col_index as i64, row_index as i64];
            match c {
                '.' => grid[coord] = Entry::Path,
                '>' => grid[coord] = Entry::Slope([1, 0]),
                '<' => grid[coord] = Entry::Slope([-1, 0]),
                '^' => grid[coord] = Entry::Slope([0, -1]),
                'v' => grid[coord] = Entry::Slope([0, 1]),
                _ => (),
            }
        }
    }

    grid
}

fn part1(data: &Parsed) -> u64 {
    let mut stack: Vec<(Coord<2>, [i8; 2], HashSet<Coord<2>>, u64)> =
        vec![([1, 0].into(), [0, 1], Default::default(), 0)];
    let mut max_trip = 0;
    let exit: Coord<2> = {
        let e: Coord<2> = data.get_dims().into();
        e - [2, 1]
    };

    while let Some((mut pos, mut dir, seen_ints, mut dist)) = stack.pop() {
        while pos != exit {
            // forward left right
            let ds = [dir, [-dir[1], -dir[0]], [dir[1], dir[0]]];

            let continuations: [_; 3] = std::array::from_fn(|i| {
                let d = ds[i];
                let new_pos = pos + [d[0] as i64, d[1] as i64];
                match data[new_pos] {
                    Entry::Path => Some((new_pos, d, 1)),
                    Entry::Forest => return None,
                    Entry::Slope(slope_dir) => {
                        if slope_dir == [-1 * d[0], -1 * d[1]] {
                            None
                        } else {
                            Some((
                                new_pos + [slope_dir[0] as i64, slope_dir[1] as i64],
                                slope_dir,
                                2,
                            ))
                        }
                    }
                }
            });

            let first = continuations.iter().position(|x| x.is_some()).unwrap();

            for (new_pos, new_dir, added_dist) in
                continuations[(first + 1)..].iter().filter_map(|x| *x)
            {
                stack.push((new_pos, new_dir, Default::default(), dist + added_dist))
            }
            let x = continuations[first].unwrap();
            pos = x.0;
            dir = x.1;
            dist += x.2;
        }
        max_trip = std::cmp::max(max_trip, dist);
    }

    max_trip
}

#[derive(Default, Debug, Clone)]
struct Node {
    neighbors: [(u16, u16); 4],
    num_neighbors: u16,
    coord: Coord<2>,
}

impl Node {
    fn push(&mut self, index: u16, dist: u16) {
        self.neighbors[self.num_neighbors as usize] = (index, dist);
        self.num_neighbors += 1;
    }
}

fn make_graph<const SLOPES: bool>(data: &Parsed) -> Vec<Node> {
    let exit: Coord<2> = {
        let e: Coord<2> = data.get_dims().into();
        e - [2, 1]
    };
    let mut nodes = vec![Node::default(); 2];
    let mut node_map = HashMap::from([([1, 0].into(), 0), (exit, 1)]);
    let mut stack: Vec<(u16, Coord<2>, [i8; 2])> = vec![(0, [1, 0].into(), [0, 1])];

    while let Some((from_node, mut pos, mut dir)) = stack.pop() {
        let mut dist = 1;
        loop {
            if pos == exit {
                nodes[from_node as usize].push(1, dist);
                break;
            }
            if SLOPES {
                if let Entry::Slope(slope_dir) = data[pos] {
                    pos += [slope_dir[0] as i64, slope_dir[1] as i64];
                    dist += 1;
                    dir = slope_dir;
                    continue;
                }
            }
            // forward left right
            let ds = [dir, [-dir[1], -dir[0]], [dir[1], dir[0]]];

            let continuations: [_; 3] = std::array::from_fn(|i| {
                let d = ds[i];
                let new_pos = pos + [d[0] as i64, d[1] as i64];
                match data[new_pos] {
                    Entry::Path | Entry::Slope(_) => Some((new_pos, d, 1)),
                    Entry::Forest => None,
                }
            });

            let num_dirs = continuations.iter().filter(|x| x.is_some()).count();
            if num_dirs == 1 {
                // fast path out
                let cont = continuations.iter().filter_map(|x| *x).next().unwrap();
                pos = cont.0;
                dir = cont.1;
                dist += cont.2;
                continue;
            }

            match node_map.entry(pos) {
                hash_map::Entry::Occupied(occ) => {
                    // So we have already arrived at this node, we do not need to queue its children
                    let dest = *occ.get();

                    let n1 = &mut nodes[from_node as usize];
                    if !n1.neighbors.iter().any(|x| x.0 == dest) {
                        n1.push(dest, dist);
                        nodes[dest as usize].push(from_node, dist);
                    }
                }
                hash_map::Entry::Vacant(vac) => {
                    // Never arrived at node; insert, queue children
                    let dest = nodes.len() as u16;
                    vac.insert(dest);
                    let mut n2 = Node {
                        num_neighbors: 1,
                        coord: pos,
                        ..Default::default()
                    };
                    n2.neighbors[0] = (from_node, dist);
                    nodes.push(n2);
                    let n1 = &mut nodes[from_node as usize];
                    if !n1.neighbors.iter().any(|x| x.0 == dest) {
                        n1.push(dest, dist);
                    }

                    for c in continuations.iter().filter_map(|x| *x) {
                        stack.push((dest, c.0, c.1))
                    }
                }
            }
            break;
        }
    }
    nodes
}

fn max_graph_path(nodes: &[Node]) -> u64 {
    let mut max_path = 0;
    assert!(nodes.len() <= 64);
    let mut stack = vec![(0, 0, 0u64)];

    while let Some((node_index, dist, mut visited)) = stack.pop() {
        if (visited & (1 << node_index)) != 0 {
            continue;
        }
        visited |= 1 << node_index;
        if node_index == 1 {
            max_path = std::cmp::max(max_path, dist);
            continue;
        }
        let cur_node = &nodes[node_index as usize];
        for i in 0..cur_node.num_neighbors {
            let n = cur_node.neighbors[i as usize];
            if (visited & (1 << n.0 as usize)) == 0 {
                stack.push((n.0, n.1 as u64 + dist, visited));
            }
        }
    }

    max_path
}

fn part2(data: &Parsed) -> u64 {
    let d = make_graph::<false>(data);
    max_graph_path(&d) - 1
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#   
";
    use crate::*;
    #[test]
    fn test_part1() {
        let x = make_graph::<true>(&parse(TEST_INPUT));
        assert_eq!(94, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        let x = make_graph::<false>(&parse(TEST_INPUT));
        for n in x {
            println!("{:?}", n);
        }
        assert_eq!(154, part2(&parse(TEST_INPUT)));
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
