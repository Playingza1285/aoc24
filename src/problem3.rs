#![allow(warnings)]

use crate::utilities::read_lines;

use std::path::Path;
use std::iter::Map;
use regex::Regex;

pub fn part1<P>(file_path: &P)
where P: AsRef<Path>, {
    let mut total = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            total += calculate_block(&line);
        }
    }
    println!("AOC24#3.1: {total}");
}

pub fn part2<P>(file_path: &P)
where P: AsRef<Path>, {
    let mut total = 0;
    let mut input = String::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            input += &line;
        }
    }
    let mut start = 0;
    let mut end = 0;
    loop {
        end = start + match input[start..].find("don't()") {
            None => input.len() - start,
            Some(index) => index,
        };
        total += calculate_block(&input[start..end]);

        start = end + match input[end..].find("do()") {
            None => input.len() - end,
            Some(index) => index,
        };

        if start == input.len() || end == input.len() { break; }
    }
    println!("AOC24#3.2: {total}");
}

fn calculate_block(block: &str) -> u32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    re.find_iter(&block).map(|m| m.as_str())
        .map(|m| m.split(',')
             .map(|s| s.as_bytes().into_iter().filter(|c| c.is_ascii_digit())
                  .map(|c| *c as char).collect::<String>().parse::<u32>().unwrap())
             .product::<u32>())
        .sum::<u32>()
}
