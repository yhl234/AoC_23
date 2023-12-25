use std::fs::read_to_string;
fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    let result: u32 = input
        .lines()
        .map(letters_to_digit)
        .map(|l| {
            let numbers: Vec<u32> = l.chars().filter_map(|c| c.to_digit(10)).collect();
            numbers
        })
        .map(|v| v.first().unwrap() * 10 + v.last().unwrap())
        .sum();
    println!("{:?}", result);
}

const DIGIT_IN_LETTERS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn letters_to_digit(line: &str) -> String {
    let mut result = String::from(line);
    for letters in DIGIT_IN_LETTERS {
        let buffer = letters.0.to_owned() + letters.1 + letters.0;
        result = result.replace(letters.0, &buffer);
        println!("{}", result);
    }
    println!("final: {}", result);

    result
}
