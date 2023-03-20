use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();

    let content = content.lines().collect::<Vec<&str>>();

    let elves = content.split(|&line| line.is_empty());
    let mut elves = elves
        .map(|e| e.iter().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<i32>>();

    elves.sort_unstable();

    let part2 = elves[elves.len() - 3..].iter().sum::<i32>();
    println!("Part 1: {}", elves.pop().unwrap());
    println!("Part 2: {}", part2);
}
