// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use advent_of_code_2024::read_puzzle_input;
use chrono::Utc;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::time::Instant;

// Advent of Code 2024 Day 8
// 8 Dec 2024
// https://adventofcode.com/2024

// A struct with two fields
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

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
    // dbg!(&input);

    // Convert input to a Vector of Vectors of String
    let input_vec = Vec::from(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>(),
    );
    // dbg!(&input_vec);

    // number of columns (width)
    let nrows = input_vec[0].len() as i32;
    println!("Number of rows: {}", nrows);

    // number of rows (height)
    let ncols = input_vec[0].len() as i32;
    println!("Number of columns: {}", ncols);

    // Represent the map of the antennas as a 2D grid array using vectors
    // https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    // Origin is top left with grid growing down (y-axis) and to the right (x-axis).
    let mut grid = vec![vec!['*'; ncols as usize]; nrows as usize];
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..nrows as usize {
        for j in 0..ncols as usize {
            let x = j as i32;
            let y = i as i32;
            let token = input_vec[i].chars().nth(j).unwrap();
            grid[j][i] = token;

            // build HashMap of antenna locations
            if token != '.' {
                // https://stackoverflow.com/questions/33243784/append-to-vector-as-value-of-hashmap
                match antennas.entry(token) {
                    Entry::Vacant(e) => {
                        e.insert(vec![(x, y)]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push((x, y));
                    }
                }
            }
        }
    }
    print_grid(&grid, nrows, ncols);
    // dbg!(&antennas);

    // Track program runtime by "clock on the wall"
    let now = Instant::now();

    // Part 1

    // This means that for any pair of antennas with the same frequency,
    // there are two antinodes, one on either side of them.

    // loop and for each pair of antennas find slope, tangent or rise over run and figure out
    // where the antinodes must be ???
    let mut antipodes = Vec::new();

    for antenna in antennas.values() {
        // need to find all pairs, not a sliding window!
        // no duplicate pairs, each combo only once!
        let count = antenna.len();
        dbg!(antenna);
        for r in 0..count {
            for s in r + 1..count {
                let (x1, y1) = antenna[r];
                let (x2, y2) = antenna[s];

                let rise = y2 - y1; // y-axis
                let run = x2 - x1; // x-axis
                let slope = rise as f32 / run as f32;

                dbg!((x1, y1, x2, y2, rise, run));

                // order of points matters, how to order the points ???

                if rise != 0 && run != 0 {
                    // that is, two different antennas, not same one twice.
                    // now find the antinode(s) for this pair of antennas
                    if rise == 0 {
                        // horizontal line
                        // add and subject x values
                        let point_1: Point;
                        let point_2: Point;
                        if x1 > x2 { // right side
                            point_1 = Point { x: x1 + run.abs(), y: y1 };
                        } else {
                            point_1 = Point { x: x2 + run.abs(), y: y2 };
                        }
                        if x1 < x2 { // left side
                            point_2 = Point { x: x1 - run.abs(), y: y1 };
                        } else {
                            point_2 = Point { x: x2 - run.abs(), y: y2 };
                        }
                        if on_grid(point_1, nrows, ncols) {
                            antipodes.push(point_1);
                            grid[point_1.x as usize][point_1.y as usize] = '#';
                        }
                        if on_grid(point_2, nrows, ncols) {
                            antipodes.push(point_2);
                            grid[point_2.x as usize][point_2.y as usize] = '#';
                        }
                    } else if run == 0 {
                        // vertical line
                        // add and subject x values
                        let point_1: Point;
                        let point_2: Point;
                        if y1 < y2 {
                            point_1 = Point {
                                x: x1,
                                y: y1 - rise,
                            };
                        } else {
                            point_1 = Point {
                                x: x2,
                                y: y2 - rise,
                            };
                        }
                        if x1 > x2 {
                            point_2 = Point {
                                x: x1,
                                y: y1 + rise,
                            };
                        } else {
                            point_2 = Point {
                                x: x2,
                                y: y2 + rise,
                            };
                        }

                        if on_grid(point_1, nrows, ncols) {
                            antipodes.push(point_1);
                            grid[point_1.x as usize][point_1.y as usize] = '#';
                        }
                        if on_grid(point_2, nrows, ncols) {
                            antipodes.push(point_2);
                            grid[point_2.x as usize][point_2.y as usize] = '#';
                        }
                    } else if slope > 0.0 {
                        // rises to the left
                        // add and subject x values
                        // 1st antinode
                        // Order matters !
                        let point_1: Point;
                        let point_2: Point;
                        if x1 < x2 {
                            point_1 = Point {
                                x: x1 - run.abs(),
                                y: y1 - rise,
                            };
                        } else {
                            point_1 = Point {
                                x: x2 - run.abs(),
                                y: y2 - rise,
                            };
                        }
                        if x2 > x1 {
                            point_2 = Point {
                                x: x2 + run.abs(),
                                y: y2 + rise,
                            };
                        } else {
                            point_2 = Point {
                                x: x1 + run.abs(),
                                y: y1 + rise,
                            };
                        }
                        if on_grid(point_1, nrows, ncols) {
                            antipodes.push(point_1);
                            grid[point_1.x as usize][point_1.y as usize] = '#';
                        }
                        if on_grid(point_2, nrows, ncols) {
                            antipodes.push(point_2);
                            grid[point_2.x as usize][point_2.y as usize] = '#';
                        }
                    } else {
                        // slope < 0.0
                        // rises to the right
                        // add and subject x values

                        let point_1: Point;
                        let point_2: Point;
                        if x1 > x2 {
                            point_1 = Point {
                                x: x1 + run.abs(),
                                y: y1 - rise.abs(),
                            };
                        } else {
                            // x2 > x1
                            point_1 = Point {
                                x: x2 + run.abs(),
                                y: y2 - rise.abs(),
                            };
                        }
                        if x1 < x2 {
                            point_2 = Point {
                                x: x1 + run.abs(),
                                y: y1 + rise.abs(),
                            };
                        } else {
                            // x1 < x2
                            point_2 = Point {
                                x: x2 + run.abs(),
                                y: y2 + rise,
                            };
                        }
                        if on_grid(point_1, nrows, ncols) {
                            antipodes.push(point_1);
                            grid[point_1.x as usize][point_1.y as usize] = '#';
                        }
                        if on_grid(point_2, nrows, ncols) {
                            antipodes.push(point_2);
                            grid[point_2.x as usize][point_2.y as usize] = '#';
                        }
                    }
                }
            }
        }
    }
    antipodes.dedup();
    dbg!(&antipodes);

    print_grid(&grid, nrows, ncols);

    let answer_p1 = antipodes.iter().count();
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

fn on_grid(p0: Point, nrows: i32, ncols: i32) -> bool {
    if (p0.x < 0 || p0.x >= ncols) || (p0.y < 0 || p0.y >= nrows) {
        return false;
    }
    true
}

fn print_grid(grid: &Vec<Vec<char>>, nrows: i32, ncols: i32) -> () {
    println!("Grid cell data(coords) ");
    for i in 0..nrows {
        // rows
        for j in 0..ncols {
            // columns
            print!(
                "[{}] ({},{}) ",
                grid[j as usize][i as usize], j as usize, i as usize
            );
        }
        println!();
    }
    println!();
}
