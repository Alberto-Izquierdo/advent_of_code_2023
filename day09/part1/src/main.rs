use itertools::{self, Itertools};
use std::{env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let history = read_file(&input_path);
    let result = get_all_prediction_sums(&history);
    println!("Result: {}", result);
}

fn get_all_prediction_sums(history: &Vec<Vec<i32>>) -> i32 {
    history
        .iter()
        .map(|history_entry| get_prediction_sum(history_entry))
        .sum()
}

fn get_prediction_sum(history_entry: &Vec<i32>) -> i32 {
    let mut result = vec![history_entry.clone()];
    while !is_history_complete(&result) {
        let last_entry = result.last().unwrap();
        let new_entry: Vec<i32> = last_entry
            .iter()
            .tuple_windows()
            .map(|(previous, next)| next - previous)
            .collect();
        result.push(new_entry);
    }
    calculate_prediction(&result)
}

fn calculate_prediction(partial_result: &Vec<Vec<i32>>) -> i32 {
    partial_result
        .iter()
        .rev()
        .fold(0, |acc, history_entry| acc + history_entry.last().unwrap())
}

fn is_history_complete(history: &Vec<Vec<i32>>) -> bool {
    let last_entry = history.last().unwrap();
    last_entry.iter().all(|number| number == &0)
}

fn read_file(input_path: &String) -> Vec<Vec<i32>> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number_str| number_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn get_input_file_path() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    args.iter().nth(1).unwrap().clone()
}
