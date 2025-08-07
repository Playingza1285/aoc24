#![allow(warnings)]

use crate::utilities::read_lines;

use std::path::Path;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::{Ordering, Reverse};

pub fn part1<P>(file_path: &P) 
where P: AsRef<Path>, {
    let mut set1: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut set2: BinaryHeap<Reverse<u32>> = BinaryHeap::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let (val1, val2) = line.split_once(char::is_whitespace).unwrap();
            set1.push(Reverse(val1.parse::<u32>().unwrap()));
            set2.push(Reverse(val2.trim_start().parse::<u32>().unwrap()));
        }
    }

    let mut difference: u32 = 0;
    while let (Some(Reverse(n1)), Some(Reverse(n2))) = (set1.pop(), set2.pop()) {
        match n1.cmp(&n2) {
            Ordering::Less => difference += n2 - n1,
            Ordering::Greater => difference += n1 - n2,
            Ordering::Equal => (),
        };
    }
    println!("AOC#1: {difference}");
}

pub fn part2<P>(file_path: &P)
where P: AsRef<Path>, {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: HashMap<u32, usize> = HashMap::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let (val1, val2)  = line.split_once(char::is_whitespace).unwrap();
            left_list.push(val1.parse::<u32>().unwrap());
            let val2 = val2.trim_start().parse::<u32>().unwrap();
            right_list.insert(val2, match right_list.get(&val2) {
                None => 1,
                Some(count) => count + 1,
            });
        }
    }

    let mut similarity_score = 0;
    for n in left_list {
        similarity_score += n * match right_list.get(&n) {
            None => 0 as u32,
            Some(n) => *n as u32,
        };
    }
    println!("AOC#2: {similarity_score}");
}
