use std::{collections::HashMap, collections::HashSet, env::args, fs::read_to_string};

fn main() {
    let file_path = get_input_file_path();
    let file_content = read_file(&file_path);
    let result = process_input(&file_content);
    println!("Result: {}", result);
}

fn process_input(content: &Vec<String>) -> usize {
    let number_of_cards = content.len();
    let mut cards_set: HashMap<usize, usize> =
        (0..number_of_cards).map(|index| (index, 1)).collect();
    content.iter().enumerate().for_each(|(index, line)| {
        let current_card_amount = *cards_set.get(&index).unwrap();
        let (_, line) = line.split_once(':').unwrap();
        let (winning_numbers, my_numbers) = line.split_once('|').unwrap();
        let winning_numbers: HashSet<&str> =
            winning_numbers.split_whitespace().into_iter().collect();
        let matching_numbers = my_numbers
            .split_whitespace()
            .filter(|current_number| winning_numbers.contains(current_number))
            .count();
        (1..matching_numbers + 1).for_each(|inner_index| {
            let next_index = index + inner_index;
            let current_amount = *cards_set.get(&next_index).unwrap();
            cards_set.insert(next_index, current_card_amount + current_amount);
        });
    });
    cards_set.iter().map(|(_, amount)| amount).sum()
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
