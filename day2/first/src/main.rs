static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    let input = include_str!("../input.txt");

    let mut sum = 0;

    for line in input.lines() {
        let first_half = &line[..line.len() / 2];
        let second_half = &line[line.len() / 2..];

        for char in first_half.chars() {
            let is_common = second_half.contains(char);
            if is_common {
                let position = ASCII_LOWER
                    .iter()
                    .position(|&c| c == char.to_ascii_lowercase())
                    .unwrap()
                    + 1;
                if char.is_lowercase() {
                    sum += position;
                } else {
                    sum += 26 + position;
                }
                break;
            }
        }
        
        println!("{}", sum);
    }
}
