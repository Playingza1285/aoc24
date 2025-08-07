#![allow(warnings)]

use crate::utilities::read_lines;

use std::path::Path;
use std::cmp::Ordering;

pub fn part1<P>(file_path: &P)
where P: AsRef<Path>, {
    let mut safe_count: usize = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let mut slice = &line[..];
            let mut direction: Option<Ordering> = None;
            let mut last_val: Option<i32> = None;
            let mut val: i32;
            loop {
                (val, slice) = match slice.split_once(char::is_whitespace) {
                    None => (match slice.parse::<i32>() {
                        Ok(val) => val,
                        Err(_) => {
                            safe_count += 1;
                            break;
                        },
                    } , ""),
                    Some((val, slice)) => (val.parse::<i32>().unwrap(), slice),
                };

                match last_val {
                    None => (),
                    Some(last_val) => {
                        if (val - last_val).abs() < 1 || (val - last_val).abs() > 3 {
                            break;
                        }
                        match direction {
                            None => direction = Some(val.cmp(&last_val)),
                            Some(dir) => {
                                if dir != val.cmp(&last_val) {
                                    break;
                                }
                                direction = Some(val.cmp(&last_val));
                            }
                        };
                    },
                };
                last_val = Some(val);
            }
        }
    }
    println!("AOC#2.1: {safe_count}");
}

pub fn part2<P>(file_path: &P) 
where P: AsRef<Path>, {
    let mut safe_count: usize = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let mut line_values: Vec<i32> = Vec::new();
            let mut slice = &line[..];
            loop {
                let mut val: i32;
                (val, slice) = match slice.split_once(char::is_whitespace) {
                    None => (match slice.parse::<i32>() {
                        Ok(val) => val,
                        Err(_) => { break },
                    }, ""),
                    Some((val, slice)) => (val.parse::<i32>().unwrap(), slice),
                };
                line_values.push(val);
            }
            if check_validity(&line_values, None) { safe_count += 1; continue; }
            for i in 0..line_values.len() {
                if check_validity(&line_values, Some(i as usize)) { safe_count += 1; break; }
            }
        }
    }
    println!("AOC#2.2: {safe_count}");
}

fn check_validity(values: &[i32], skip_index: Option<usize>) -> bool {
    let mut direction: Option<Ordering> = None;
    let mut last_val: Option<i32> = None;

    for (i, val) in values.iter().enumerate() {
        if skip_index != None && i == skip_index.unwrap() { continue; }
        match last_val {
            None => (),
            Some(last_val) => {
                if (val - last_val).abs() < 1 || (val - last_val).abs() > 3 { return false; }
                match direction {
                    None => direction = Some(val.cmp(&last_val)),
                    Some(dir) => {
                        if dir != val.cmp(&last_val) { return false; }
                        direction = Some(val.cmp(&last_val));
                    }
                };
            }
        };
        last_val = Some(*val);
    }
    true
}
