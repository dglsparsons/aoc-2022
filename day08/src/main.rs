use std::fs;

#[derive(Default)]
struct Score {
    l: u32,
    r: u32,
    t: u32,
    b: u32,
}

impl Score {
    fn scenic(&self) -> u32 {
        self.l * self.r * self.t * self.b
    }
}

struct Tree {
    visible: bool,
    height: u32,
    score: Score,
}

fn main() {
    let mut content = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tree {
                    height: c.to_digit(10).unwrap(),
                    visible: false,
                    score: Score::default(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // scan L to R
    for row in content.iter_mut() {
        let mut max_height = None;
        let mut previous: Vec<u32> = Vec::new();
        for tree in row.iter_mut() {
            if max_height.map(|n| tree.height > n).unwrap_or(true) {
                max_height = Some(tree.height);
                tree.visible = true;
            }
            let mut score = 0;
            for t in previous.iter().rev() {
                score += 1;
                if t >= &tree.height {
                    break;
                }
            }
            tree.score.l = score;
            previous.push(tree.height);
        }
    }

    // R to L
    for row in content.iter_mut() {
        let mut max_height = None;
        let mut previous: Vec<u32> = Vec::new();
        for tree in row.iter_mut().rev() {
            if max_height.map(|n| tree.height > n).unwrap_or(true) {
                max_height = Some(tree.height);
                tree.visible = true;
            }
            let mut score = 0;
            for t in previous.iter().rev() {
                score += 1;
                if t >= &tree.height {
                    break;
                }
            }
            tree.score.r = score;
            previous.push(tree.height);
        }
    }

    // T to B
    for i in 0..content[0].len() {
        let mut max_height = None;
        let mut previous: Vec<u32> = Vec::new();
        for tree in content.iter_mut().map(|r| r.get_mut(i).unwrap()) {
            if max_height.map(|n| tree.height > n).unwrap_or(true) {
                max_height = Some(tree.height);
                tree.visible = true;
            }
            let mut score = 0;
            for t in previous.iter().rev() {
                score += 1;
                if t >= &tree.height {
                    break;
                }
            }
            tree.score.t = score;
            previous.push(tree.height);
        }
    }

    // B to T
    for i in 0..content[0].len() {
        let mut max_height = None;
        let mut previous: Vec<u32> = Vec::new();
        for tree in content.iter_mut().map(|r| r.get_mut(i).unwrap()).rev() {
            if max_height.map(|n| tree.height > n).unwrap_or(true) {
                max_height = Some(tree.height);
                tree.visible = true;
            }
            let mut score = 0;
            for t in previous.iter().rev() {
                score += 1;
                if t >= &tree.height {
                    break;
                }
            }
            tree.score.b = score;
            previous.push(tree.height);
        }
    }

    let visible_count: usize = content
        .iter()
        .map(|l| l.iter().filter(|t| t.visible).count())
        .sum();
    println!("visible {visible_count}");

    let highest_scenic = content
        .iter()
        .map(|l| l.iter().map(|t| t.score.scenic()).max().unwrap())
        .max()
        .unwrap();
    println!("highest scenic score {highest_scenic}");
}
