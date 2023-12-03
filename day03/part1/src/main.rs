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
        let mut current_number: String = "".to_string();
        inner_vector
            .into_iter()
            .enumerate()
            .for_each(|(x, character)| {
                if character.is_numeric() {
                    current_number = format!("{}{}", current_number, character);
                } else {
                    if !current_number.is_empty() {
                        let partial_result =
                            check_around(&current_number, x - current_number.len(), y, input)
                                .unwrap_or(0);
                        result += partial_result;
                        current_number.clear();
                    }
                }
            });
        if !current_number.is_empty() {
            let partial_result = check_around(
                &current_number,
                input.len() - 1 - current_number.len(),
                y,
                input,
            )
            .unwrap_or(0);
            result += partial_result;
            current_number.clear();
        }
    });
    result
}

fn check_around(number: &String, x: usize, y: usize, input: &Vec<Vec<char>>) -> Option<i32> {
    let left_x = if x > 0 { x - 1 } else { x };
    let right_x = if x + number.len() < input.get(0).unwrap().len() {
        x + number.len()
    } else {
        x + number.len() - 1
    };
    let top_y = if y > 0 { y - 1 } else { y };
    let bottom_y = if y < input.len() - 1 { y + 1 } else { y };
    for current_y in top_y..bottom_y + 1 {
        for current_x in left_x..right_x + 1 {
            let character = input.get(current_y).unwrap().get(current_x).unwrap();
            if *character != '.' && !character.is_numeric() {
                return Some(number.parse::<i32>().unwrap());
            }
        }
    }
    None
}
