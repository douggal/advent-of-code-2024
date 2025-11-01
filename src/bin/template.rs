// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use advent_of_code_2024::{read_puzzle_input};
use chrono::Utc;
use std::time::Instant;

// Advent of Code 2024 Day XX
// XX Dec 2024
// https://adventofcode.com/2024

fn main() {
    println!("--- Advent of Code 2024 ---");
    println!("--- Day XX: ... ---\n");

    // Reading buffered file contents into a string line by line
    //let filename = "./input/dayXX.txt";
    let filename = "./test_input/dayXX-test.txt";

    println!("Reading input file, filename = {}", filename);
    let input = read_puzzle_input(filename);

    // Debugging check input was correctly read in.
    // Look for first and last values!
    dbg!(&input);

    // Track program runtime by "clock on the wall"
    let now = Instant::now();

    // Part 1

    let answer_p1 = 0;
    println!("Day XX Part 1.  ...?  {answer_p1}");
    let elapsed = now.elapsed();
    println!("Elapsed time part 1: {:.2?}", elapsed);

    // Part 2

    let answer_p2 = 0;
    println!("Day XX Part 2. ...? {answer_p2}");
    let elapsed = now.elapsed();
    println!("Elapsed time parts 1 and 2: {:.2?}", elapsed);

    // End
    let current_datetime = Utc::now();
    println!(
        "End.  Current date and time (UTC): {}",
        current_datetime.format("%Y-%m-%d %H:%M:%S")
    );
}
