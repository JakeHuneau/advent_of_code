use advent_of_code::parse_file_delimiter;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

// Check if each passport is valid
// Run in parallel since we can
pub fn solver() {
    let data = parse_file_delimiter::<String>("src/day4/input", "\n\n");

    let count = data
        .into_par_iter()
        .fold(
            || 0_usize,
            |sum, passport| {
                if is_valid(passport) {
                    sum + 1
                } else {
                    sum
                }
            },
        )
        .sum::<usize>();

    println!("Total valid IDs: {}", count);
}

// Same as above but with different check
pub fn solver_extra() {
    let data = parse_file_delimiter::<String>("src/day4/input", "\n\n");

    let count = data
        .into_par_iter()
        .fold(
            || 0_usize,
            |sum, passport| {
                if is_valid_bonus(passport) {
                    sum + 1
                } else {
                    sum
                }
            },
        )
        .sum::<usize>();

    println!("Total valid IDs: {}", count);
}

// Checks if all the required fields are present in the passport:
// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
fn is_valid(passport: String) -> bool {
    let mut fields = HashSet::new();

    // Make everything on one line to be easier to handle
    let passport_formatted = passport.replace("\n", " ");
    // Grab the field of everything by first splitting each k:v, then splitting by : and grabbing
    // first index of that
    passport_formatted.split(" ").for_each(|kv| {
        fields.insert(*kv.split(":").collect::<Vec<&str>>().get(0).unwrap());
    });

    // Now check if everything is in there
    fields.contains("byr")
        && fields.contains("iyr")
        && fields.contains("eyr")
        && fields.contains("hgt")
        && fields.contains("hcl")
        && fields.contains("ecl")
        && fields.contains("pid")
}

// Checks if all the passports have valid fields
fn is_valid_bonus(passport: String) -> bool {
    // Make everything on one line to be easier to handle
    let passport_formatted = passport.replace("\n", " ");
    // Split each key:value pair and then give 1 point for each valid field
    let valid_fields = passport_formatted.split(" ").fold(0, |sum, kv| {
        let kv_vec = kv.split(":").collect::<Vec<&str>>();
        if valid_key_value_pair(kv_vec) {
            sum + 1
        } else {
            sum
        }
    });

    // Need 7 points to win
    valid_fields == 7
}

// Uses rules to chek the key:value pairs
fn valid_key_value_pair(kv_vec: Vec<&str>) -> bool {
    let key: &str = kv_vec.get(0).unwrap();
    let value: &str = kv_vec.get(1).unwrap();
    match key {
        "byr" => {
            let year = usize::from_str(value).unwrap_or(0);
            year >= 1920 && year <= 2002
        }
        "iyr" => {
            let year = usize::from_str(value).unwrap_or(0);
            year >= 2010 && year <= 2020
        }
        "eyr" => {
            let year = usize::from_str(value).unwrap_or(0);
            year >= 2020 && year <= 2030
        }
        "hgt" => {
            if value.ends_with("cm") {
                let height = usize::from_str(value.strip_suffix("cm").unwrap()).unwrap_or(0);
                height >= 150 && height <= 193
            } else if value.ends_with("in") {
                let height = usize::from_str(value.strip_suffix("in").unwrap()).unwrap_or(0);
                height >= 59 && height <= 76
            } else {
                false
            }
        }
        "hcl" => value.starts_with("#") && value.len() == 7,
        "ecl" => {
            value == "amb"
                || value == "blu"
                || value == "brn"
                || value == "gry"
                || value == "grn"
                || value == "hzl"
                || value == "oth"
        }
        "pid" => value.len() == 9 && isize::from_str(value).unwrap_or(-1) != -1,
        _ => false,
    }
}
