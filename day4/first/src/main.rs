#![feature(test)]

extern crate test;

use test::Bencher;

use std::collections::VecDeque;

fn parse_move_instruction(line: &str) -> (i32, i32, i32) {
    let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
    let quantity = parts[1].parse::<i32>().unwrap();
    let from = parts[3].parse::<i32>().unwrap() - 1;
    let to = parts[5].parse::<i32>().unwrap() - 1;

    (quantity, from, to)
}

fn main() {
    let input = include_str!("../input.txt");
    do_work(input);
}

fn do_work(input: &str) {
    let lines = input.lines();

    let mut crates: Vec<VecDeque<char>> = vec![VecDeque::new(); 10];

    // Build crates data structure
    for line in lines.into_iter() {
        if line.starts_with("m") {
            // Move instruction
            let (quantity, from, to) = parse_move_instruction(line);
            let stack = if &crates[from as usize].len() < &(quantity as usize) {
                crates[from as usize].len()
            } else {
                quantity as usize
            };

            let start = crates[from as usize].len() - stack;
            let end = crates[from as usize].len();

            // println!("{crates:?}");

            // println!("Moving {} from {} to {}", quantity, from, to);

            let moved = &crates[from as usize]
                .drain(start..end)
                .rev()
                .collect::<VecDeque<char>>();

            crates[to as usize].extend(moved);

            // println!("{crates:?}");
        } else if line.starts_with(" 1") {
            continue;
        } else {
            // Parse crates
            let crates_row_vec = line.chars().collect::<Vec<_>>();
            let crates_row = crates_row_vec.chunks(4);

            for (i, column) in crates_row.enumerate() {
                if column[1] != ' ' {
                    crates[i].push_front(column[1]);
                }
            }
        }
    }

    let mut result = String::from("");
    for mut my_crate in crates.into_iter().filter(|c| !c.is_empty()) {
        result.push(my_crate.pop_back().unwrap());
    }

    println!("{}", result);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let input = include_str!("../input.txt");
    b.iter(|| do_work(input));
}
