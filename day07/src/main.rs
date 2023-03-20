use std::{collections::HashMap, fs, str::FromStr};

enum Line {
    Cd(String),
    Ls,
    Dir(String),
    File(String, u64),
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('$') {
            let line = s.strip_prefix("$ ").unwrap();
            if line == "ls" {
                return Ok(Self::Ls);
            }
            let (_, dir) = line.split_once(' ').unwrap();
            return Ok(Self::Cd(dir.to_owned()));
        }

        let (a, b) = s.split_once(' ').unwrap();

        Ok(match a {
            "dir" => Self::Dir(b.to_owned()),
            _ => Self::File(b.to_owned(), a.parse().unwrap()),
        })
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    let mut dir_stack = Vec::new();
    for line in content.lines() {
        let line = line.parse::<Line>().unwrap();

        match line {
            Line::Cd(dir) => {
                if dir == ".." {
                    dir_stack.pop();
                } else {
                    dir_stack.push(dir);
                }
            }
            Line::Ls => (),
            Line::Dir(_) => (),
            Line::File(_, size) => {
                for i in 0..dir_stack.len() {
                    let dir = dir_stack[0..i + 1].join("/");
                    *dir_sizes.entry(dir).or_default() += size;
                }
            }
        };
    }

    let total = dir_sizes.values().filter(|v| *v <= &100000).sum::<u64>();
    println!("part one {total}");

    let need_to_delete = dir_sizes.get("/").unwrap_or(&0) - 40_000_000;

    let smallest_to_remove = dir_sizes
        .values()
        .filter(|v| *v >= &need_to_delete)
        .min()
        .unwrap();
    println!("part two {smallest_to_remove}");
}
