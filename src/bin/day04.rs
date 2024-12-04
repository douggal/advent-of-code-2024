use advent_of_code_2024::read_contents_buffered;
use chrono::Utc;

// Advent of Code 2024 Day 4
// 4 Dec 2024
// https://adventofcode.com/2024

fn main() {

    println!("--- Advent of Code 2024 ---");
    println!("--- Day 4: Ceres Search ---\n");

    // Reading buffered file contents into a string line by line
    //let filename = "./input/day04.txt";
    let filename = "./test_input/day04-test.txt";

    println!("Reading input file, filename = {}", filename);
    let input = match read_contents_buffered(filename) {
        Ok(file_contents) => {
            println!("Read input file contents successfully!\n\n");
            file_contents
        },
        Err(err) => {
            print!("Error reading input file contents: {:?}\n\n", err);
            panic!();
        }
    };

    // Check input was correctly read in.  Look for first and last values!
    // dbg!(&input);

    // Part 1

    let answer_p1 = 0;
    println!("Day 04 Part 1.  How many times does XMAS appear?  {answer_p1}");



    // Part 2

    let answer_p2 = 0;
    println!("Day 04 Part 2. ... {answer_p2}");

    // End
    let current_datetime = Utc::now();
    println!("End.  Current date and time (UTC): {}", current_datetime.format("%Y-%m-%d %H:%M:%S"));

}
