use std::{collections::HashMap, env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let mut input = read_file(&input_path);
    input.sort();
    let result: i32 = input
        .iter()
        .enumerate()
        .map(|(index, hand_and_bet)| (index as i32 + 1) * hand_and_bet.bet)
        .sum();
    println!("Result: {}", result);
}

#[derive(Debug)]
struct HandAndBet {
    hand: CamelHand,
    bet: i32,
}

impl HandAndBet {
    fn new(hand: String, bet: i32) -> Self {
        let map = hand.chars().fold(HashMap::new(), |mut map, character| {
            let current_amount = map.get(&character).unwrap_or(&0);
            map.insert(character, current_amount + 1);
            map
        });
        let hand = hand
            .chars()
            .map(|character| {
                let value = character.to_string().parse::<i32>();
                if value.is_ok() {
                    value.unwrap()
                } else {
                    match character {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Unknown value"),
                    }
                }
            })
            .collect::<Vec<i32>>();
        let hand_type =
            map.iter().fold(
                HandType::HighCard,
                |current_hand_type, (_, amount)| match amount {
                    5 => HandType::FiveOfAKind,
                    4 => HandType::FourOfAKind,
                    3 if { HandType::OnePair == current_hand_type } => HandType::FullHouse,
                    3 => HandType::ThreeOfAKind,
                    2 if { HandType::ThreeOfAKind == current_hand_type } => HandType::FullHouse,
                    2 if { HandType::OnePair == current_hand_type } => HandType::TwoPair,
                    2 => HandType::OnePair,
                    1 => current_hand_type,
                    _ => panic!("Invalid amount"),
                },
            );
        HandAndBet {
            hand: CamelHand { hand, hand_type },
            bet,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct CamelHand {
    hand: Vec<i32>,
    hand_type: HandType,
}

impl Ord for HandAndBet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut ordering = self.hand.hand_type.cmp(&other.hand.hand_type);
        if ordering == std::cmp::Ordering::Equal {
            self.hand
                .hand
                .iter()
                .zip(other.hand.hand.iter())
                .skip_while(|(char1, char2)| char1 == char2)
                .take(1)
                .for_each(|(char1, char2)| ordering = char1.cmp(char2));
        }
        ordering
    }
}

impl PartialOrd for HandAndBet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandAndBet {
    fn eq(&self, other: &Self) -> bool {
        self.hand.hand == other.hand.hand && self.bet == other.bet
    }
}

impl Eq for HandAndBet {}

fn read_file(input_path: &String) -> Vec<HandAndBet> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next().unwrap().to_string();
            let bet: i32 = parts.next().unwrap().parse().unwrap();
            HandAndBet::new(hand, bet)
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
