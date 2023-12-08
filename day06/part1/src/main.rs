use std::{env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let input = read_file(&input_path);
    let result = get_total_winning_situations(&input);
    println!("Result: {}", result);
}

struct Race {
    time: u32,
    distance: u32,
}

fn get_total_winning_situations(races: &Vec<Race>) -> usize {
    races
        .iter()
        .map(|race| get_winning_situations(race))
        .fold(1, |acc, amount| acc * amount)
}

fn get_winning_situations(race: &Race) -> usize {
    (1..race.distance)
        .map(|index| index + race.distance / index)
        .filter(|total_time| total_time < &race.time)
        .count()
}

fn read_file(input_path: &String) -> Vec<Race> {
    let binding = read_to_string(input_path).unwrap();
    let mut lines = binding.lines();
    let times = lines.next().unwrap().split_whitespace();
    let distances = lines.next().unwrap().split_whitespace();
    times
        .into_iter()
        .skip(1)
        .zip(distances.into_iter().skip(1))
        .map(|(time, distance)| Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        })
        .collect()
}

fn get_input_file_path() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    args.iter().nth(1).unwrap().clone()
}
