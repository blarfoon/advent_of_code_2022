use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let mut lines = buf.lines();

    let mut calories = vec![0];

    let mut current_elf = 0;
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() {
            calories.push(0);
            current_elf += 1;
        } else {
            calories[current_elf] += line.parse::<i32>().unwrap();
        }
    }

    calories.sort();

    let top = calories
        .into_iter()
        .rev()
        .take(1)
        .reduce(|accum, item| accum + item)
        .unwrap();

    println!("Top: {:?}", top);
}
