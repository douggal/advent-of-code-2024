use advent_of_code_2024::read_contents_buffered;
use chrono::Utc;
use regex::Regex;
// Advent of Code 2024 Day 7
// 7 Dec 2024
// https://adventofcode.com/2024



fn main() {
    println!("--- Advent of Code 2024 ---");
    println!("--- Day 7: Bridge Repair ---\n");

    // Reading buffered file contents into a string line by line
    // let filename = "./input/day07.txt";
    let filename = "./test_input/day07-test.txt";

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
    let _re_parse = Regex::new(r"(\d+): ((\d+) )+(\d+)").unwrap();  // didn't work as I intended

    let input_vec = Vec::from(
        input
            .lines()
            .map(|line|line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            );
    dbg!(&input_vec);

    let mut calibration_eqs = Vec::new();
    for line in input_vec.iter() {
        let (sum, tail) = line.split_once(":").unwrap();
        let sum = sum.parse::<i32>().unwrap();
        let operands = tail.trim().split_whitespace()
            .map(|o| o.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        calibration_eqs.push((sum, operands));
    }
    dbg!(&calibration_eqs);

    let operands = ['+','*'];
    for eq in &calibration_eqs {
        let sum = eq.0;
        let mut adds = Vec::new();
        let mut muls = Vec::new();
        
        for pair in eq.1.windows(2) {
            for op in &operands {
                
                match op {
                    '+' => adds.push(pair[0] + pair[1]),
                    '*' => muls.push(pair[0] * pair[1]),
                    _ => unreachable!(),
                }
                
            }
        }
        for sum in &adds {
            for mul in &muls {
                if sum {  }
            }
            
        }
    }

    println!("Day 07 Part 1.  What is their total calibration result?  0");

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
