use std::env;
use std::fs::read_to_string;

fn main() {
    // File hosts must exist in current path before this produces output
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    let input_file = args.iter().nth(1).unwrap();
    let lines = read_file(input_file);
    let numbers = extrac_number_from_lines(&lines);
    let result: i32 = numbers.iter().sum();
    println!("Result: {}", result);
}

fn read_file(input_path: &String) -> Vec<String> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .into_iter()
        .map(|line| line.to_string())
        .collect()
}

fn extrac_number_from_lines(lines: &Vec<String>) -> Vec<i32> {
    lines
        .iter()
        .map(|line| {
            let first_number = line
                .chars()
                .filter(|character| character.is_numeric())
                .next()
                .unwrap();
            let last_number = line
                .chars()
                .rev()
                .filter(|character| character.is_numeric())
                .next()
                .unwrap();
            let number = format!("{}{}", first_number, last_number);
            number.parse::<i32>().unwrap()
        })
        .collect()
}
