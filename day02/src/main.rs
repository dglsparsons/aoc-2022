use std::{fs, str::FromStr};

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

type Turn = (Move, Move, Outcome);

fn shape_score(us: &Move) -> usize {
    match us {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn outcome_score(outcome: &Outcome) -> usize {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn outcome(us: &Move, them: &Move) -> Outcome {
    match us {
        Move::Rock => match them {
            Move::Paper => Outcome::Lose,
            Move::Rock => Outcome::Draw,
            Move::Scissors => Outcome::Win,
        },
        Move::Paper => match them {
            Move::Rock => Outcome::Win,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Lose,
        },
        Move::Scissors => match them {
            Move::Rock => Outcome::Lose,
            Move::Paper => Outcome::Win,
            Move::Scissors => Outcome::Draw,
        },
    }
}

fn shape_for_outcome(them: &Move, outcome: &Outcome) -> Move {
    match them {
        Move::Rock => match outcome {
            Outcome::Lose => Move::Scissors,
            Outcome::Draw => Move::Rock,
            Outcome::Win => Move::Paper,
        },
        Move::Paper => match outcome {
            Outcome::Lose => Move::Rock,
            Outcome::Draw => Move::Paper,
            Outcome::Win => Move::Scissors,
        },
        Move::Scissors => match outcome {
            Outcome::Lose => Move::Paper,
            Outcome::Draw => Move::Scissors,
            Outcome::Win => Move::Rock,
        },
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let turns = content
        .lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .map(|l| {
            (
                l[0].parse::<Move>().unwrap(),
                l[1].parse::<Move>().unwrap(),
                l[1].parse::<Outcome>().unwrap(),
            )
        })
        .collect::<Vec<Turn>>();

    let mut score = 0;
    for turn in turns.iter() {
        let out = outcome(&turn.1, &turn.0);
        score += shape_score(&turn.1) + outcome_score(&out);
    }

    println!("Part 1: {}", score);

    score = 0;
    for turn in turns {
        let our_move = shape_for_outcome(&turn.0, &turn.2);
        score += shape_score(&our_move) + outcome_score(&turn.2);
    }

    println!("Part 2: {}", score);
}
