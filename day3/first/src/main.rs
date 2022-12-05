struct Elf {
    start: i32,
    end: i32,
}

fn main() {
    let input = include_str!("../input.txt");
    let mut overlap_count = 0;

    for line in input.lines() {
        let (first, second) = line.split_once(",").unwrap();
        let (s1, e1) = first.split_once("-").unwrap();
        let (s2, e2) = second.split_once("-").unwrap();

        let first_elf = Elf {
            start: s1.parse::<i32>().unwrap(),
            end: e1.parse::<i32>().unwrap(),
        };

        let second_elf = Elf {
            start: s2.parse::<i32>().unwrap(),
            end: e2.parse::<i32>().unwrap(),
        };

        if first_elf.start >= second_elf.start && first_elf.end <= second_elf.end
            || second_elf.start >= first_elf.start && second_elf.end <= first_elf.end
        {
            overlap_count += 1;
        }
    }
    println!("Overlap count: {}", overlap_count);
}
