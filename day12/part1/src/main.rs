use std::{env::args, fs::read_to_string};

fn main() {
    let input_path = get_input_file_path();
    let input = read_file(&input_path);
    let result = total_arrangements(&input);
    println!("Result: {}", result);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spring {
    OPERATIONAL,
    DAMAGED,
    UNKNOWN,
}

fn total_arrangements(spring_groups: &Vec<(Vec<Spring>, Vec<i32>)>) -> i32 {
    spring_groups
        .iter()
        .map(|spring_group| find_arrangements(spring_group))
        .sum()
}

fn find_arrangements(spring_group: &(Vec<Spring>, Vec<i32>)) -> i32 {
    let springs = &spring_group.0;
    let groups = &spring_group.1;
    let expanded_springs = expand_springs(springs);
    expanded_springs
        .iter()
        .filter(|current_springs| &count_groups(current_springs) == groups)
        .count() as i32
}

fn expand_springs(springs: &Vec<Spring>) -> Vec<Vec<Spring>> {
    let first = *springs.first().unwrap();
    let first = if first == Spring::UNKNOWN {
        vec![Spring::OPERATIONAL, Spring::DAMAGED]
    } else {
        vec![first]
    };
    if springs.len() == 1 {
        return first.iter().map(|value| vec![*value]).collect();
    }
    first
        .iter()
        .flat_map(|first| {
            let others = &springs.iter().skip(1).copied().collect();
            let others_expanded = expand_springs(others);
            others_expanded
                .iter()
                .map(|others| {
                    let a = std::iter::once(*first)
                        .chain(others.iter().copied())
                        .collect::<Vec<Spring>>();
                    a
                })
                .collect::<Vec<Vec<Spring>>>()
        })
        .collect()
}

fn count_groups(springs: &Vec<Spring>) -> Vec<i32> {
    let mut result = vec![0];
    for spring in springs {
        match spring {
            Spring::DAMAGED => *result.last_mut().unwrap() += 1,
            Spring::OPERATIONAL => {
                if *result.last().unwrap() != 0 {
                    result.push(0)
                }
            }
            _ => panic!(),
        }
    }
    if *result.last().unwrap() == 0 {
        result.pop();
    }
    result
}

fn read_file(input_path: &String) -> Vec<(Vec<Spring>, Vec<i32>)> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .map(|line| {
            let (springs_str, arrangements_str) = line.split_once(' ').unwrap();
            let springs = springs_str
                .chars()
                .map(|character| match character {
                    '.' => Spring::OPERATIONAL,
                    '#' => Spring::DAMAGED,
                    '?' => Spring::UNKNOWN,
                    _ => panic!(""),
                })
                .collect();
            let arrangements = arrangements_str
                .split(',')
                .map(|number_str| number_str.parse::<i32>().unwrap())
                .collect();
            (springs, arrangements)
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
