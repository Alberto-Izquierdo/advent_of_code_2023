use std::{collections::HashMap, env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let (directions, map) = read_file(&input_path);
    let result = traverse_map(&directions, &map);
    println!("Result: {}", result);
}

fn traverse_map(directions: &Vec<bool>, map: &HashMap<String, (String, String)>) -> usize {
    let mut current_node = "AAA".to_string();
    let end_node = "ZZZ";
    directions
        .iter()
        .cycle()
        .take_while(|go_left| {
            let (left, right) = map.get(&current_node).unwrap();
            current_node = if **go_left {
                left.clone()
            } else {
                right.clone()
            };
            current_node != end_node
        })
        .count()
        + 1
}

fn read_file(input_path: &String) -> (Vec<bool>, HashMap<String, (String, String)>) {
    let iter_lines = read_to_string(input_path).unwrap();
    let mut lines = iter_lines.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|character| character == 'L')
        .collect::<Vec<bool>>();
    let map: HashMap<String, (String, String)> = lines
        .skip(1)
        .map(|line| {
            let (key, values) = line.split_once('=').unwrap();
            let key = key.trim_end().to_string();
            let (left_value, right_value) = values.split_once(", ").unwrap();
            let left_value = left_value.strip_prefix(" (").unwrap().to_string();
            let right_value = right_value.strip_suffix(")").unwrap().to_string();
            (key, (left_value, right_value))
        })
        .collect();
    (directions, map)
}

fn get_input_file_path() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    args.iter().nth(1).unwrap().clone()
}
