use advent_of_code_2024::read_contents_buffered;
use chrono::Utc;
use regex::Regex;
use std::time::Instant;


// Advent of Code 2024 Day 7
// 7 Dec 2024
// https://adventofcode.com/2024



fn main() {
    println!("--- Advent of Code 2024 ---");
    println!("--- Day 7: Bridge Repair ---\n");

    // Reading buffered file contents into a string line by line
    let filename = "./input/day07.txt";
    // let filename = "./test_input/day07-test.txt";

    println!("Reading input file, filename = {}", filename);
    let input = match read_contents_buffered(filename) {
        Ok(file_contents) => {
            println!("Read input file contents successfully!\n\n");
            file_contents
        }
        Err(err) => {
            print!("Error reading input file contents: {:?}\n\n", err);
            panic!();
        }
    };

    // Check input was correctly read in.  Look for first and last values!
    // dbg!(&input);

    // Part 1
    let now = Instant::now();

    let input_vec = Vec::from(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
    );
    // dbg!(&input_vec);

    let mut calibration_eqs = Vec::new();
    for line in input_vec.iter() {
        let (sum, tail) = line.split_once(":").unwrap();
        let sum = sum.parse::<i64>().unwrap();
        let operands = tail.trim().split_whitespace()
            .map(|o| o.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        calibration_eqs.push((sum, operands));
    }
    // dbg!(&calibration_eqs);

    let _operators = ['+', '*'];

    let mut count_calibration_eqs : Vec<i64>= Vec::new();
    for eq in calibration_eqs.iter() {
        let mut results: Vec<i64> = Vec::new();
        let sum = eq.0 as i64;
        build(eq.1[0] as i64, eq.1[1], &eq.1[2..], _operators, &mut results, sum);
        results.sort();
        results.dedup();
        results.iter().for_each(|r| count_calibration_eqs.push(*r));
    }

    // dbg!(&count_calibration_eqs);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Day 07 Part 1.  What is their total calibration result?  {}", count_calibration_eqs.iter().sum::<i64>());

    // Part 2

    // let answer_p2 = 0;
    // println!("Day 07 Part 2. ... {answer_p2}");

    // End
    let current_datetime = Utc::now();
    println!(
        "End.  Current date and time (UTC): {}",
        current_datetime.format("%Y-%m-%d %H:%M:%S")
    );
}

fn build(result: i64, head: i32, tail: &[i32], operators: [char; 2], results: &mut Vec<i64>, sum: i64) {

    let add = result + head as i64;
    let mul = result * head as i64;
    if tail.is_empty() {
        if add == sum && add != mul{
            results.push(add);
        }
        if mul == sum && add != mul{
            results.push(mul);
        }
    }
    else {
        build(add, tail[0], &tail[1..], operators, results, sum);
        build(mul, tail[0], &tail[1..], operators, results, sum);
    }

    ()
}

