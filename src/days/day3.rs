use std::iter::FromIterator;

pub fn run() {
    let input: Vec<String> = include_str!("input_files/day3_input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let number_of_bits = 12;
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
            true => {
                gamma.push('1');
            }
            false => {
                gamma.push('0');
            }
        }
    }
    let byte_mask = 0b1111_1111_1111;
    let gamma = isize::from_str_radix(String::from_iter(gamma).as_str(), 2).unwrap();
    let epsilon = gamma ^ byte_mask;
    println!(
        "Gamma => {:b}  Epsilon => {:b}  Answer => {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
