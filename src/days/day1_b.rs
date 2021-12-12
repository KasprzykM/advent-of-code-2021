pub fn run() {
    let result = include_str!("day1_input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|a| a[0] < a[1])
        .count();

    println!("day 1b: {}", result);
}
