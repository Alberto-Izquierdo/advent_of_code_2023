use std::{collections::HashMap, env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let (starting_nodes, directions, map) = read_file(&input_path);
    let result = traverse_map(&starting_nodes, &directions, &map);
    println!("Result: {}", result);
}

fn traverse_map(
    starting_nodes: &Vec<String>,
    directions: &Vec<bool>,
    map: &HashMap<(String, bool), String>,
) -> usize {
    let found_first_time = starting_nodes
        .iter()
        .map(|starting_node| {
            let mut current_node = starting_node;
            directions
                .iter()
                .cycle()
                .take_while(|go_left| {
                    current_node = map.get(&(current_node.to_string(), **go_left)).unwrap();
                    !current_node.ends_with('Z')
                })
                .count()
                + 1
        })
        .collect::<Vec<usize>>();
    println!("First times found: {:?}", found_first_time);
    lcm(&found_first_time)
}

fn lcm(input: &Vec<usize>) -> usize {
    let mut result = input.clone();
    while !all_values_equal(&result) {
        let (lowest_index, _) =
            result
                .iter()
                .enumerate()
                .fold((0, usize::MAX), |min, (index, current_amount)| {
                    if current_amount < &min.1 {
                        (index, *current_amount)
                    } else {
                        min
                    }
                });
        *result.get_mut(lowest_index).unwrap() += input.get(lowest_index).unwrap();
    }
    *result.get(0).unwrap()
}

fn all_values_equal(input: &Vec<usize>) -> bool {
    let first = input.get(0).unwrap();
    input.iter().all(|value| value == first)
}

fn read_file(input_path: &String) -> (Vec<String>, Vec<bool>, HashMap<(String, bool), String>) {
    let iter_lines = read_to_string(input_path).unwrap();
    let mut lines = iter_lines.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|character| character == 'L')
        .collect::<Vec<bool>>();
    let mut starting_nodes: Vec<String> = vec![];
    let map: HashMap<(String, bool), String> = lines
        .skip(1)
        .flat_map(|line| {
            let (key, values) = line.split_once('=').unwrap();
            let key = key.trim_end().to_string();
            let (left_value, right_value) = values.split_once(", ").unwrap();
            let left_value = left_value.strip_prefix(" (").unwrap().to_string();
            let right_value = right_value.strip_suffix(")").unwrap().to_string();
            if key.ends_with('A') {
                starting_nodes.push(key.clone());
            }
            std::iter::once(((key.clone(), false), (right_value)))
                .chain(std::iter::once(((key, true), (left_value))))
        })
        .collect();
    (starting_nodes, directions, map)
}

fn get_input_file_path() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    args.iter().nth(1).unwrap().clone()
}
