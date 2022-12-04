// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 2.  
//! Bart Massey 2022

use aoc::*;
use aoc_reparse::*;

fn outcome(opp: u32, me: u32) -> u32 {
    if opp == me {
        3
    } else if (opp + 1) % 3 == me {
        6
    } else {
        0
    }
}

fn play(opp: u32, fix: u32) -> u32 {
    match fix {
        0 => (opp + 2) % 3,
        1 => opp,
        2 => (opp + 1) % 3,
        _ => panic!("bad fix: {fix}"),
    }
}

fn main() {
    let parser = Reparse::new(r"([ABC]) ([XYZ])");
    let strategy = input_lines().map(|line| {
        let parsed = parser.parse(&line).unwrap();
        (
            parsed.get::<char>(1) as u32 - 'A' as u32,
            parsed.get::<char>(2) as u32 - 'X' as u32,
        )
    });
    match get_part() {
        Part1 => {
            let score: u32 = strategy
                .map(|(opp, me)| me + 1 + outcome(opp, me))
                .sum();
            println!("{score}");
        }
        Part2 => {
            let score: u32 = strategy
                .map(|(opp, fix)| (opp, play(opp, fix)))
                .map(|(opp, me)| me + 1 + outcome(opp, me))
                .sum();
            println!("{score}");
        }
    }
}
