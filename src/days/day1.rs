use std::fs;

pub fn run() {
    let contents = fs::read_to_string("src/days/input_files/day1_input.txt")
        .expect("Something went wrong reading the file");

    let input: Vec<i32> = contents
        .split('\n')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let mut count = 0;
    let mut previous_measure = input[0];
    for measure in input.into_iter().skip(1) {
        if measure > previous_measure {
            count += 1;
        }
        previous_measure = measure;
    }
    println!("Result: {}", count);
}
