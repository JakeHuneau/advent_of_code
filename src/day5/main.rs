use advent_of_code::parse_file_delimiter;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

// Check if each passport is valid
// Run in parallel since we can
pub fn solver() {
    let data = parse_file_delimiter::<String>("src/day5/input", "\r\n");
    let max_seat = data.par_iter().map(|d| {
        let (row, col) = translate(d);
        row * 8 + col
    }).max();
    println!("max seat number: {:?}", max_seat);
}

// Same as above but with different check
pub fn solver_extra() {
    let data = parse_file_delimiter::<String>("src/day5/input", "\r\n");
}

fn translate(seat: &String) -> (u8, u8) {
    let row = u8::from_str_radix(&seat[..7].replace("F", "0").replace("B", "1"), 2).unwrap();
    let column = u8::from_str_radix(&seat[7..].replace("L", "0").replace("R", "1"), 2).unwrap();
    (row, column)
}