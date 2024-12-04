use advent_of_code_2024::read_contents_buffered;
use chrono::Utc;

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
        },
        Err(err) => {
            print!("Error reading input file contents: {:?}\n\n", err);
            panic!();
        }
    };

    // Check input was correctly read in.  Look for first and last values!
    // dbg!(&input);

    // Part 1
    let mut safes = Vec::<i32>::new();
    for line in input.lines() {
        let iter = line.split_whitespace();
        let xs = iter
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        //dbg!(&xs);

        // usinga sliding window of size 2, find difference between each pair of numbers
        // method 1
        // let mut t = vec!();
        // for i in 1..xs.len() {
        //     t.push(xs[i] - xs[i-1]);
        // }
        // dbg!(&t);

        // method 2
        let diffs = xs
            .windows(2)
            .map(|xs| {xs[1] - xs[0]})
            .collect::<Vec<i32>>();
        // dbg!(&diffs);

        // all the diffs between each sliding window pair must be either between 1 and 3 inclusive
        //  or all between -3 and -1 inclusive.
        let test_decreasing = diffs
            .iter()
            .filter(|k| **k >= -3 && **k <= -1)
            .count() == diffs.len();

        let test_increasing = diffs
            .iter()
            .filter(|k| **k >= 1 && **k <= 3)
            .count() == diffs.len();

        // dbg!(test_increasing, test_decreasing);

        // if passes either increasing or decreasing test add 1 to result vector else add 0
        if test_increasing || test_decreasing { safes.push(1)} else { safes.push(0)}

    }
    let answer_p1 = safes.iter().sum::<i32>();
    println!("Day 03 Part 1.  What do you get if you add up all of the results of the multiplications? {answer_p1}");

    // Part 2

    // same as Part 1, but I'm allowed to drop 1 bad level from each report
    let mut safes_p2 = Vec::<i32>::new();
    for line in input.lines() {
        let iter = line.split_whitespace();
        let xs = iter
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        //dbg!(&xs);

        let diffs = xs
            .windows(2)
            .map(|xs| { xs[1] - xs[0] })
            .collect::<Vec<i32>>();

        dbg!(&diffs);




    }

    let answer_p2 = 0;
    println!("Day 02 Part 2. How many reports are now safe? {answer_p2}");

    // End
    let current_datetime = Utc::now();
    println!("End.  Current date and time (UTC): {}", current_datetime.format("%Y-%m-%d %H:%M:%S"));

}
