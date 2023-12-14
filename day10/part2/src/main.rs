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
    let width = input.get(0).unwrap().len() * 2 + 1;
    let height = input.len() * 2 + 1;
    let non_surrounded = get_non_surrounded_tiles(&path, width as i32, height as i32);
    //_draw_map(width, height, &path, &non_surrounded);
    get_surrounded_tiles(&non_surrounded, &path, width, height)
}

fn get_surrounded_tiles(
    non_surrounded: &Vec<(i32, i32)>,
    path: &Vec<(i32, i32)>,
    width: usize,
    height: usize,
) -> i32 {
    let mut result = 0;
    for y in 0..(height as i32) {
        if y % 2 == 1 {
            for x in 0..(width as i32) {
                if x % 2 == 1 {
                    if !non_surrounded.iter().any(|value| value == &(x, y))
                        && !path.iter().any(|value| value == &(x, y))
                    {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn get_adjacent_tiles(position: &(i32, i32), width: i32, height: i32) -> Vec<(i32, i32)> {
    let mut result = vec![];
    if position.0 > 0 {
        result.push((position.0 - 1, position.1));
    }
    if position.0 < width - 1 {
        result.push((position.0 + 1, position.1));
    }
    if position.1 > 0 {
        result.push((position.0, position.1 - 1));
    }
    if position.1 < height - 1 {
        result.push((position.0, position.1 + 1));
    }
    result
}

fn get_non_surrounded_tiles(path: &Vec<(i32, i32)>, width: i32, height: i32) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![(0, 0)];
    let mut checked_tiles = result.clone();
    let mut current_index = 0;
    //println!("path: {:?}", path);
    while current_index < result.len() {
        let adjacents = get_adjacent_tiles(result.get(current_index).unwrap(), width, height);
        //println!("Adjacents: {:?}", adjacents);
        for adjacent in adjacents.iter() {
            if !checked_tiles.iter().any(|value| value == adjacent) {
                //println!("not checked: {:?}", adjacent);
                if !path.iter().any(|value| value == adjacent) {
                    //println!("not in path: {:?}", adjacent);
                    result.push(*adjacent);
                }
                checked_tiles.push(*adjacent);
            }
        }
        current_index += 1;
    }
    result
}

fn _draw_map(
    width: usize,
    height: usize,
    path: &Vec<(i32, i32)>,
    non_surrounded: &Vec<(i32, i32)>,
) {
    for y in 0..(height as i32) {
        for x in 0..(width as i32) {
            if path.iter().any(|value| value == &(x, y))
                || non_surrounded.iter().any(|value| value == &(x, y))
            {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }
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
                    let fake_next_position = (next_position.0 * 2 + 1, next_position.1 * 2 + 1);
                    result.push(fake_next_position);
                    let fake_next_position =
                        (next_position.0 * 2 + 1, next_position.1 * 2 + 1 - (y - 1));
                    result.push(fake_next_position);
                    //result.push(next_position);
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
                    let fake_next_position = (next_position.0 * 2 + 1, next_position.1 * 2 + 1);
                    result.push(fake_next_position);
                    let fake_next_position =
                        (next_position.0 * 2 + 1 - (x - 1), next_position.1 * 2 + 1);
                    result.push(fake_next_position);
                    //result.push(next_position);
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
