mod days;

use days::{day1, day1_b};
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

    for day in days {
        let func = match day {
            1 => day1::run,
            2 => day1_b::run,
            _ => panic!("Day not implemented."),
        };

        println!("\n=== Day {:02} ===", day);
        func();
    }
}
