use advent_of_code_2024::read_contents_buffered;
use chrono::Utc;
use regex::Regex;
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


    // How to find overlapping matches.  Look ahead:  https://stackoverflow.com/questions/11430863/how-to-find-overlapping-matches-with-a-regexp
    // let re_xmas = Regex::new(r"(?=(XMAS))").unwrap();
    // regex parse error:
    //     (?=(XMAS))
    //     ^^^
    // error: look-around, including look-ahead and look-behind, is not supported
    let re_xmas = Regex::new(r"XMAS").unwrap();
    for m in re_xmas.find_iter(&s) {
        let x = m.as_str();
        let s = m.start();
        println!("cap: {}, start: {}", x,s);
    }
    for m in re_xmas.find_iter(&s.chars().rev().collect::<String>()) {
        let x = m.as_str();
        let s = m.start();
        println!("cap: {}, start: {}", x,s);
    }

    // transpose and put string in column order



    let answer_p1 = 0;
    println!("Day 04 Part 1.   {answer_p1}");


    // Part 2

    let answer_p2 = 0;
    println!("Day 04 Part 2. ... {answer_p2}");

    // End
    let current_datetime = Utc::now();
    println!("End.  Current date and time (UTC): {}", current_datetime.format("%Y-%m-%d %H:%M:%S"));

}
