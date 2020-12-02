use std::env;
use std::process::exit;
use std::time::Instant;

mod day1;
mod day2;

// Times the function call
macro_rules! timed {
    ($func:expr) => {
        let _start_time = Instant::now();
        $func();
        println!(
            "\nCompleted problem in {:.3} ms",
            _start_time.elapsed().as_secs_f64() * 1000.
        );
    };
}

fn main() {
    // Get CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Must have 1 arguments: <problem_number");
        exit(1);
    }

    // Solve that problem
    let problem_number = args[1].parse::<usize>().expect("Could not parse argument");
    match problem_number {
        1 => {
            timed!(day1::main::solver);
            println!("--Bonus--");
            timed!(day1::main::solver_extra);
        }
        2 => {
            timed!(day2::main::solver);
        }
        _ => {
            println!("Problem {} is not available yet", problem_number);
        }
    }
}
