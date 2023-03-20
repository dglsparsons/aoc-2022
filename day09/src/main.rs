use std::{cmp::Ordering, collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut snake = vec![(0, 0); 10];
    let mut tail_history = vec![HashSet::from([(0, 0)]); 9];
    for line in content.lines() {
        let (direction, times) = line.split_once(' ').unwrap();
        let times = times.parse::<u32>().unwrap();

        for _ in 0..times {
            let head = snake.get_mut(0).unwrap();
            match direction {
                "R" => head.1 += 1,
                "L" => head.1 -= 1,
                "U" => head.0 += 1,
                _ => head.0 -= 1,
            };

            for (i, history) in tail_history.iter_mut().enumerate() {
                let head = snake[i];
                let tail = snake.get_mut(i + 1);
                match tail {
                    None => (),
                    Some(tail) => {
                        let y_diff = (head.0 - tail.0 as i32).abs();
                        let x_diff = (head.1 - tail.1 as i32).abs();
                        if y_diff > 1 || x_diff > 1 {
                            *tail = (
                                match head.0.cmp(&tail.0) {
                                    Ordering::Equal => tail.0,
                                    Ordering::Greater => tail.0 + 1,
                                    Ordering::Less => tail.0 - 1,
                                },
                                match head.1.cmp(&tail.1) {
                                    Ordering::Equal => tail.1,
                                    Ordering::Greater => tail.1 + 1,
                                    Ordering::Less => tail.1 - 1,
                                },
                            );
                            history.insert(*tail);
                        }
                    }
                }
            }
        }
    }

    let visited_count = tail_history[0].len();
    println!("part 1: {visited_count}");

    let visited_count = tail_history[8].len();
    println!("part 2: {visited_count}");
}
