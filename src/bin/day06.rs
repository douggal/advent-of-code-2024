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
    // let filename = "./input/day06.txt";
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
    // Naive solution :)
    let input_vec = Vec::from(input.lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>());
    // dbg!(&input_vec);

    // number of columns (width)
    let nrows = input_vec[0].len();
    println!("Number of rows: {}", nrows);

    // number of rows (height)
    let ncols = input_vec[0].len();
    println!("Number of columns: {}", ncols);

    // Represent the grid as a 2D array using vectors
    // https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    // and note the position of the guard.
    // Origin is top left with grid growing down (y-axis) and to the right (x-axis).
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

    // for development let's see what the grid looks like.
    let mut x = 0;
    let mut y = 0;
    for i in 0..nrows { // rows
        for j in 0..ncols { // columns
            print!("{} ", grid[j][i] );
        }
        println!();
    }

    let mut cycle_count = 0;
    let mut current_x = guard_x;
    let mut current_y = guard_y;
    let mut direction = 'N';
    loop {
        cycle_count += 1;
        let mut x = current_x;
        let mut y = current_y;
        match direction {
            'N' => y -= 1,
            'S' => y += 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => break,
        }
        if grid[y][x] == '#' {

        }

        break;
    }
    println!("Cycle count: {}", cycle_count);


    let answer_p1 = 0;
    println!("Day 06 Part 1.  How many distinct positions will the guard visit before leaving the mapped area?  {answer_p1}");



    // Part 2

    // let answer_p2 = 0;
    // println!("Day 06 Part 2. ... {answer_p2}");

    // End
    let current_datetime = Utc::now();
    println!("End.  Current date and time (UTC): {}", current_datetime.format("%Y-%m-%d %H:%M:%S"));

}
