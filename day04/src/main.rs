use std::{fs, str::FromStr};

#[derive(Debug)]
struct Range(usize, usize);

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        Ok(Range(start, end))
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let pairs = content
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<Range>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line| {
            let mut i = line.into_iter();
            [i.next().unwrap(), i.next().unwrap()]
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for [a, b] in pairs.iter() {
        if (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1) {
            count += 1;
        }
    }

    let mut count2 = 0;
    for [a, b] in pairs.iter() {
        if (a.0 <= b.0 && a.1 >= b.0) || (b.0 <= a.0 && b.1 >= a.0) {
            count2 += 1;
        }
    }

    println!("part one {}", count);
    println!("part two {}", count2);
}
