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

    let input_vec = Vec::from(input.lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>());
    dbg!(&input_vec);

    // number of columns (width)
    let nrows = input_vec[0].len();
    println!("Number of rows: {}", nrows);

    // number of rows (height)
    let ncols = input_vec[0].len();
    println!("Number of columns: {}", ncols);

    // Create 2D array using vectors
    // https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    let mut grid = vec![vec!['X'; ncols]; nrows];
    let mut guard_x = 0;
    let mut guard_y = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            grid[j][i] = input_vec[i].chars().nth(j).unwrap();
            if grid[j][i] == '^' {
                guard_x = j;
                guard_y = i;
            }
        }
    }
    println!("Guard position: x = {}, y  = {}", guard_x, guard_y);


    // Let's see what the grid looks like,
    // and note the position of the guard
    let mut x = 0;
    let mut y = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            if grid[j][i] == '^' {
                x  = j;
                y = i;
            }
            print!("{} ", grid[j][i] );
        }
        println!();
    }





    let answer_p1 = 0;
    println!("Day 06 Part 1.  How many distinct positions will the guard visit before leaving the mapped area?  {answer_p1}");



    // Part 2

    // let answer_p2 = 0;
    // println!("Day 06 Part 2. ... {answer_p2}");

    // End
    let current_datetime = Utc::now();
    println!("End.  Current date and time (UTC): {}", current_datetime.format("%Y-%m-%d %H:%M:%S"));

}
