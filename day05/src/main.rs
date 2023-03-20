use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::str::FromStr;

fn parse_stacks(input: &str) -> HashMap<usize, Vec<char>> {
    let mut stacks = HashMap::<usize, Vec<char>>::new();
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.reverse();
    for line in lines[1..].iter() {
        let line = line
            .chars()
            .enumerate()
            .filter(|&(i, _)| (i + 1) % 4 == 2)
            .map(|(_, c)| c)
            .collect::<Vec<_>>();

        for (i, c) in line.iter().enumerate() {
            if c != &' ' {
                stacks.entry(i + 1).or_default().push(*c);
            }
        }
    }

    stacks
}

struct Instruction {
    source: usize,
    dest: usize,
    count: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();

        Ok(Instruction {
            count: words[1].parse().unwrap(),
            source: words[3].parse().unwrap(),
            dest: words[5].parse().unwrap(),
        })
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let [raw_stacks, instructions]: [&str; 2] = content
        .split("\n\n")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut stacks = parse_stacks(raw_stacks);
    for line in instructions.lines() {
        let instruction = line.parse::<Instruction>().unwrap();
        for _ in 0..instruction.count {
            let c = stacks.get_mut(&instruction.source).unwrap().pop().unwrap();
            stacks.entry(instruction.dest).or_default().push(c);
        }
    }

    for i in 1..stacks.keys().len() + 1 {
        let x = stacks.get(&i).unwrap();
        let last = x.last().unwrap();
        print!("{last}");
    }
    println!();

    let mut stacks = parse_stacks(raw_stacks);
    for line in instructions.lines() {
        let instruction = line.parse::<Instruction>().unwrap();
        let stack = stacks.get_mut(&instruction.source).unwrap();
        let popped = stack
            .drain(stack.len() - instruction.count..)
            .collect::<Vec<_>>();
        for c in popped {
            stacks.entry(instruction.dest).or_default().push(c);
        }
    }

    for i in 1..stacks.keys().len() + 1 {
        let x = stacks.get(&i).unwrap();
        let last = x.last().unwrap();
        print!("{last}");
    }
    println!();
}
