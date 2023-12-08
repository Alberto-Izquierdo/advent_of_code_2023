use std::{env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let input = read_file(&input_path);
    let result = get_winning_situations(&input);
    println!("Result: {}", result);
}

struct Race {
    time: u128,
    distance: u128,
}

fn get_winning_situations(race: &Race) -> usize {
    /*
     * We want to know the values for which the expression "x + distance / x < time" is true
     * If we solve the x, we end up with "0 < -x^2 + time*x - distance", we can now get the values
     * by applying (-b +- √(b^2 - 4ac)) / 2a
     */
    let a = -1 as f64;
    let b = race.time as f64;
    let c = -(race.distance as f64);
    let ac = (b * b - 4.0 * a * c).sqrt();
    let v1 = (-b + ac) / 2.0 * a;
    let v2 = (-b - ac) / 2.0 * a;
    (v2.ceil() - v1.ceil()) as usize
}

fn read_file(input_path: &String) -> Race {
    let binding = read_to_string(input_path).unwrap();
    let mut lines = binding.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u128>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u128>()
        .unwrap();

    Race { time, distance }
}

fn get_input_file_path() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    args.iter().nth(1).unwrap().clone()
}
