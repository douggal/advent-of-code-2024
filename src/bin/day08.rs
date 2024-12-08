use std::collections::hash_map::Entry;
use std::collections::HashMap;
use advent_of_code_2024::{read_puzzle_input};
use chrono::Utc;
use std::time::Instant;

// Advent of Code 2024 Day 8
// 8 Dec 2024
// https://adventofcode.com/2024

fn main() {
    println!("--- Advent of Code 2024 ---");
    println!("--- Day 8: Resonant Collinearity ---\n");

    // Reading input into a newline separated String
    //let filename = "./input/day08.txt";
    let filename = "./test_input/day08-test.txt";

    println!("Reading input file, filename = {}", filename);
    let input = read_puzzle_input(filename);

    // Debugging check input was correctly read in.
    // Look for first and last values!
    dbg!(&input);

    // Convert input to a Vector of Vectors of String
    let input_vec = Vec::from(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>(),
    );

    // number of columns (width)
    let nrows = input_vec[0].len();
    println!("Number of rows: {}", nrows);

    // number of rows (height)
    let ncols = input_vec[0].len();
    println!("Number of columns: {}", ncols);

    // Represent the map of the antennas as a 2D grid array using vectors
    // https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    // Origin is top left with grid growing down (y-axis) and to the right (x-axis).
    let mut grid = vec![vec!['*'; ncols]; nrows];
    let mut antennas: HashMap<char, Vec<(usize,usize)> >= HashMap::new();
    for i in 0..nrows {
        for j in 0..ncols {
            let token = input_vec[i].chars().nth(j).unwrap();
            grid[j][i] = token;

            // build HashMap of antenna locations
            if token != '.' {
                // https://stackoverflow.com/questions/33243784/append-to-vector-as-value-of-hashmap
                match antennas.entry(token) {
                    Entry::Vacant(e) => { e.insert(vec![(j, i)]); },
                    Entry::Occupied(mut e) => { e.get_mut().push((j, i)); }
                }
            }
        }
    }
    print_grid(&grid, nrows, ncols);
    dbg!(&antennas);


    // Track program runtime by "clock on the wall"
    let now = Instant::now();

    // Part 1

    let answer_p1 = 0;
    println!("Day 08 Part 1.  How many unique locations within the bounds of the map contain an antinode?  {answer_p1}");
    let elapsed = now.elapsed();
    println!("Elapsed time part 1: {:.2?}", elapsed);



    // Part 2

    let answer_p2 = 0;
    println!("Day 08 Part 2. ... {answer_p2}");
    let elapsed = now.elapsed();
    println!("Elapsed time parts 1 and 2: {:.2?}", elapsed);

    // End
    let current_datetime = Utc::now();
    println!(
        "End.  Current date and time (UTC): {}",
        current_datetime.format("%Y-%m-%d %H:%M:%S")
    );
}

fn print_grid(grid: &Vec<Vec<char>>, nrows: usize, ncols: usize)-> () {
    println!("Grid cell data(coords) ");
    for i in 0..nrows {
        // rows
        for j in 0..ncols {
            // columns
            print!("[ {}({},{})]", grid[j][i],j, i);
        }
        println!();
    }
    println!();
}