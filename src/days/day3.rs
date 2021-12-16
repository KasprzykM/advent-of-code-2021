use std::iter::FromIterator;

pub fn run() {
    run_a();
}

fn run_a() {
    let input: Vec<String> = include_str!("input_files/day3_input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let number_of_bits = input[0].len();
    let mut gamma: Vec<char> = vec![];
    for bit in 0..number_of_bits {
        let mut ones = 0;
        let mut zeroes = 0;
        for line in input.iter() {
            match line.chars().nth(bit).unwrap() {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("Unknown bit"),
            }
        }
        match ones > zeroes {
            true => gamma.push('1'),
            false => gamma.push('0'),
        }
    }
    let byte_mask = isize::from_str_radix("1".repeat(number_of_bits).as_str(), 2).unwrap();
    let gamma = isize::from_str_radix(String::from_iter(gamma).as_str(), 2).unwrap();
    let epsilon = gamma ^ byte_mask;
    println!(
        "a) Gamma => {:b}  Epsilon => {:b}  Answer => {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
