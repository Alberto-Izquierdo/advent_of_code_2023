use std::collections::HashMap;
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
    let numbers_as_strings = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    lines
        .iter()
        .map(|line| {
            let mut first_number: Option<String> = None;
            let mut current_substring = line.clone();
            while first_number.is_none() {
                let first_character = current_substring.chars().into_iter().next().unwrap();
                first_number = if first_character.is_numeric() {
                    Some(first_character.to_string())
                } else {
                    get_starting_number(&current_substring, &numbers_as_strings)
                };
                current_substring = current_substring.split_at(1).1.to_string();
            }
            let mut last_number: Option<String> = None;
            let mut current_substring = line.clone();
            while last_number.is_none() {
                let last_character = current_substring.chars().into_iter().rev().next().unwrap();
                last_number = if last_character.is_numeric() {
                    Some(last_character.to_string())
                } else {
                    get_ending_number(&current_substring, &numbers_as_strings)
                };
                current_substring = current_substring
                    .split_at(current_substring.len() - 1)
                    .0
                    .to_string();
            }
            format!("{}{}", first_number.unwrap(), last_number.unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .collect()
}

fn get_starting_number(line: &String, numbers_as_strings: &HashMap<&str, &str>) -> Option<String> {
    numbers_as_strings
        .iter()
        .filter(|(number_as_string, _)| line.starts_with(*number_as_string))
        .map(|(_, number)| number.to_string())
        .next()
}

fn get_ending_number(line: &String, numbers_as_strings: &HashMap<&str, &str>) -> Option<String> {
    numbers_as_strings
        .iter()
        .filter(|(number_as_string, _)| line.ends_with(*number_as_string))
        .map(|(_, number)| number.to_string())
        .next()
}
