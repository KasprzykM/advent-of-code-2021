use core::panic;

pub fn run() {
    let input = include_str!("input_files/day2_input.txt").lines();
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input {
        let (direction, movement) = line.split_at(line.find(' ').unwrap());
        let movement: i32 = movement[1..].parse().unwrap();
        match direction {
            "forward" => horizontal += movement,
            "down" => depth += movement,
            "up" => depth -= movement,
            _ => panic!("Unexpected direction"),
        }
    }
    println!("{}", horizontal * depth);
}
