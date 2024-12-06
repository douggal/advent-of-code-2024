use advent_of_code_2024::read_contents_buffered;
use chrono::Utc;
use regex::Regex;
// Advent of Code 2024 Day 6
// 6 Dec 2024
// https://adventofcode.com/2024

fn main() {

    println!("--- Advent of Code 2024 ---");
    println!("--- Day 6: Guard Gallivant ---\n");

    // Reading buffered file contents into a string line by line
    //let filename = "./input/day06.txt";
    let filename = "./test_input/day06-test.txt";

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

    // Nifty way to replace chars in a string this using match expression.
    // https://stackoverflow.com/questions/34606043/how-do-i-replace-specific-characters-idiomatically-in-rust
    let s:String = input.chars()
        .map(|x| match x {
            '\n' => ' ',
            _ => x
        }).collect();
    println!("{}", s);

    let len_row = s.find(' ').unwrap();
    let len_col = s.chars().filter(|x| *x == ' ').count();
    println!("Row length: {}", len_row);
    println!("Column length: {}", len_col);


    let answer_p1 = 0;
    println!("Day 06 Part 1.  How many distinct positions will the guard visit before leaving the mapped area?  {answer_p1}");


    // Part 2

    // let answer_p2 = 0;
    // println!("Day 06 Part 2. ... {answer_p2}");

    // End
    let current_datetime = Utc::now();
    println!("End.  Current date and time (UTC): {}", current_datetime.format("%Y-%m-%d %H:%M:%S"));

}
