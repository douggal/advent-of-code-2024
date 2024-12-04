use std::collections::HashMap;
use advent_of_code_2024::read_contents_buffered;
use chrono::Utc;
use regex::{Captures, Regex};

// Advent of Code 2024 Day 3
// 3 Dec 2024
// https://adventofcode.com/2024

fn main() {
    println!("--- Advent of Code 2024 ---");
    println!("--- Day 3: Mull It Over ---\n");

    // Reading buffered file contents into a string line by line
    //let filename = "./input/day03.txt";
    let filename = "./test_input/day03-test.txt";

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

    // Let's try a finite state machine!
    // states {M,U,L,L,(,1,2,3,',',4,5,6,),S}

    // vector to hold each multiplication instruction
    let mut mulls: Vec<String> = Vec::new();

    // regex to match a single digit 0 to 9
    let re_digit = Regex::new(r"[0-9]").unwrap();

    let mut state = "E"; // Start and/or Error back to square 1
    let mut instruction: Vec<char> = Vec::new();
    for line in input.lines() {
        for c in line.trim().chars() {
            match state {
                "E" => {
                    state = if c == 'm' {
                        instruction.push(c);
                        "M"
                    } else {
                        "E"
                    }
                }
                "M" => {
                    state = if c == 'u' {
                        instruction.push(c);
                        "U"
                    } else {
                        "E"
                    }
                }
                "U" => {
                    state = if c == 'l' {
                        instruction.push(c);
                        "L"
                    } else {
                        "E"
                    }
                }
                "L" => {
                    state = if c == '(' {
                        instruction.push(c);
                        "OP"
                    } else {
                        "E"
                    }
                }
                "OP" => {
                    state = if re_digit.is_match(c.to_string().as_str()) {
                        instruction.push(c);
                        "DIGIT1"
                    } else {
                        "E"
                    }
                }
                "DIGIT1" => {
                    state = if re_digit.is_match(c.to_string().as_str()) {
                        instruction.push(c);
                        "DIGIT2"
                    } else if c == ',' {
                        instruction.push(c);
                        "COMMA"
                    } else {
                        "E"
                    }
                }
                "DIGIT2" => {
                    state = if re_digit.is_match(c.to_string().as_str()) {
                        instruction.push(c);
                        "DIGIT3"
                    } else if c == ',' {
                        instruction.push(c);
                        "COMMA"
                    } else {
                        "E"
                    }
                }
                "DIGIT3" => {
                    state = if c == ',' {
                        instruction.push(c);
                        "COMMA"
                    } else {
                        "E"
                    }
                }
                "COMMA" => {
                    state = if re_digit.is_match(c.to_string().as_str()) {
                        instruction.push(c);
                        "DIGIT4"
                    } else {
                        "E"
                    }
                }
                "DIGIT4" => {
                    state = if re_digit.is_match(c.to_string().as_str()) {
                        instruction.push(c);
                        "DIGIT5"
                    } else if c == ')' {
                        instruction.push(c);
                        "SUCCESS"
                    } else {
                        "E"
                    }
                }
                "DIGIT5" => {
                    state = if re_digit.is_match(c.to_string().as_str()) {
                        instruction.push(c);
                        "DIGIT6"
                    } else if c == ')' {
                        instruction.push(c);
                        "SUCCESS"
                    } else {
                        "E"
                    }
                }
                "DIGIT6" => {
                    state = if c == ')' {
                        instruction.push(c);
                        "SUCCESS"
                    } else {
                        "E"
                    }
                },
                _ => { println!("State doesn't match known value! {state}"); std::process::exit(1) },
            }
            dbg!(&instruction);
            if state == "SUCCESS" {
                mulls.push(instruction.iter().collect::<String>().clone().to_string());
                instruction.clear();
                state = "E";
            } else if state == "E" {
                instruction.clear();
            }
        }
    }

    // dbg!(&mulls);

    // for each instruction found, extract integers and multiply them together
    let re_instruction = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results: Vec<i32> = Vec::new();
    for s in mulls {
        let caps = re_instruction.captures(s.as_str()).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        results.push(x*y);
    }

    dbg!(&results);
    let answer_p1 = results.iter().sum::<i32>();
    println!("Day 03 Part 1.  What do you get if you add up all of the results of the multiplications? {answer_p1}");

    // Part 2

    let answer_p2 = 0;
    println!("Day 03 Part 2.  ... {answer_p2}");

    // End
    let current_datetime = Utc::now();
    println!(
        "End.  Current date and time (UTC): {}",
        current_datetime.format("%Y-%m-%d %H:%M:%S")
    );
}
