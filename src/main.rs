mod problem1;

use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];

    problem1::part1(file_path);
    problem1::part2(file_path);
}
