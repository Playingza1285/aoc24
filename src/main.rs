mod problem1;
mod problem2;
mod problem3;
mod utilities;

use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];

    // problem1::part1(file_path);
    // problem1::part2(file_path);
    
    // problem2::part1(file_path);
    // problem2::part2(file_path);

    // problem3::part1(file_path);
    problem3::part2(file_path);
    
    // let test_string = "mul(2425";
    // let stripped: String = test_string.as_bytes().into_iter().filter(|c| c.is_ascii_digit()).map(|c| *c as char).collect();
    // dbg!(stripped);
}
