use microbench::{self, Options};
use utils::*;

type Parsed<'a> = Vec<GearRow<'a>>;

#[derive(Debug)]
struct GearRow<'a> {
    record: &'a [u8],
    contiguous: Vec<u8>,
}

fn line_to_row(line: &str) -> GearRow<'_> {
    let (record, contiguous) = line.split_once(' ').unwrap();
    let contiguous = contiguous.split(',').map(|x| x.parse().unwrap()).to_vec();
    GearRow {
        record: record.as_bytes(),
        contiguous,
    }
}

fn process_line_third(row: &GearRow<'_>) -> u64 {
    let mut soln = vec![1; row.record.len()];
    let mut default = 1;
    for cont_size in &row.contiguous {
        let cont_size = *cont_size as usize;
        let mut v = vec![false; row.record.len()];
        for record_index in 0..row.record.len() {
            let Some(maybe_chunk) = row
                .record
                .get((record_index + 1).wrapping_sub(cont_size)..(record_index + 1))
            else {
                continue;
            };

            if !maybe_chunk.iter().all(|&x| x != b'.') {
                continue;
            }

            if row
                .record
                .get(record_index.wrapping_sub(cont_size))
                == Some(&b'#')
                || row.record.get(record_index + 1) == Some(&b'#')
            {
                continue;
            }
            v[record_index] = true;
        }
        let mut new_soln = vec![1; row.record.len()];

        for i in 0..new_soln.len() {
            new_soln[i] = *new_soln.get(i.wrapping_sub(1)).unwrap_or(&0);
            if v[i] {
                new_soln[i] += soln.get(i.wrapping_sub(cont_size+1)).unwrap_or(&default);
            }
        }

        soln = new_soln;

        default = 0;
    }
    *soln.last().unwrap()
}

fn process_line_dp(row: &GearRow<'_>) -> u64 {
    let mut soln = Grid::new(0, &[row.record.len() as i64, row.contiguous.len() as i64]);

    for cont_index in 0..row.contiguous.len() {
        let cont_size = row.contiguous[cont_index] as usize;
        let cont_index = cont_index as i64;
        for record_index in 0..row.record.len() {
            let cur_spring = row.record[record_index];
            let signed_record = record_index as i64;

            if cur_spring != b'#' {
                soln[[signed_record, cont_index]] +=
                    *soln.get([signed_record - 1, cont_index]).unwrap_or(&0);
            }

            // Now we have to consider that the cont_index'th chunk could end here

            // Can't end here because the next spot is filled
            if row.record.get(record_index + 1) == Some(&b'#') {
                continue;
            }

            // Can't end here because we dont' have enough elements from the beginning
            let Some(maybe_cont) = row
                .record
                .get(((record_index + 1).wrapping_sub(cont_size))..(record_index + 1))
            else {
                continue;
            };

            // Can't end here because not all elements in the span can be filled
            if !maybe_cont.iter().all(|&x| x != b'.') {
                continue;
            }
            soln[[signed_record, cont_index]] += *soln
                .get([signed_record - (cont_size as i64) - 1, cont_index - 1])
                .unwrap_or(&1);
        }
    }
    for j in 0..soln.get_dims()[1] {
        for i in 0..soln.get_dims()[0] {
            print!("{} ", soln[[i, j]])
        }
        println!("");
    }
    println!("{:?}", soln);

    soln[[row.record.len() as i64 - 1, row.contiguous.len() as i64 - 1]]
}

fn process_line(row: &GearRow<'_>) -> u64 {
    let mut num_arrangements = 0;
    let mut search_stack = vec![(0, 0)];

    while let Some((mut record_index, mut cont_index)) = search_stack.pop() {
        let Some(next_group) = row.record[record_index..]
            .iter()
            .position(|x| matches!(x, b'#' | b'?'))
        else {
            continue;
        };
        record_index += next_group;
        if row.record[record_index] == b'?' && record_index < row.record.len() {
            search_stack.push((record_index + 1, cont_index));
        }
        let Some(should_group) = row
            .record
            .get(record_index..(record_index + row.contiguous[cont_index] as usize))
        else {
            continue;
        };
        if should_group.iter().any(|x| *x == b'.') {
            continue;
        }
        record_index += row.contiguous[cont_index] as usize;
        if row.record.get(record_index) == Some(&b'#') {
            continue; // ensure that the contiguous group is broken up by something potentially operational
        }
        cont_index += 1;
        if cont_index == row.contiguous.len() {
            if row.record[record_index..]
                .iter()
                .all(|x| *x == b'.' || *x == b'?')
            {
                num_arrangements += 1;
            }
        } else if record_index < row.record.len() {
            search_stack.push((record_index + 1, cont_index))
        }
    }

    num_arrangements
}

fn process_line2(row: &GearRow<'_>) -> u64 {
    let mut s = Vec::with_capacity(5 * row.record.len() + 4);
    s.extend_from_slice(row.record);
    for _ in 0..4 {
        s.push(b'?');
        s.extend_from_slice(row.record);
    }
    let contiguous = row
        .contiguous
        .iter()
        .copied()
        .cycle()
        .take(5 * row.contiguous.len())
        .collect();
    let row = GearRow {
        record: &s,
        contiguous,
    };
    process_line_third(&row)
}

fn parse(input: &str) -> Parsed {
    input
        .split_terminator('\n')
        .map(|line| line_to_row(line))
        .collect()
}

fn part1(data: &Parsed) -> u64 {
    data.iter().map(|row| process_line(row)).sum()
}

fn part2(data: &Parsed) -> u64 {
    data.iter().map(|row| process_line2(row)).sum()
}

#[cfg(test)]
mod tests {
    // const TEST_INPUT: &str = "";
    use crate::*;
    #[test]
    fn test_part1() {
        let line = "???.### 1,1,3";
        assert_eq!(1, process_line_third(&line_to_row(line)));
        assert_eq!(1, process_line2(&line_to_row(line)));

        let line = ".??..??...?##. 1,1,3";
        assert_eq!(4, process_line_third(&line_to_row(line)));
        assert_eq!(16384, process_line2(&line_to_row(line)));

        let line = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(1, process_line_third(&line_to_row(line)));
        assert_eq!(1, process_line2(&line_to_row(line)));

        let line = "????.#...#... 4,1,1";
        assert_eq!(1, process_line_third(&line_to_row(line)));
        assert_eq!(16, process_line2(&line_to_row(line)));

        let line = "????.######..#####. 1,6,5";
        assert_eq!(4, process_line_third(&line_to_row(line)));
        assert_eq!(2500, process_line2(&line_to_row(line)));

        let line = "?###???????? 3,2,1";
        // assert_eq!(10, process_line_third(&line_to_row(line)));
        assert_eq!(506250, process_line2(&line_to_row(line)));
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
