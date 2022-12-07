#![feature(test)]

extern crate test;
use test::Bencher;

const MARKER_LENGTH: usize = 14;

fn main() {
    let input = include_bytes!("../input.txt");
    do_work(input);
}

fn do_work(input: &[u8]) {
    'outer: for (index, window) in input.windows(MARKER_LENGTH).enumerate() {
        for (sub_index, byte) in window.iter().enumerate() {
            if window[..sub_index].contains(byte) {
                continue 'outer;
            }
        }
        println!("Found: {:?}", index + MARKER_LENGTH);
        return;
    }
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let input = include_bytes!("../input.txt");
    b.iter(|| do_work(input));
}
