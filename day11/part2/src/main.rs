use std::{env::args, fs::read_to_string};

fn main() {
    let input_file_path = get_input_file_path();
    let input = read_file(&input_file_path);
    let expanded_map = expand_map(&input);
    let result = shortest_maps_sum(&expanded_map);
    println!("Result: {}", result);
}

fn shortest_maps_sum(expanded_map: &Vec<(usize, usize)>) -> usize {
    expanded_map
        .iter()
        .enumerate()
        .map(|(index, (x1, y1))| {
            expanded_map
                .iter()
                .skip(index)
                .map(|(x2, y2)| {
                    (*x2 as i32 - *x1 as i32).abs() as usize
                        + (*y2 as i32 - *y1 as i32).abs() as usize
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn expand_map(map: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let empty_rows = find_empty_rows(map);
    let empty_columns = find_empty_columns(map);
    map.iter()
        .map(|(x, y)| {
            (
                *x + empty_columns
                    .iter()
                    .take_while(|column| *column < x)
                    .count()
                    * (1000000 - 1),
                *y + empty_rows.iter().take_while(|row| *row < y).count() * (1000000 - 1),
            )
        })
        .collect()
}

fn find_empty_rows(map: &Vec<(usize, usize)>) -> Vec<usize> {
    let height = *map.iter().map(|(_, y)| y).max().unwrap();
    let result = (0..height)
        .filter(|y| !map.iter().any(|(_, map_y)| map_y == y))
        .collect();
    result
}

fn find_empty_columns(map: &Vec<(usize, usize)>) -> Vec<usize> {
    let width = *map.iter().map(|(x, _)| x).max().unwrap();
    let result = (0..width)
        .filter(|x| !map.iter().any(|(map_x, _)| map_x == x))
        .collect();
    result
}

fn read_file(input_path: &String) -> Vec<(usize, usize)> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, character)| character == &'#')
                .map(move |(x, _)| (x, y))
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
