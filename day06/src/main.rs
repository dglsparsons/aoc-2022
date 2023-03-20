use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    for i in 3..content.len() {
        let set: HashSet<char> = HashSet::from_iter(content.chars().skip(i - 3).take(4));
        if set.len() == 4 {
            println!("{}", i + 1);
            break;
        }
    }

    for i in 14..content.len() {
        let set: HashSet<char> = HashSet::from_iter(content.chars().skip(i - 13).take(14));
        if set.len() == 14 {
            println!("{}", i + 1);
            break;
        }
    }
}
