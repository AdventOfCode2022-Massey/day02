// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 2.  
//! Bart Massey 2022

use aoc::*;
use std::collections::HashMap;

fn main() {
    let mut results = HashMap::new();

    match get_part() {
        Part1 => {
            // A or X = rock
            // B or Y = paper
            // C or Z = scizzors
            results.insert("A X".to_string(), 3 + 1);
            results.insert("A Y".to_string(), 6 + 2);
            results.insert("A Z".to_string(), 0 + 3);
            results.insert("B X".to_string(), 0 + 1);
            results.insert("B Y".to_string(), 3 + 2);
            results.insert("B Z".to_string(), 6 + 3);
            results.insert("C X".to_string(), 6 + 1);
            results.insert("C Y".to_string(), 0 + 2);
            results.insert("C Z".to_string(), 3 + 3);
        }
        Part2 => {
            // A = rock, X = lose
            // B = paper, Y = tie
            // C = scizzors, Z = win
            results.insert("A X".to_string(), 0 + 3);
            results.insert("A Y".to_string(), 3 + 1);
            results.insert("A Z".to_string(), 6 + 2);
            results.insert("B X".to_string(), 0 + 1);
            results.insert("B Y".to_string(), 3 + 2);
            results.insert("B Z".to_string(), 6 + 3);
            results.insert("C X".to_string(), 0 + 2);
            results.insert("C Y".to_string(), 3 + 3);
            results.insert("C Z".to_string(), 6 + 1);
        }
    }
    
    let mut points = 0u64;
    for line in input_lines() {
        if line.is_empty() {
            eprintln!("empty line: unexpected, skipping");
        }
        else {
            if results.contains_key(&line) {
                points += results[&line];
            }
            else {
                eprintln!("unexpected line '{line}', skipping");
            }
        }
    }
    println!("{points}");
}
