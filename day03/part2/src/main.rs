use std::{env, fs::read_to_string};

fn main() {
    let input_file = get_input_file_path();
    let file_contents = read_file(&input_file);
    let result = process_input(&file_contents);
    println!("Result: {}", result);
}

fn read_file(input_path: &String) -> Vec<Vec<char>> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn get_input_file_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    args.iter().nth(1).unwrap().clone()
}

fn process_input(input: &Vec<Vec<char>>) -> i32 {
    let mut result: i32 = 0;
    input.into_iter().enumerate().for_each(|(y, inner_vector)| {
        inner_vector
            .into_iter()
            .enumerate()
            .for_each(|(x, character)| {
                if *character == '*' {
                    let value = check_gear(x, y, input);
                    result += value.unwrap_or(0);
                }
            });
    });
    result
}

fn check_gear(x: usize, y: usize, input: &Vec<Vec<char>>) -> Option<i32> {
    let left_x = if x > 0 { x - 1 } else { x };
    let right_x = if x < input.get(0).unwrap().len() - 1 {
        x + 1
    } else {
        x
    };
    let top_y = if y > 0 { y - 1 } else { y };
    let bottom_y = if y < input.len() - 1 { y + 1 } else { y };
    let mut found_numbers: Vec<(usize, usize)> = vec![];
    for current_y in top_y..bottom_y + 1 {
        for current_x in left_x..right_x + 1 {
            let character = input.get(current_y).unwrap().get(current_x).unwrap();
            if character.is_numeric() {
                found_numbers.push((current_x, current_y));
            }
        }
    }
    found_numbers = remove_adjacent_numbers(&found_numbers);
    if found_numbers.len() == 2 {
        let first_index = found_numbers.get(0).unwrap();
        let first_number = get_number(input, first_index.0, first_index.1);
        let second_index = found_numbers.get(1).unwrap();
        let second_number = get_number(input, second_index.0, second_index.1);
        Some(first_number * second_number)
    } else {
        None
    }
}

fn remove_adjacent_numbers(input: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut prev_value = *input.get(0).unwrap();
    let mut result: Vec<(usize, usize)> = vec![prev_value];
    for index in 1..input.len() {
        let current_value = *input.get(index).unwrap();
        if current_value.1 != prev_value.1 || current_value.0 != prev_value.0 + 1 {
            result.push(current_value);
        }
        prev_value = current_value;
    }
    result
}

fn get_number(input: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let row_size = input.get(0).unwrap().len();
    let mut result = input.get(y).unwrap().get(x).unwrap().to_string();
    for current_x in x + 1..row_size {
        let character = input.get(y).unwrap().get(current_x).unwrap();
        if character.is_numeric() {
            result = format!("{}{}", result, character);
        } else {
            break;
        }
    }
    for current_x in (0..x).rev() {
        let character = input.get(y).unwrap().get(current_x).unwrap();
        if character.is_numeric() {
            result = format!("{}{}", character, result);
        } else {
            break;
        }
    }
    result.parse().unwrap()
}
