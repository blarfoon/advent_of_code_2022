#![feature(test)]

extern crate test;
use test::Bencher;

const MARKER_LENGTH: usize = 4;

fn main() {
    let input = include_bytes!("../input.txt");
    do_work(input);
}

fn do_work(input: &[u8]) {
    let lookup: &mut [u8] = &mut [0; MARKER_LENGTH];

    'outer: for (index, window) in input.windows(MARKER_LENGTH).enumerate() {

        for (sub_index, byte) in window.iter().enumerate() {
            if lookup[..sub_index].contains(byte) {
                continue 'outer;
            } else {
                lookup[sub_index] = *byte;
            }
        }

        if lookup.len() == MARKER_LENGTH {
            println!("Found: {:?}", index + MARKER_LENGTH);
            break;
        }
    }
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let input = include_bytes!("../input.txt");
    b.iter(|| do_work(input));
}
