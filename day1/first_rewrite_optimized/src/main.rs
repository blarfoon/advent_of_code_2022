use std::{fs, sync::Arc};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let score: u32 = contents
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            score_round(bytes[0], bytes[2]) as u32
        })
        .sum();
    println!("{score}");
}

fn score_round(opponent: u8, outcome: u8) -> u8 {
    let shift = outcome - b'X' + 2;
    let me = b'X' + (opponent - b'A' + shift) % 3;
    println!(
        "{}({opponent}) {}({outcome}) | 'X({}) + ({}({opponent}) - 'A({}) + shift({shift})) % 3 = me({me})",
        char::from(opponent),
        char::from(outcome),
        b'X',
        char::from(opponent),
        b'A',
    );
    (me - b'W') + 3 * (outcome - b'X')
}
