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
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid shape"),
        }
    }
}

enum GameOutcome {
    Win,
    Lose,
    Draw,
}

fn determine_winner(shape1: &Shape, shape2: &Shape) -> Option<usize> {
    match (shape1, shape2) {
        (Shape::Rock, Shape::Scissors) | (Shape::Paper, Shape::Rock) | (Shape::Scissors, Shape::Paper) => Some(1),
        (Shape::Rock, Shape::Paper) | (Shape::Paper, Shape::Scissors) | (Shape::Scissors, Shape::Rock) => Some(2),
        _ => None,
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

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let mut my_total_score = 0;

    let mut lines = buf.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let opponent = Shape::from(parts[0]);
        let myself = Shape::from(parts[1]);

        let my_outcome = match determine_winner(&opponent, &myself) {
            Some(1) => GameOutcome::Lose,
            Some(2) => GameOutcome::Win,
            None | Some(_) => GameOutcome::Draw,
        };

        my_total_score += determine_round_score(myself, my_outcome);


    }

    println!("My total score: {}", my_total_score);
}
