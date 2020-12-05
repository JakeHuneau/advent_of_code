use advent_of_code::parse_file;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

// Convert the seats to binary -> u8 and then get seat ID from row * 8 + col. Find the max
// Run in parallel since we can
pub fn solver() {
    let data = parse_file::<String>("src/day5/input");
    let max_seat = data
        .par_iter()
        .map(|d| {
            let (row, col) = translate(d);
            u16::from(row) * 8 + u16::from(col)
        })
        .max()
        .unwrap();
    println!("max seat number: {:?}", max_seat);
}

// Find sum of all seat IDs, then subtract that from the total sum of min_seat -> max_seat to find
// the missing seat
pub fn solver_extra() {
    let data = parse_file::<String>("src/day5/input");
    let seat_sum = data
        .par_iter()
        .fold(
            || 0_usize,
            |sum, d| {
                let (row, col) = translate(d);
                sum + usize::from(row) * 8 + usize::from(col)
            },
        )
        .sum::<usize>();
    // We know the min seat is 38 and max is 998. So the expected sum total is
    // (max - min + 1)(max + min) / 2
    let expected_sum = (998 - 38 + 1) * (998 + 38) / 2;
    println!("missing seat: {:?}", expected_sum - seat_sum);
}

// Switch those F with 0, B with 1, L with 0, and R with 1 and convert those binary to u8
fn translate(seat: &String) -> (u8, u8) {
    let row = u8::from_str_radix(&seat[..7].replace("F", "0").replace("B", "1"), 2).unwrap();
    let column = u8::from_str_radix(&seat[7..].replace("L", "0").replace("R", "1"), 2).unwrap();
    (row, column)
}
