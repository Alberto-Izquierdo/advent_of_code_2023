use std::{env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let input = read_file(&input_path);
    let result = get_furthest_point_from_start(&input);
    println!("Result: {}", result);
}

fn get_furthest_point_from_start(input: &Vec<Vec<char>>) -> i32 {
    let starting_point = get_starting_point(input);
    let path = get_path(input, starting_point);
    path.len() as i32 / 2
}

fn add(left: (i32, i32), right: (i32, i32)) -> (i32, i32) {
    (left.0 + right.0, left.1 + right.1)
}

fn is_position_conected(
    current_position: (i32, i32),
    other_position: (i32, i32),
    path: &Vec<Vec<char>>,
) -> bool {
    if other_position.0 >= 0
        && other_position.1 >= 0
        && other_position.0 < path.first().unwrap().len() as i32
        && other_position.1 < path.len() as i32
    {
        let current_char = path
            .get(current_position.1 as usize)
            .unwrap()
            .get(current_position.0 as usize)
            .unwrap();
        let other_char = path
            .get(other_position.1 as usize)
            .unwrap()
            .get(other_position.0 as usize)
            .unwrap();
        let current_char_directions = get_directions_from_pipe(*current_char);
        let other_char_directions = get_directions_from_pipe(*other_char);
        if (add(current_position, current_char_directions.0) == other_position
            || add(current_position, current_char_directions.1) == other_position
            || *current_char == 'S')
            && (add(other_position, other_char_directions.0) == current_position
                || add(other_position, other_char_directions.1) == current_position
                || *other_char == 'S')
        {
            return true;
        }
    }

    return false;
}

fn get_directions_from_pipe(pipe: char) -> ((i32, i32), (i32, i32)) {
    match pipe {
        '|' => ((0, -1), (0, 1)),
        '-' => ((-1, 0), (1, 0)),
        'L' => ((0, -1), (1, 0)),
        'J' => ((0, -1), (-1, 0)),
        '7' => ((0, 1), (-1, 0)),
        'F' => ((0, 1), (1, 0)),
        _ => ((0, 0), (0, 0)),
    }
}

fn get_path(input: &Vec<Vec<char>>, starting_point: (i32, i32)) -> Vec<(i32, i32)> {
    let mut previous_position = starting_point;
    let mut current_position = starting_point;
    let mut result = vec![];
    while result.is_empty() || current_position != starting_point {
        for y in (0..3).step_by(2) {
            let next_position = (current_position.0, current_position.1 + y - 1);
            if next_position != previous_position {
                if is_position_conected(current_position, next_position, input) {
                    result.push(next_position);
                    previous_position = current_position;
                    current_position = next_position;
                    break;
                }
            }
        }
        for x in (0..3).step_by(2) {
            let next_position = (current_position.0 + x - 1, current_position.1);
            if next_position != previous_position {
                if is_position_conected(current_position, next_position, input) {
                    result.push(next_position);
                    previous_position = current_position;
                    current_position = next_position;
                    break;
                }
            }
        }
    }
    result
}

fn get_starting_point(input: &Vec<Vec<char>>) -> (i32, i32) {
    let mut result = (0, 0);
    let row_len = input.first().unwrap().len();
    for y in 0..input.len() {
        for x in 0..row_len {
            if input.get(y).unwrap().get(x).unwrap().clone() == 'S' {
                result = (x as i32, y as i32)
            }
        }
    }
    result
}

fn read_file(input_path: &String) -> Vec<Vec<char>> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_input_file_path() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    args.iter().nth(1).unwrap().clone()
}
