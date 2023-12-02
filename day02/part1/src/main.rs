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
    let bag_content = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let result: i32 = lines
        .iter()
        .map(|line| is_game_possible(line.as_str(), &bag_content).unwrap_or(0))
        .sum();
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

fn is_game_possible(line: &str, bag_content: &HashMap<&str, i32>) -> Option<i32> {
    let (game_id, game_information)= line.split_once(':').unwrap();
    let sets: Vec<&str> = game_information.split(";").collect();
    for set in sets {
        let cubes_information: Vec<&str> = set.split(",").collect();
        for cube_information in cubes_information {
            // Remove the first whitespace, we now have "X color"
            let cube_information = cube_information.trim_start();
            let (quantity, color) = cube_information.split_once(" ").unwrap();
            let quantity = quantity.parse::<i32>().unwrap();
            if quantity > bag_content[color] {
                return None;
            }
        }
    }
    let game_id = game_id.split_once(' ').unwrap().1;
    let game_id = game_id.parse::<i32>().unwrap();
    Some(game_id)
}
