// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use advent_of_code_2024::read_puzzle_input;
use chrono::Utc;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

// Advent of Code 2024 Day 8
// 8 Dec 2024
// https://adventofcode.com/2024

// A struct with two fields
#[derive(Debug, Clone, Copy)]
#[derive(Eq)]
#[derive(Hash)]
struct Point {
    x: i32,
    y: i32,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn run() {
    println!("--- Advent of Code 2024 ---");
    println!("--- Day 8: ---\n");

    // Reading input into a newline separated String
    let filename = "./input/day08.txt";
    // let filename = "./test_input/day08-test.txt";

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
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
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
                        e.insert(vec![Point { x, y }]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(Point { x, y });
                    }
                }
            }
        }
    }
    // print_grid(&grid, nrows, ncols);
    // dbg!(&antennas);

    // Track program runtime by "clock on the wall"
    let now = Instant::now();

    // Part 1

    // This means that for any pair of antennas with the same frequency,
    // there are two antinodes, one on either side of them.

    // loop and for each pair of antennas find slope, tangent or rise over run and figure out
    // where the antinodes are.
    let mut antinodes = HashSet::new();
    for antenna in antennas.values() {
        // need to find all pairs, not a sliding window!
        // no duplicate pairs, each combo only once!
        let count = antenna.len();
        // dbg!(antenna);
        for r in 0..count {
            for s in r + 1..count {
                let point_1: Point;
                let point_2: Point;
                let antinode_1: Point;
                let antinode_2: Point;

                // order of points matters in my solution: order by x-values
                // point_1 is on the left side and point_2 is on the right
                if antenna[r].x <= antenna[s].x {
                    point_1 = Point {
                        x: antenna[r].x,
                        y: antenna[r].y,
                    };
                    point_2 = Point {
                        x: antenna[s].x,
                        y: antenna[s].y,
                    }
                } else {
                    point_1 = Point {
                        x: antenna[s].x,
                        y: antenna[s].y,
                    };
                    point_2 = Point {
                        x: antenna[r].x,
                        y: antenna[r].y,
                    }
                }

                let rise = point_1.y - point_2.y; // y-axis
                let run = point_1.x - point_2.x; // x-axis
                let slope = slope(rise, run);

                // dbg!((point_1, point_2, rise, run, slope));

                // now find the antinode(s) for this pair of antennas
                match slope {
                    Some(s) => {
                        // handle each case
                        // horizontal, vertical, sloped
                        if s == 0.0 {
                            // horizontal line
                            // left side antinode
                            antinode_1 = Point {
                                x: point_1.x - run,
                                y: point_1.y,
                            };
                            // right side antinode
                            antinode_2 = Point {
                                x: point_2.x + run,
                                y: point_2.y,
                            };
                        } else {
                            antinode_1 = Point {
                                x: point_1.x + run,
                                y: point_1.y + rise,
                            };
                            // right side
                            antinode_2 = Point {
                                x: point_2.x - run,
                                y: point_2.y - rise,
                            };
                        }
                    }
                    None => {
                        // Vertical line, slope is not defined.
                        // need to look at y-values since x-values are same
                       if point_1.y <= point_2.y {
                           // top antinode
                           antinode_1 = Point {
                               x: point_1.x,
                               y: point_1.y + rise,
                           };
                           // bottom antinode
                           antinode_2 = Point {
                               x: point_2.x,
                               y: point_2.y - rise,
                           };
                       } else {
                           antinode_1 = Point {
                               x: point_1.x,
                               y: point_1.y - rise,
                           };
                           // bottom antinode
                           antinode_2 = Point {
                               x: point_2.x,
                               y: point_2.y + rise,
                           };
                       }
                    }
                }

                if on_grid(antinode_1, nrows, ncols) {
                    antinodes.insert(antinode_1);
                    grid[antinode_1.x as usize][antinode_1.y as usize] = '#';
                }
                if on_grid(antinode_2, nrows, ncols) {
                    antinodes.insert(antinode_2);
                    grid[antinode_2.x as usize][antinode_2.y as usize] = '#';
                }
            }
        }
    }
    // antinodes.sort_by(|a, b| a.x.cmp(&b.x));
    // antinodes.dedup();
    // dbg!(&antinodes);

    // print_grid(&grid, nrows, ncols);

    // 374 too high
    // 369 too high
    // 351 wins

    let answer_p1 = antinodes.iter().count();
    println!("Day 08 Part 1.   {answer_p1}");
    let elapsed = now.elapsed();
    println!("Elapsed time part 1: {:.2?}", elapsed);

    // Part 2

    let answer_p2 = 0;
    println!("Day 08 Part 2. {answer_p2}");
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
    if p0.x < 0 || p0.x >= ncols || p0.y < 0 || p0.y >= nrows {
        false
    } else {
        true
    }
}

fn print_grid(grid: &Vec<Vec<char>>, nrows: i32, ncols: i32) -> () {
    // print out grid and coords of each point
    println!("Grid cell data(coords) ");
    for i in 0..nrows {
        // rows
        for j in 0..ncols {
            // columns
            print!(
                "{} ({},{}) ",
                grid[j as usize][i as usize], j as usize, i as usize
            );
        }
        println!();
    }
    println!();
}
