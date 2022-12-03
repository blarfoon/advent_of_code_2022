static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    let input = include_str!("../input.txt");

    let mut sum = 0;

    let lines: Vec<&str> = input.lines().collect();

    for lines in lines.chunks(3) {
        let chars: Vec<char> = lines[0].chars().collect();
        for char in chars {
            let is_common = lines[1..].iter().all(|line| line.contains(char));
            if is_common {
                let priority = ASCII_LOWER
                    .iter()
                    .position(|&c| c == char.to_ascii_lowercase())
                    .unwrap();

                sum += priority + if char.is_lowercase() { 1 } else { 27 }; // implies + 1 for the index lookup
                break;
            }
        }
    }
    println!("{}", sum);
}
