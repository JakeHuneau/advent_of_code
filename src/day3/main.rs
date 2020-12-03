use advent_of_code::parse_file;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::iter::Sum;

// enumerate data to get line index.
// We can find the index that we hit with `line_number * 3 % line.length`
// If that char is # then we hit a tree
// Run in parallel since we can
pub fn solver() {
    let data = parse_file::<String>("src/day3/input");

    let trees = data
        .into_par_iter()
        .enumerate()
        .fold(
            || 0_usize,
            |trees, (i, line)| {
                if line.chars().nth(i * 3 % line.len()).unwrap() == '#' {
                    trees + 1
                } else {
                    trees
                }
            },
        )
        .sum::<usize>();

    println!("hit {} trees", trees);
}

// Similar to the above, but we'll do each of the checks for each line
// Use a TreeCounter object for keeping track of all those
pub fn solver_extra() {
    let data = parse_file::<String>("src/day3/input");
    let mut tree_counter = data
        .into_par_iter()
        .enumerate()
        .fold(
            || TreeCounter::new(),
            |mut tree_counter, (i, line)| {
                if line.chars().nth(i % line.len()).unwrap() == '#' {
                    // right 1 down 1
                    tree_counter.inc(1);
                }
                if line.chars().nth(i * 3 % line.len()).unwrap() == '#' {
                    // right 3 down 1
                    tree_counter.inc(2);
                }
                if line.chars().nth(i * 5 % line.len()).unwrap() == '#' {
                    // right 5 down 1
                    tree_counter.inc(3);
                }
                if line.chars().nth(i * 7 % line.len()).unwrap() == '#' {
                    // right 7 down 1
                    tree_counter.inc(4);
                }
                if i % 2 == 0 && line.chars().nth((i / 2) % line.len()).unwrap() == '#' {
                    // right 1 down 2
                    tree_counter.inc(5);
                }
                tree_counter
            },
        )
        .sum::<TreeCounter>();

    println!("product of slope trees is {}", tree_counter.product())
}

// Tree counter that can increment a counter for each possible slope
// And has a product method for multiplying all the slopes together
struct TreeCounter {
    slope1: usize,
    slope2: usize,
    slope3: usize,
    slope4: usize,
    slope5: usize,
}

impl TreeCounter {
    pub fn new() -> TreeCounter {
        TreeCounter {
            slope1: 0,
            slope2: 0,
            slope3: 0,
            slope4: 0,
            slope5: 0,
        }
    }

    pub fn inc(&mut self, slope_num: u8) {
        match slope_num {
            1 => {
                self.slope1 += 1;
            }
            2 => {
                self.slope2 += 1;
            }
            3 => {
                self.slope3 += 1;
            }
            4 => {
                self.slope4 += 1;
            }
            5 => {
                self.slope5 += 1;
            }
            _ => (),
        }
    }

    pub fn product(&mut self) -> usize {
        self.slope1 * self.slope2 * self.slope3 * self.slope4 * self.slope5
    }
}

// Have to implement sum for fold sum
impl Sum for TreeCounter {
    fn sum<I>(iter: I) -> TreeCounter
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(TreeCounter::new(), |a, b| TreeCounter {
            slope1: a.slope1 + b.slope1,
            slope2: a.slope2 + b.slope2,
            slope3: a.slope3 + b.slope3,
            slope4: a.slope4 + b.slope4,
            slope5: a.slope5 + b.slope5,
        })
    }
}
