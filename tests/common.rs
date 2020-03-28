use std::fs::*;

pub fn read_masses_from_file(name : String) -> Vec<i32>{
    let filename : String = format!("inputs/{}", name);
    let input_str = read_to_string(filename);
    let parsed_string = match input_str {
        Ok(str) => {str},
        Err(e) => {panic!(e)},
    };
    let masses : Vec<i32> = parsed_string.split_whitespace()
        .map(|str| str.parse::<i32>().unwrap()).collect();
    masses
}

pub fn read_02_input() -> Vec<i32> {
    let filename = format!("inputs/02_input.txt");
    let input_str = read_to_string(filename);
    let parsed_string = match input_str {
        Ok(str) => {str},
        Err(e) => {panic!(e)},
    };
    let masses : Vec<i32> = parsed_string.split(',')
        .map(|str| str.parse::<i32>().unwrap())
        .collect();
    masses
}