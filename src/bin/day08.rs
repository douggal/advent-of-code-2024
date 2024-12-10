// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use advent_of_code_2024::read_puzzle_input;
use chrono::Utc;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::time::Instant;

// Advent of Code 2024 Day 8
// 8 Dec 2024
// https://adventofcode.com/2024

// A struct with two fields
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

                let point_1:Point;
                let point_2:Point;
                let antinode_1: Point;
                let antinode_2: Point;

                // order of points matters (to me anyway), order by x-values
                // point_1 is on the left side and point_2 is on the right
                if antenna[r].0 <= antenna[s].0 {
                    point_1 = Point {
                        x:  antenna[r].0,
                        y:  antenna[r].1,
                    };
                    point_2 = Point {
                        x: antenna[s].0,
                        y: antenna[s].1
                    }
                } else {
                    point_1 = Point {
                        x:  antenna[s].0,
                        y:  antenna[s].1,
                    };
                    point_2 = Point {
                        x: antenna[r].0,
                        y: antenna[r].1
                    }
                }

                let rise = point_2.y - point_1.y ; // y-axis
                let run = point_2.x - point_1.x;  // x-axis
                let slope = slope(rise, run);

                // dbg!((x1, y1, x2, y2, rise, run, slope));

                // now find the antinode(s) for this pair of antennas
                match slope {
                    Some(s) => {
                        // handle each case
                        // horizontal, vertical, sloped
                        if s == 0.0 {
                            // horizontal line
                            // left side antinode
                            antinode_1 = Point {
                                x: point_1.x + run,
                                y: point_1.y,
                            };
                            // right side antinode
                            antinode_2 = Point {
                                x: point_2.x + run,
                                y: point_2.y,
                            };
                        } else { // s != 0
                            antinode_1 = Point {
                                x: point_1.x - run,
                                y: point_1.y - rise,
                            };
                            // right side
                            antinode_2 = Point {
                                x: point_1.x + run,
                                y: point_2.y + run,
                            };
                        }
                    }
                    None => {
                        // Vertical line, slope is not defined.
                        // horizontal line
                        // left side antinode
                        antinode_1 = Point {
                            x: point_1.x,
                            y: point_1.y + rise,
                        };
                        // right side antinode
                        antinode_2 = Point {
                            x: point_2.x,
                            y: point_2.y + rise,
                        };
                    }
                }

                if on_grid(antinode_1, nrows, ncols) {
                    antipodes.push(antinode_1);
                    grid[antinode_1.x as usize][antinode_1.y as usize] = '#';
                }
                if on_grid(antinode_2, nrows, ncols) {
                    antipodes.push(antinode_2);
                    grid[antinode_2.x as usize][antinode_2.y as usize] = '#';
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

fn slope(rise: i32, run: i32) -> Option<f32> {
    if run == 0 {
        return None;
    }
    let slope = rise as f32 / run as f32;
    Some(slope)
}

fn on_grid(p0: Point, nrows: i32, ncols: i32) -> bool {
    // returns true if point is within the grid, false otherwise
    if (p0.x < 0 || p0.x >= ncols) || (p0.y < 0 || p0.y >= nrows) {
        return false;
    }
    true
}

fn print_grid(grid: &Vec<Vec<char>>, nrows: i32, ncols: i32) -> () {
    // print out grid and coords of each point
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
