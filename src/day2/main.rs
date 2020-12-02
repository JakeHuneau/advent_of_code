use advent_of_code::parse_file;
use rayon::prelude::*;
use std::str::FromStr;

// Tried to do this with Regex with the Regex engine is really slow when handling capture groups

// Run in parallel since we can
pub fn solver() {
    let data = parse_file::<String>("src/day2/input");

    // parallel fold and use a counter to sum the total number of good lines
    let good_passwords = data
        .par_iter()
        .fold(
            || 0_usize,
            |sum, d| {
                if passes_test(d.to_string()) {
                    sum + 1
                } else {
                    sum
                }
            },
        )
        .sum::<usize>();
    println!("{}", good_passwords);
}

pub fn solver_extra() {
    let data = parse_file::<String>("src/day2/input");

    // parallel fold and use a counter to sum the total number of good lines
    let good_passwords = data
        .par_iter()
        .fold(
            || 0_usize,
            |sum, d| {
                if passes_extra_test(d.to_string()) {
                    // println!("passed");
                    sum + 1
                } else {
                    // println!("no pass");
                    sum
                }
            },
        )
        .sum::<usize>();
    println!("{}", good_passwords);
}

// Manually split string to get each part of it
fn parse_str(password_test_str: &String) -> (usize, usize, &str, &str) {
    // {min}-{max} {required}: {test}
    let password_test_str_split = password_test_str.split(": ").collect::<Vec<&str>>();

    // {min}-{max} {required}
    let params = *password_test_str_split.get(0).unwrap();
    // {test}
    let test = *password_test_str_split.get(1).unwrap();

    // [{min}-{max}, {required}]
    let params_split = params.split(" ").collect::<Vec<&str>>();
    // {min}-{max}
    let min_max = params_split
        .get(0)
        .unwrap()
        .split("-")
        .collect::<Vec<&str>>();
    // {min}
    let min = usize::from_str(*min_max.get(0).unwrap()).unwrap();
    // {max}
    let max = usize::from_str(*min_max.get(1).unwrap()).unwrap();
    // {required}
    let required = *params.split(" ").collect::<Vec<&str>>().get(1).unwrap();
    (min, max, required, test)
}

// Filter the test string to get a count of the required char
// Then return if that count is within the allowed bounds
fn passes_test(password_test_str: String) -> bool {
    let (min, max, required, test) = parse_str(&password_test_str);
    // println!("{} {} {} {}", min, max, required, test);

    let character_count = test
        .chars()
        .filter(|&c| c == required.chars().next().unwrap())
        .collect::<Vec<char>>()
        .len();

    min <= character_count && character_count <= max
}

// Check if one of the indexes of test string matches the required char. Use XOR for check
fn passes_extra_test(password_test_str: String) -> bool {
    let (index1, index2, required, test) = parse_str(&password_test_str);

    let required_char = required.chars().next().unwrap();
    (test.chars().nth(index1 - 1).unwrap() == required_char)
        ^ (test.chars().nth(index2 - 1).unwrap() == required_char)
}
