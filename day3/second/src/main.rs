struct Elf {
    start: i32,
    end: i32,
}

fn main() {
    let input = include_str!("../input.txt");
    let mut overlap_count = 0;

    for line in input.lines() {
        let elves = line.split(",").collect::<Vec<_>>();
        let first_elf_parts = elves[0].split("-").collect::<Vec<_>>();
        let second_elf_parts = elves[1].split("-").collect::<Vec<_>>();
        let mut first_elf =
            first_elf_parts[0].parse::<i32>().unwrap()..=first_elf_parts[1].parse::<i32>().unwrap();

        let second_elf = second_elf_parts[0].parse::<i32>().unwrap()
            ..=second_elf_parts[1].parse::<i32>().unwrap();

        if first_elf.any(|x| second_elf.contains(&x)) {
            overlap_count += 1;
        }
    }
    println!("Overlap count: {}", overlap_count);
}
