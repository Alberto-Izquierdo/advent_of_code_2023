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
    let result: i32 = lines
        .iter()
        .map(|line| {
            let cubes = get_min_cubes(line.as_str());
            cubes.0 * cubes.1 * cubes.2
        })
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

fn get_min_cubes(line: &str) -> (i32, i32, i32) {
    let (_, game_information) = line.split_once(':').unwrap();
    let sets: Vec<&str> = game_information.split(";").collect();
    let mut min_cubes = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for set in sets {
        let cubes_information: Vec<&str> = set.split(",").collect();
        for cube_information in cubes_information {
            // Remove the first whitespace, we now have "X color"
            let cube_information = cube_information.trim_start();
            let (quantity, color) = cube_information.split_once(" ").unwrap();
            let quantity = quantity.parse::<i32>().unwrap();
            if quantity > min_cubes[color] {
                min_cubes.insert(color, quantity);
            }
        }
    }
    (min_cubes["red"], min_cubes["green"], min_cubes["blue"])
}
