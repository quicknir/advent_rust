use std::cmp::max;

use utils::*;

#[derive(Default, Debug)]
struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

impl Colors {
    fn new(s: &str) -> Self {
        let mut d = Colors::default();
        s.split(", ").for_each(|c| d.assign_color(c));
        d
    }
    fn assign_color(&mut self, s: &str) {
        let (num, col) = s.split_once(' ').unwrap();
        let num = num.parse().unwrap();
        match col {
            "blue" => self.blue = num,
            "green" => self.green = num,
            "red" => self.red = num,
            _ => panic!(),
        }
    }
    fn is_all_greater(&self, other: &Colors) -> bool {
        self.blue >= other.blue && self.green >= other.green && self.red >= other.red
    }
    fn max_of(&self, other: &Colors) -> Colors {
        Colors {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }
    fn power_of(&self) -> i64 {
        (self.blue * self.red * self.green) as i64
    }
}

const BAG: Colors = Colors {
    red: 12,
    green: 13,
    blue: 14,
};

fn parse_line(line: &str) -> (&str, impl Iterator<Item=Colors> + '_) {
    let (id, game) = line.split_once(": ").unwrap();
    (id, game.split("; ").map(|s| Colors::new(s)))
}

fn process_line1(line: &str) -> i64{
    let (id, mut turns) = parse_line(line);
    let valid_game = turns.all(|c| BAG.is_all_greater(&c));
    if valid_game {
        id.strip_prefix("Game ").unwrap().parse().unwrap()
    } else {
        0
    }
}

fn part1(input: &str) -> i64 {
    input.split_terminator('\n').map(|l| process_line1(l)).sum()
}

fn process_line2(line: &str) -> i64{
    let (_id, turns) = parse_line(line);
    turns.fold(Colors::default(), |acc: Colors, e| acc.max_of(&e)).power_of()
}

fn part2(input: &str) -> i64 {
    input.split_terminator('\n').map(|l| process_line2(l)).sum()
}

fn main() {
    let s = read_aoc!();
    println!("{:?}", part1(&s));
    println!("{:?}", part2(&s));
}
