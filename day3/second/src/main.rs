// enable unstable bench feature
#![feature(test)]
extern crate test;
use test::Bencher;

fn main() {
    let input = include_str!("../input.txt");
    do_work(input);
}

fn do_work(input: &str) {
    let mut overlap_count = 0;

    for line in input.lines() {
        let elves = line.split_once(",").unwrap();
        let (s1, e1) = elves.0.split_once("-").unwrap();
        let (s2, e2) = elves.1.split_once("-").unwrap();
        let mut first_elf = s1.parse::<i32>().unwrap()..=e1.parse::<i32>().unwrap();

        let second_elf = s2.parse::<i32>().unwrap()..=e2.parse::<i32>().unwrap();

        if first_elf.any(|x| second_elf.contains(&x)) {
            overlap_count += 1;
        }
    }
    println!("Overlap count: {}", overlap_count);
}

#[bench]
fn bench(b: &mut Bencher) {
    let input = include_str!("../input.txt");
    b.iter(|| do_work(input));
}
