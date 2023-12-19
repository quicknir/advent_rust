use microbench::{self, Options};
use utils::*;

type PartRating = i16;
type DestIndex = u16;
const ACCEPT: DestIndex = DestIndex::MAX;
const REJECT: DestIndex = DestIndex::MAX - 1;

#[derive(Debug)]
struct Part {
    ratings: [PartRating; 4],
}

#[derive(Debug)]
struct Rule {
    threshold: PartRating,
    mult: i8,
    field: u8,
    dest: DestIndex,
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<DestIndex> {
        let part_val = part.ratings[self.field as usize];
        if part_val * (self.mult as PartRating) > self.threshold * (self.mult as PartRating) {
            Some(self.dest)
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
struct Workflow {
    rules: Vec<Rule>,
    fallback: DestIndex,
}

type Parsed = (Vec<Workflow>, Vec<Part>);

fn get_index<'a, 'b: 'a>(
    index_map: &mut HashMap<&'a str, DestIndex>,
    workflows: &mut Vec<Workflow>,
    name: &'b str,
) -> DestIndex {
    *index_map.entry(name).or_insert_with(|| {
        workflows.push(Workflow::default());
        workflows.len() as DestIndex - 1
    })
}

fn parse(input: &str) -> Parsed {
    let (workflow_text, part_text) = input.split_once("\n\n").unwrap();

    let mut workflows = vec![Workflow::default()];
    let mut index_map = HashMap::from([("in", 0), ("A", ACCEPT), ("R", REJECT)]);
    for line in workflow_text.split('\n') {
        let (workflow_name, rest) = line.split_once('{').unwrap();
        let workflow_index = get_index(&mut index_map, &mut workflows, workflow_name) as usize;

        let mut it = rest.strip_suffix("}").unwrap().split(',');
        loop {
            let cur_split = it.next().unwrap();
            match cur_split.split_once(':') {
                Some((disc, dest)) => {
                    let field = match disc.chars().next().unwrap() {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => unreachable!(),
                    };
                    let mult = if disc.chars().nth(1).unwrap() == '>' {
                        1
                    } else {
                        -1
                    };
                    let dest = get_index(&mut index_map, &mut workflows, dest);
                    let threshold = disc[2..].parse().unwrap();
                    workflows[workflow_index].rules.push(Rule {
                        field,
                        mult,
                        dest,
                        threshold,
                    });
                }
                None => {
                    workflows[workflow_index].fallback =
                        get_index(&mut index_map, &mut workflows, cur_split);
                    break;
                }
            }
        }
    }

    let parts = part_text
        .split_terminator('\n')
        .map(|line| {
            let mut it = line
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .split(',');
            let ratings = std::array::from_fn(|_| {
                it.next()
                    .unwrap()
                    .split_once('=')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap()
            });

            Part { ratings }
        })
        .to_vec();

    (workflows, parts)
}

fn part1(data: &Parsed) -> u64 {
    let (workflows, parts) = data;
    parts
        .iter()
        .map(|part| {
            let mut cur_index = 0;
            loop {
                let workflow = &workflows[cur_index];
                cur_index = workflow
                    .rules
                    .iter()
                    .find_map(|rule| rule.apply(part))
                    .unwrap_or(workflow.fallback) as usize;
                if cur_index == ACCEPT as usize {
                    return part.ratings.iter().sum::<i16>() as u64;
                } else if cur_index == REJECT as usize {
                    return 0;
                }
            }
        })
        .sum()
}

#[derive(Debug, Clone)]
struct PartRange {
    ranges: [[PartRating; 2]; 4],
}

fn apply_rule_to_range(
    rule: &Rule,
    mut part: PartRange,
) -> (Option<(DestIndex, PartRange)>, Option<PartRange>) {
    let r = part.ranges[rule.field as usize];

    if (rule.mult == 1 && r[0] > rule.threshold) || (rule.mult == -1 && r[1] < rule.threshold) {
        return (Some((rule.dest, part)), None)
    }
    if (rule.mult == 1 && r[1] <= rule.threshold) || (rule.mult == -1 && r[0] >= rule.threshold) {
        return (None, Some(part))
    }
    let mut sent_part = part.clone();
    if rule.mult == 1 {
        sent_part.ranges[rule.field as usize][0] = rule.threshold+1;
        part.ranges[rule.field as usize][1] = rule.threshold;
    }
    else {
        sent_part.ranges[rule.field as usize][1] = rule.threshold-1;
        part.ranges[rule.field as usize][0] = rule.threshold;
    }

    return (Some((rule.dest, sent_part)), Some(part))
}

fn handle_destination(
    processing: &mut Vec<(DestIndex, PartRange)>,
    elem: (DestIndex, PartRange),
) -> u64 {
    if elem.0 == ACCEPT {
        return elem.1.ranges.iter().fold(1, |acc, e| acc * (e[1] - e[0] + 1) as u64)
    }
    if elem.0 != REJECT {
        processing.push(elem);
    }
    0
}

fn part2(data: &Parsed) -> u64 {
    let (workflows, _) = data;
    let ranges = std::array::from_fn(|_| [1, 4000]);
    let mut processing_ranges = vec![(0, PartRange { ranges })];
    let mut accepted_ranges = 0;

    while let Some((index, range)) = processing_ranges.pop() {
        let mut maybe_range = Some(range);
        let workflow = &workflows[index as usize];
        for rule in &workflow.rules {
            let Some(range) = maybe_range else {
                break;
            };
            let (sent, stay) = apply_rule_to_range(rule, range);
            sent.map(|x| accepted_ranges += handle_destination(&mut processing_ranges, x));
            maybe_range = stay;
        }
        maybe_range.map(|x| {
            accepted_ranges += handle_destination(&mut processing_ranges, (workflow.fallback, x))
        });
    }
    accepted_ranges
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "";
    use crate::*;
    #[test]
    fn test_part1() {
        assert_eq!(0, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(0, part1(&parse(TEST_INPUT)));
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
