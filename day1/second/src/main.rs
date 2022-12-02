use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        match s {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid shape"),
        }
    }
}

enum GameOutcome {
    Win,
    Lose,
    Draw,
}

impl From<&str> for GameOutcome {
    fn from(outcome: &str) -> Self {
        match outcome {
            "X" => GameOutcome::Lose,
            "Y" => GameOutcome::Draw,
            "Z" => GameOutcome::Win,
            _ => panic!("Invalid outcome"),
        }
    }
}

fn determine_round_score(shape: Shape, outcome: GameOutcome) -> i32 {
    let mut total_score = match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    total_score += match outcome {
        GameOutcome::Win => 6,
        GameOutcome::Draw => 3,
        GameOutcome::Lose => 0,
    };
    
    total_score
}

fn determine_shape_for_outcome(shape: &Shape, outcome: &GameOutcome) -> Shape {
    match (shape, outcome) {
        (Shape::Rock, GameOutcome::Win) => Shape::Paper,
        (Shape::Rock, GameOutcome::Lose) => Shape::Scissors,
        (Shape::Rock, GameOutcome::Draw) => Shape:: Rock,
        (Shape::Paper, GameOutcome::Win) => Shape::Scissors,
        (Shape::Paper, GameOutcome::Lose) => Shape::Rock,
        (Shape::Paper, GameOutcome::Draw) => Shape::Paper,
        (Shape::Scissors, GameOutcome::Win) => Shape::Rock,
        (Shape::Scissors, GameOutcome::Lose) => Shape::Paper,
        (Shape::Scissors, GameOutcome::Draw) => Shape::Scissors,
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let mut my_total_score = 0;

    let mut lines = buf.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let opponent_shape = Shape::from(parts[0]);
        let outcome = GameOutcome::from(parts[1]);

        let my_shape = determine_shape_for_outcome(&opponent_shape, &outcome);
        let my_score = determine_round_score(my_shape, outcome);

        my_total_score += my_score;
    }

    println!("My total score: {}", my_total_score);
}
