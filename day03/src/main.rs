use std::{collections::HashSet, fs};

fn get_priority(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 'a' as u32 + 1
    } else {
        *c as u32 - 'A' as u32 + 27
    }
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();

    let mut total = 0;
    let lines = content.lines().collect::<Vec<_>>();

    for line in lines.iter() {
        let first_half = &line[..line.len() / 2];
        let second_half = &line[line.len() / 2..];

        let first_half = first_half.chars().collect::<HashSet<_>>();
        let second_half = second_half.chars().collect::<HashSet<_>>();

        let intersect = first_half.intersection(&second_half);

        for c in intersect {
            total += get_priority(c);
        }
    }

    let mut total2 = 0;
    for line in lines.chunks(3).flat_map(<&[&str; 3]>::try_from) {
        let [a, b, c] = line.map(|s| s.chars().collect::<HashSet<_>>());
        let thing = a
            .intersection(&b)
            .map(|c| c.to_owned())
            .collect::<HashSet<_>>();
        let intersect = thing.intersection(&c);

        for c in intersect {
            total2 += get_priority(c);
        }
    }

    println!("part one {}", total);
    println!("part two {}", total2);
}
