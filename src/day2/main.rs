use advent_of_code::parse_file;
use regex::Regex;
use std::str::FromStr;

// Tried to do this with Regex with the Regex engine is really slow when handling capture groups

pub fn solver() {
    let data = parse_file::<String>("src/day2/input");

    let good_passwords = data.iter().fold(0, |sum, d| {
        if passes_test(d.to_string()) {
            println!("passed");
            sum + 1
        } else {
            println!("no pass");
            sum
        }
    });
    println!("{}", good_passwords);
}

// Tests the string by first regexing it to find the min and max allowed characters and what
// the allowed character is. Then builds another regex from that to test the actual test string.
fn passes_test(password_test_str: String) -> bool {
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
    let min = *min_max.get(0).unwrap();
    // {max}
    let max = *min_max.get(1).unwrap();
    // {required}
    let required = *params.split(" ").collect::<Vec<&str>>().get(1).unwrap();

    let regex_string = format!("^(\\w*{}\\w*){{{},{}}}$", required, min, max);
    println!("testing {} with {}", test, regex_string);
    let test_re = Regex::new(regex_string.as_str()).unwrap();
    test_re.is_match(test)
}
