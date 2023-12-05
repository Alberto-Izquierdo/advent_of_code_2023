use std::{env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let input = read_file(&input_path);
    let result = get_nearest_location(&input);
    println!("Result: {}", result);
}

fn get_nearest_location(input: &Vec<String>) -> u64 {
    let (seeds, maps) = generate_seeds_and_maps(input);
    return get_result_from_seeds_and_maps(&seeds, &maps);
}

fn get_result_from_seeds_and_maps(seeds: &Vec<u64>, maps: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    seeds
        .iter()
        .map(|seed| get_result_from_seed_and_map(*seed, &maps[0]))
        .map(|seed| get_result_from_seed_and_map(seed, &maps[1]))
        .map(|seed| get_result_from_seed_and_map(seed, &maps[2]))
        .map(|seed| get_result_from_seed_and_map(seed, &maps[3]))
        .map(|seed| get_result_from_seed_and_map(seed, &maps[4]))
        .map(|seed| get_result_from_seed_and_map(seed, &maps[5]))
        .map(|seed| get_result_from_seed_and_map(seed, &maps[6]))
        .min()
        .unwrap()
}

fn get_result_from_seed_and_map(seed: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    let mapping = map.iter().find(
        |(_destination_range_start, source_range_start, range_length)| {
            seed >= *source_range_start && seed <= *source_range_start + *range_length
        },
    );
    mapping.map_or(
        seed,
        |(destination_range_start, source_range_start, _range_length)| {
            seed + destination_range_start - source_range_start
        },
    )
}

fn generate_seeds_and_maps(input: &Vec<String>) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    let mut iter = input.iter();
    let seeds = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|number_str| number_str.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let maps = iter.filter(|line| !line.is_empty()).fold(
        vec![],
        |mut vec_acc: Vec<Vec<(u64, u64, u64)>>, line| {
            let first_character = line.chars().next().unwrap();
            if first_character.is_numeric() {
                let numbers: Vec<_> = line
                    .split_whitespace()
                    .map(|number_str| number_str.parse::<u64>().unwrap())
                    .collect();
                vec_acc
                    .last_mut()
                    .unwrap()
                    .push((numbers[0], numbers[1], numbers[2]));
            } else {
                vec_acc.push(vec![]);
            }
            vec_acc
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
