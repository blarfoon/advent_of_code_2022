#![feature(test)]
#![feature(get_many_mut)]
#![feature(iter_array_chunks)]

extern crate test;

use test::Bencher;

use std::collections::VecDeque;

fn parse_move_instruction(line: &str) -> (i32, i32, i32) {
    let mut parts = line.split_ascii_whitespace();
    let quantity = parts.nth(1).unwrap().parse::<i32>().unwrap();
    let from = parts.nth(1).unwrap().parse::<i32>().unwrap() - 1;
    let to = parts.nth(1).unwrap().parse::<i32>().unwrap() - 1;

    (quantity, from, to)
}

fn main() {
    let input = include_str!("../input.txt");
    do_work(input);
}

fn do_work(input: &str) {
    let lines = input.lines();
    let mut crates: Vec<VecDeque<char>> = vec![VecDeque::new(); 10];

    for line in lines {
        if line.starts_with("m") {
            let (quantity, from, to) = parse_move_instruction(line);
            let stack = if &crates[from as usize].len() < &(quantity as usize) {
                crates[from as usize].len()
            } else {
                quantity as usize
            };

            let start = crates[from as usize].len() - stack;
            let end = crates[from as usize].len();
            // Unoptimized version
            // let moved = &crates[from as usize]
            //     .drain(start..end)
            //     .collect::<VecDeque<char>>();

            // crates[to as usize].extend(moved);

            // Kind of unsafe but optimized version
            let [from_crate, to_crate] = crates.get_many_mut([from as usize, to as usize]).unwrap();
            let moved = from_crate.drain(start..end);
            to_crate.extend(moved);
            
        } else if line.starts_with(" 1") {
            continue;
        } else {
            let crates_row_vec = line.chars();
            let crates_row = crates_row_vec.array_chunks::<4>();
            for (i, column) in crates_row.enumerate() {
                if column[1] !=  ' ' {
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
