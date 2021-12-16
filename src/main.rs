mod days;

use days::{day1, day1_b, day2, day2_b, day3};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day to run as a command-line argument.");
    }

    let days: Vec<u8> = args
        .iter()
        .skip(1)
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();

    // TODO Refactor this so its less cumbersome, already quite bad.
    for day in days {
        let func = match day {
            0 => day1::run,
            1 => day1_b::run,
            2 => day2::run,
            3 => day2_b::run,
            4 => day3::run,
            _ => panic!("Day not implemented."),
        };

        println!("\n=== Day {:02} ===", day);
        func();
    }
}
