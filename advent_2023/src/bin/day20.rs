use microbench::{self, Options};
use std::collections::VecDeque;
use utils::*;

type IndexType = u16;

#[derive(Debug, Clone, Copy)]
enum PulseHeight {
    Low,
    High,
}

#[derive(Debug)]
struct Pulse {
    src: IndexType,
    dest: IndexType,
    height: PulseHeight,
}

#[derive(Debug, Clone)]
struct FlipFlopModule {
    is_on: bool,
}
#[derive(Debug, Clone)]
struct ConjunctionModule {
    memory: Vec<IndexType>,
}

#[derive(Debug, Clone)]
enum ModuleKind {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Broadcaster,
    Test,
}

#[derive(Debug, Clone)]
struct Module {
    outputs: Vec<IndexType>,
    last_pulse: PulseHeight,
    kind: ModuleKind,
}

type Parsed = Vec<Module>;

impl Default for Module {
    fn default() -> Self {
        Self {
            outputs: Default::default(),
            last_pulse: PulseHeight::Low,
            kind: ModuleKind::Test,
        }
    }
}

fn get_index<'a, 'b: 'a>(
    index_map: &mut HashMap<&'a str, IndexType>,
    modules: &mut Vec<Module>,
    name: &'b str,
) -> IndexType {
    *index_map.entry(name).or_insert_with(|| {
        modules.push(Module::default());
        modules.len() as IndexType - 1
    })
}

fn parse(input: &str) -> Parsed {
    let mut modules = vec![
        Module {
            kind: ModuleKind::Broadcaster,
            ..Default::default()
        },
        Module::default(),
    ];
    let mut index_map = HashMap::with_capacity(1000);
    index_map.extend([("broadcaster", 0), ("rx", 1)]);

    for line in input.split_terminator('\n') {
        let (first, last) = line.split_once(" -> ").unwrap();
        let (name, kind) = match line.chars().next().unwrap() {
            '%' => (
                &first[1..],
                ModuleKind::FlipFlop(FlipFlopModule { is_on: false }),
            ),
            '&' => (
                &first[1..],
                ModuleKind::Conjunction(ConjunctionModule {
                    memory: Default::default(),
                }),
            ),
            _ => ("broadcaster", ModuleKind::Broadcaster),
        };
        let cur_module_index = get_index(&mut index_map, &mut modules, name) as usize;
        let mut cur_module = std::mem::take(&mut modules[cur_module_index]);
        cur_module.kind = kind;
        cur_module.outputs.extend(
            last.split(", ")
                .map(|s| get_index(&mut index_map, &mut modules, s)),
        );
        modules[cur_module_index] = cur_module;
    }

    for input in 0..modules.len() {
        for output_index in 0..modules[input].outputs.len() {
            let output = modules[input].outputs[output_index];
            let ModuleKind::Conjunction(ref mut x) = modules[output as usize].kind else {
                continue;
            };
            x.memory.push(input as IndexType);
        }
    }
    modules
}

fn press_button<F: FnMut(&Pulse) -> bool>(
    modules: &mut Vec<Module>,
    pulses: &mut VecDeque<Pulse>,
    mut f: F,
) -> bool {
    pulses.extend(modules[0].outputs.iter().map(|&x| Pulse {
        src: 0,
        dest: x,
        height: PulseHeight::Low,
    }));
    while let Some(cur_pulse) = pulses.pop_front() {
        modules[cur_pulse.src as usize].last_pulse = cur_pulse.height;
        // call func
        if f(&cur_pulse) {
            return true;
        }

        // take to dodge borrow checker issues
        let mut cur_module = std::mem::take(&mut modules[cur_pulse.dest as usize]);
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
                if c.memory
                    .iter()
                    .all(|&x| matches!(modules[x as usize].last_pulse, PulseHeight::High))
                {
                    Some(PulseHeight::Low)
                } else {
                    Some(PulseHeight::High)
                }
            }
            ModuleKind::Broadcaster => unreachable!(),
            ModuleKind::Test => None,
        };
        maybe_height.map(|new_height| {
            for &o in &cur_module.outputs {
                pulses.push_back(Pulse {
                    src: cur_pulse.dest,
                    dest: o,
                    height: new_height,
                });
            }
        });
        // put back what we took
        modules[cur_pulse.dest as usize] = cur_module;
    }
    false
}

fn part1(data: &Parsed) -> u64 {
    let mut modules = data.to_owned();
    let mut pulses = VecDeque::<Pulse>::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut f = |cur_pulse: &Pulse| {
        if let PulseHeight::Low = cur_pulse.height {
            low_pulses += 1;
        } else {
            high_pulses += 1;
        }
        false
    };
    for _ in 0..1000 {
        press_button(&mut modules, &mut pulses, &mut f);
    }
    return (low_pulses + 1000) * high_pulses;
}

fn part2(data: &Parsed) -> u64 {
    let mut modules = data.to_owned();
    let mut pulses = VecDeque::<Pulse>::new();

    // we will assume that rx is fed by a single conjunctive input. Find all of the inputs of that input.
    // See how long it takes them each to send their first high; multiply to get rx's first low
    // rx is at index 1; convention established by parse
    let conj_index = modules.iter().position(|m| m.outputs.contains(&1)).unwrap() as u16;
    assert!(matches!(
        &modules[conj_index as usize].kind,
        ModuleKind::Conjunction(_)
    ));

    let mut conj_inputs = modules
        .iter()
        .enumerate()
        .filter(|(_, m)| m.outputs.contains(&conj_index))
        .map(|(i, _)| i as u16)
        .to_hashset();

    let mut prod = 1;
    let mut button_pushes = 0;

    loop {
        button_pushes += 1;
        let done = press_button(&mut modules, &mut pulses, |cur_pulse: &Pulse| {
            if matches!(cur_pulse.height, PulseHeight::High)
                && cur_pulse.dest == conj_index
                && conj_inputs.contains(&cur_pulse.src)
            {
                prod *= button_pushes;
                conj_inputs.remove(&cur_pulse.src);
            }
            conj_inputs.is_empty()
        });
        if done {
            return prod;
        }
    }
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

    use crate::*;
    #[test]
    fn test_part1() {
        let data = parse(TEST_INPUT1);
        assert_eq!(32000000, part1(&data));
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
