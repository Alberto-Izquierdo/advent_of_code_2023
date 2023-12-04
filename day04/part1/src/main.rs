use std::{collections::HashSet, env::args, fs::read_to_string};

fn main() {
    let file_path = get_input_file_path();
    let file_content = read_file(&file_path);
    let result = process_input(&file_content);
    println!("Result: {}", result);
}

fn process_input(_content: &Vec<String>) -> usize {
    _content
        .iter()
        .map(|line| {
            let (_, line) = line.split_once(':').unwrap();
            let (winning_numbers, my_numbers) = line.split_once('|').unwrap();
            let winning_numbers: HashSet<&str> =
                winning_numbers.split_whitespace().into_iter().collect();
            let my_numbers: Vec<_> = my_numbers
                .split_whitespace()
                .filter(|number| winning_numbers.contains(number))
                .collect();
            if my_numbers.len() > 0 {
                2_usize.pow(my_numbers.len() as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn read_file(input_path: &String) -> Vec<String> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .into_iter()
        .map(|line| line.to_string())
        .collect()
}

fn get_input_file_path() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    args.iter().nth(1).unwrap().clone()
}
