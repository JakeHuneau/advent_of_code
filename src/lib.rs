use std::fs;
use std::str::FromStr;

pub fn parse_file<T: FromStr>(filename: &str) -> Vec<T> {
    let raw_data = fs::read_to_string(filename).expect("Unable to read file");
    let data = raw_data
        .split("\n")
        .filter_map(|s| s.parse::<T>().ok())
        .collect::<Vec<T>>();
    data
}

pub fn parse_file_delimiter<T: FromStr>(filename: &str, delimiter: &str) -> Vec<T> {
    let raw_data = fs::read_to_string(filename).expect("Unable to read file");
    let data = raw_data
        .split(delimiter)
        .filter_map(|s| s.parse::<T>().ok())
        .collect::<Vec<T>>();
    data
}
