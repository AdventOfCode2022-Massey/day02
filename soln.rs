// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 2.  
//! Ben and Bart Massey 2022

use aoc::*;
use std::collections::HashMap;

fn main() {
    let scores = match get_part() {
        #[allow(clippy::identity_op)]
        Part1 => &[
            // A or X = rock
            // B or Y = paper
            // C or Z = scissors
            ("A X", 3 + 1),
            ("A Y", 6 + 2),
            ("A Z", 0 + 3),
            ("B X", 0 + 1),
            ("B Y", 3 + 2),
            ("B Z", 6 + 3),
            ("C X", 6 + 1),
            ("C Y", 0 + 2),
            ("C Z", 3 + 3),
        ],
        #[allow(clippy::identity_op)]
        Part2 => &[
            // A = rock, X = lose
            // B = paper, Y = tie
            // C = scissors, Z = win
            ("A X", 0 + 3),
            ("A Y", 3 + 1),
            ("A Z", 6 + 2),
            ("B X", 0 + 1),
            ("B Y", 3 + 2),
            ("B Z", 6 + 3),
            ("C X", 0 + 2),
            ("C Y", 3 + 3),
            ("C Z", 6 + 1),
        ],
    };

    let results: HashMap<String, u64> = scores
        .iter()
        .map(|&(desc, score)| (desc.to_string(), score))
        .collect();

    let mut points = 0u64;
    for line in input_lines() {
        if line.is_empty() {
            eprintln!("empty line: unexpected, skipping");
        } else if results.contains_key(&line) {
            points += results[&line];
        } else {
            eprintln!("unexpected line '{line}', skipping");
        }
    }
    println!("{points}");
}
