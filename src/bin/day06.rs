use advent_of_code_2024::read_contents_buffered;
use chrono::Utc;
use std::collections::HashMap;

// Advent of Code 2024 Day 6
// 6 Dec 2024
// https://adventofcode.com/2024

fn print_grid(grid: &Vec<Vec<char>>, nrows: usize, ncols: usize)-> () {
    for i in 0..nrows {
        // rows
        for j in 0..ncols {
            // columns
            print!(" {}({},{})", grid[j][i],j, i);
        }
        println!();
    }
    println!();
}


fn main() {
    println!("--- Advent of Code 2024 ---");
    println!("--- Day 6: Guard Gallivant ---\n");

    // Reading buffered file contents into a string line by line
    let filename = "./input/day06.txt";
    // let filename = "./test_input/day06-test.txt";

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
    // Naive solution :)
    let input_vec = Vec::from(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>(),
    );
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
                grid[j][i] = '.';
            }
        }
    }
    println!("Guard start position (origin is top-left): x = {}, y  = {}", guard_x, guard_y);

    // for development let's see what the grid looks like.
    // print_grid(&grid, nrows, ncols);

    let direction_map = HashMap::from([('N', 'E'), ('E', 'S'), ('S', 'W'), ('W', 'N')]);
    let mut cycle_count = 0;
    let mut current_x = guard_x;
    let mut current_y = guard_y;
    let mut direction = 'N';
    loop {
        cycle_count += 1;
        let change: (i32, i32) = match direction {
            'N' => (0, -1),
            'S' => (0, 1),
            'E' => (1, 0),
            'W' => (-1, 0),
            _ => break,
        };
        let new_position_x: i32 = change.0 + current_x as i32;
        let new_position_y: i32 = change.1 + current_y as i32;
        if new_position_x >= 0
            && new_position_x < nrows as i32
            && new_position_y >= 0
            && new_position_y < ncols as i32
        {
            // dbg!(grid[new_position_x as usize][new_position_y as usize]);
            if grid[new_position_x as usize][new_position_y as usize] != '#' {
                grid[new_position_x as usize][new_position_y as usize] = 'X';
                current_x = new_position_x as usize;
                current_y = new_position_y as usize;
            }
            else {
                direction = direction_map[&direction];
            }
        } else {
            break;
        }
        // dbg!((current_x, current_y, direction));
        if cycle_count > 10000000 { println!("Break!"); break; }

    }

    // print_grid(&grid, nrows, ncols);

    let mut position_count = 1;  // guard's intial position counts as 1
    for i in 0..nrows {
        // rows
        for j in 0..ncols {
            // columns
            if grid[j][i] == 'X' {
                position_count += 1;
            }
        }
    }

    println!("Loop cycle count: {}", cycle_count);
    println!("Unique position count: {}", position_count);

    println!("Day 06 Part 1.  How many distinct positions will the guard visit before leaving the mapped area?  {position_count}");

    // Part 2

    // let answer_p2 = 0;
    // println!("Day 06 Part 2. ... {answer_p2}");

    // End
    let current_datetime = Utc::now();
    println!(
        "End.  Current date and time (UTC): {}",
        current_datetime.format("%Y-%m-%d %H:%M:%S")
    );
}
