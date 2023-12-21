use microbench::{self, Options};
use std::collections::VecDeque;
use utils::*;

type Parsed = (HashMap<String, Module>, Vec<String>);
type IndexType = u16;

#[derive(Debug, Clone, Copy)]
enum PulseHeight {
    Low,
    High,
}

#[derive(Debug)]
struct Pulse {
    src: String,
    dest: String,
    height: PulseHeight,
}

#[derive(Debug, Clone)]
struct FlipFlopModule {
    is_on: bool,
}
#[derive(Debug, Clone)]
struct ConjunctionModule {
    memory: HashMap<String, PulseHeight>,
}

#[derive(Debug, Clone)]
enum ModuleKind {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
}

#[derive(Debug, Clone)]
struct Module {
    outputs: Vec<String>,
    kind: ModuleKind,
}

fn parse(input: &str) -> (HashMap<String, Module>, Vec<String>) {
    let mut broadcaster = vec![];
    let mut modules = HashMap::new();

    for line in input.split_terminator('\n') {
        let (first, last) = line.split_once(" -> ").unwrap();
        let outputs = last.split(", ").map(|s| s.to_string()).to_vec();
        match line.chars().next().unwrap() {
            '%' => {
                modules.insert(
                    first[1..].to_string(),
                    Module {
                        outputs,
                        kind: ModuleKind::FlipFlop(FlipFlopModule { is_on: false }),
                    },
                );
            }
            '&' => {
                modules.insert(
                    first[1..].to_string(),
                    Module {
                        outputs,
                        kind: ModuleKind::Conjunction(ConjunctionModule {
                            memory: Default::default(),
                        }),
                    },
                );
            }
            _ => broadcaster = outputs,
        };
    }
    let keys = modules.keys().cloned().to_vec();

    for key in keys {
        let outputs = modules.get(&key).unwrap().outputs.clone();

        for output in outputs {
            // some outputs are not in the graph, "test" outputs
            let Some(x) = modules.get_mut(&output) else {
                continue;
            };
            let ModuleKind::Conjunction(ref mut x) = x.kind else {
                continue;
            };
            x.memory.insert(key.clone(), PulseHeight::Low);
        }
    }

    // println!("{:?}, {:?}", modules, broadcaster);

    (modules, broadcaster)
}

fn part1(data: &Parsed) -> u64 {
    let (mut modules, broadcaster) = (*data).clone();
    let mut pulses = VecDeque::<Pulse>::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        low_pulses += 1; // button
        pulses.extend(broadcaster.iter().map(|x| Pulse {
            src: "broadcaster".to_string(),
            dest: x.clone(),
            height: PulseHeight::Low,
        }));

        while let Some(cur_pulse) = pulses.pop_front() {
            if let PulseHeight::Low = cur_pulse.height {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            }
            // some modules not in map as they are output only
            let Some(cur_module) = modules.get_mut(&cur_pulse.dest) else {
                continue;
            };
            let maybe_height = match cur_module.kind {
                ModuleKind::FlipFlop(ref mut f) => {
                    if let PulseHeight::High = cur_pulse.height {
                        None
                    } else if f.is_on {
                        f.is_on = false;
                        Some(PulseHeight::Low)
                    } else {
                        f.is_on = true;
                        Some(PulseHeight::High)
                    }
                }
                ModuleKind::Conjunction(ref mut c) => {
                    *c.memory.get_mut(&cur_pulse.src).unwrap() = cur_pulse.height;
                    // println!("{:?}", c.memory);
                    if c.memory.values().all(|x| matches!(*x, PulseHeight::High)) {
                        Some(PulseHeight::Low)
                    } else {
                        Some(PulseHeight::High)
                    }
                }
            };
            maybe_height.map(|new_height| {
                for d in &cur_module.outputs {
                    pulses.push_back(Pulse {
                        src: cur_pulse.dest.clone(),
                        dest: d.clone(),
                        height: new_height,
                    });
                }
            });
        }
    }

    low_pulses * high_pulses
}

fn part2(data: &Parsed) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const TEST_INPUT2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    use crate::*;
    #[test]
    fn test_part1() {
        let data = parse(TEST_INPUT1);
        assert_eq!(32000000, part1(&data));
    }
    #[test]
    fn test_part2() {
        // assert_eq!(0, part1(&parse(TEST_INPUT)));
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
    // microbench::bench(&options, "part2", || {
    //     part2(&data);
    // });
    // microbench::bench(&options, "combined", || {
    //     let data = parse(&s);
    //     part1(&data);
    //     part2(&data);
    // });
}

fn main() {
    let s = read_aoc!();
    let data = parse(&s);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
    benchmark(&s);
}
