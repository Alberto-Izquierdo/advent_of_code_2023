use std::{collections::HashMap, env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let input = read_file(&input_path);
    let result = get_nearest_location(&input);
    println!("Result: {}", result);
}

fn get_nearest_location(input: &Vec<String>) -> u32 {
    let (seeds, maps) = generate_seeds_and_maps(input);
    return get_result_from_seeds_and_maps(&seeds, &maps);
}

fn get_result_from_seeds_and_maps(seeds: &Vec<u32>, maps: &Vec<HashMap<u32, u32>>) -> u32 {
    *seeds
        .iter()
        .map(|seed| maps[0].get(seed).unwrap_or(seed))
        .map(|seed| maps[1].get(seed).unwrap_or(seed))
        .map(|seed| maps[2].get(seed).unwrap_or(seed))
        .map(|seed| maps[3].get(seed).unwrap_or(seed))
        .map(|seed| maps[4].get(seed).unwrap_or(seed))
        .map(|seed| maps[5].get(seed).unwrap_or(seed))
        .map(|seed| maps[6].get(seed).unwrap_or(seed))
        .min()
        .unwrap()
}

fn generate_seeds_and_maps(input: &Vec<String>) -> (Vec<u32>, Vec<HashMap<u32, u32>>) {
    let mut iter = input.iter();
    let seeds = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|number_str| number_str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let maps = iter.filter(|line| !line.is_empty()).fold(
        vec![],
        |mut acc: Vec<HashMap<u32, u32>>, line| {
            let first_character = line.chars().next().unwrap();
            if first_character.is_numeric() {
                let numbers = line
                    .split_whitespace()
                    .map(|number_str| number_str.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                for index in 0..numbers[2] {
                    acc.last_mut()
                        .unwrap()
                        .insert(numbers[1] + index, numbers[0] + index);
                }
            } else {
                acc.push(HashMap::new());
            }
            acc
        },
    );
    (seeds, maps)
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
