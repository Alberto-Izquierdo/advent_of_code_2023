use std::{
    cmp::{max, min},
    env::args,
    fs::read_to_string,
};

fn main() {
    let input_path = get_input_file_path();
    let input = read_file(&input_path);
    let result = get_nearest_location(&input);
    println!("Result: {}", result);
}

struct Transformation {
    range: (i64, i64),
    transformation: i64,
}

fn get_nearest_location(input: &Vec<String>) -> i64 {
    let (seeds, maps) = generate_seeds_and_ranges(input);
    return get_result_from_seeds_and_maps(&seeds, &maps);
}

fn get_result_from_seeds_and_maps(seeds: &Vec<i64>, _maps: &Vec<Vec<Transformation>>) -> i64 {
    seeds
        .chunks(2)
        .map(|seed_range_info| {
            (
                seed_range_info[0],
                (seed_range_info[0] + seed_range_info[1] - 1),
            )
        })
        .flat_map(|seed_range| transform_range(seed_range, &_maps[0]))
        .flat_map(|seed_range| transform_range(seed_range, &_maps[1]))
        .flat_map(|seed_range| transform_range(seed_range, &_maps[2]))
        .flat_map(|seed_range| transform_range(seed_range, &_maps[3]))
        .flat_map(|seed_range| transform_range(seed_range, &_maps[4]))
        .flat_map(|seed_range| transform_range(seed_range, &_maps[5]))
        .flat_map(|seed_range| transform_range(seed_range, &_maps[6]))
        .min()
        .unwrap()
        .0
}

fn transform_range(
    seed_range: (i64, i64),
    transformations: &Vec<Transformation>,
) -> Vec<(i64, i64)> {
    let mut untouched = vec![seed_range];
    let mut transformed = vec![];
    for transformation in transformations {
        let mut new_untouched = vec![];
        for unt in untouched {
            let (untouched_tmp, new_transformed) = apply_transformation(unt, transformation);
            for range in untouched_tmp {
                new_untouched.push(range);
            }
            for range in new_transformed {
                transformed.push(range);
            }
        }
        untouched = new_untouched;
    }
    untouched.extend(transformed.iter());
    untouched
}

fn apply_transformation(
    seed_range: (i64, i64),
    transformation: &Transformation,
) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    if seed_range.0 > transformation.range.1 || seed_range.1 < transformation.range.0 {
        (vec![(seed_range.0, seed_range.1)], vec![])
    } else {
        // There is a match
        let mut untouched = vec![];
        let mut transformed = vec![];
        if seed_range.0 < transformation.range.0 {
            untouched.push((seed_range.0, transformation.range.0 - 1));
        }
        if seed_range.1 > transformation.range.1 {
            untouched.push((transformation.range.1 + 1, seed_range.1));
        }
        transformed.push((
            max(seed_range.0, transformation.range.0) + transformation.transformation,
            min(seed_range.1, transformation.range.1) + transformation.transformation,
        ));
        (untouched, transformed)
    }
}

fn generate_seeds_and_ranges(input: &Vec<String>) -> (Vec<i64>, Vec<Vec<Transformation>>) {
    let mut iter = input.iter();
    let seeds = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|number_str| number_str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let maps = iter.filter(|line| !line.is_empty()).fold(
        vec![],
        |mut vec_acc: Vec<Vec<Transformation>>, line| {
            let first_character = line.chars().next().unwrap();
            if first_character.is_numeric() {
                let numbers: Vec<_> = line
                    .split_whitespace()
                    .map(|number_str| number_str.parse::<i64>().unwrap())
                    .collect();
                vec_acc.last_mut().unwrap().push(Transformation {
                    range: (numbers[1], numbers[1] + numbers[2] - 1),
                    transformation: numbers[0] - numbers[1],
                });
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
