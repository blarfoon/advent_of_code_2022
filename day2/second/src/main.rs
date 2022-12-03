fn main() {
    let input = include_str!("../input.txt");

    let mut sum = 0;

    let lines: Vec<&str> = input.lines().collect();

    for lines in lines.chunks(3) {
        let chars: Vec<char> = lines[0].chars().collect();
        for char in chars {
            let is_common = lines[1..].iter().all(|line| line.contains(char));
            if is_common {
                let priority = if char.is_lowercase() {
                    (char as u32) - 96
                } else {
                    ((char as u32) - 64) + 26
                };

                sum += priority;
                break;
            }
        }
    }
    println!("{}", sum);
}
