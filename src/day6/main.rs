use advent_of_code::parse_file_delimiter;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

// For each set of letters, we find the total number of distinct letters and then sum each of those
// counts up
// Run in parallel since we can
pub fn solver() {
    let data = parse_file_delimiter::<String>("src/day6/input", "\n\n");
    let sum = data
        .par_iter()
        .fold(|| 0_usize, |sum, d| sum + unique_letters(d.to_string()))
        .sum::<usize>();
    println!("sum of unique characters in each group: {:?}", sum);
}

// For each set of letters, we find how many letters have a count equal to the number of lines
// in that set of letters
pub fn solver_extra() {
    let data = parse_file_delimiter::<String>("src/day6/input", "\n\n");
    let sum = data
        .par_iter()
        .fold(|| 0_usize, |sum, d| sum + everyone_said_yes(d.to_string()))
        .sum::<usize>();
    println!(
        "sum of characters everyone in a group said yes to: {:?}",
        sum
    );
}

// Finds how many unique letters are in `letters`. Does this by adding each to a HashSet
// Then returning the length of that set
fn unique_letters(letters: String) -> usize {
    let single_line = letters.replace("\n", "");
    let mut letter_set = HashSet::new();
    single_line.chars().for_each(|char| {
        letter_set.insert(char);
    });
    letter_set.len()
}

// Find how many letters are contained in every line. Do this with a HashMap and a counter
// variable. Iterate over each line, and increment the counter for each line. Use the HashMap
// as a letter counter. Return how many letters in that HashMap have a counter equal to the
// people counter
fn everyone_said_yes(letters: String) -> usize {
    let mut people_counter: usize = 0;
    let mut letter_counter: HashMap<char, usize> = HashMap::new();
    letters.split("\n").for_each(|letter_line| {
        people_counter += 1;
        letter_line.chars().for_each(|char| {
            // let current_letter_count = letter_counter.get(&char);
            match letter_counter.get(&char) {
                Some(&val) => {
                    letter_counter.insert(char, val + 1);
                }
                None => {
                    letter_counter.insert(char, 1);
                }
            }
        });
    });
    let mut everyone_said_yes: usize = 0;
    for count in letter_counter.values() {
        if *count == people_counter {
            everyone_said_yes += 1;
        }
    }
    everyone_said_yes
}
