use advent_of_code::parse_file;

pub fn solver() {
    let mut data = parse_file::<usize>("src/day1/input");
    data.sort(); // Sort data so we can binary search

    for (i, &d) in data.iter().enumerate() {
        // find the difference of 2020 and each value in list
        // then see if that difference is in the list of data
        // If so, then print that and return
        let search_for = 2020 - d;
        // Only search data after current value
        if data[i + 1..].binary_search(&search_for).is_ok() {
            // only returns Ok() value if it was found
            println!("{} * {} = {}", d, search_for, d * search_for);
            return;
        }
    }
    println!("Didn't find any good pair");
    return;
}

pub fn solver_extra() {
    let mut data = parse_file::<usize>("src/day1/input");
    data.sort();
    for (i, &d) in data.iter().enumerate() {
        for (j, &d2) in data[i..].iter().enumerate() {
            // Check everything in remainder of data
            if d + d2 > 2020 {
                // Sanity check that we won't underflow
                continue;
            }
            let search_for = 2020 - d - d2;
            if data[j + 1..].binary_search(&search_for).is_ok() {
                println!("{} * {} * {} = {}", d, d2, search_for, d * d2 * search_for);
                return;
            }
        }
    }
}
