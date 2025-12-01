use std::time::Instant;
// use regex::Regex;


////////////////////////////////////////////////////////////////
// Advent of Code 2024 Day 4
// Link: <a href="...">https://adventofcode.com/2024/day/4</a>
// 1 December 2025 - warm-up for AoC 2025
////////////////////////////////////////////////////////////////

// Grid:
// (q,r) == (y, x) == (row, column), reverse of typical Cartesian coord point
// q increases going down and r increases to the right
//
// [1 (0, 0)]	[2 (0, 1)]	[3 (0, 2)]
// [4 (1, 0)]	[5 (1, 1)]	[6 (1, 2)]
// [7 (2, 0)]	[8 (2, 1)]	[9 (2, 2)]

#[derive(Debug, Clone, PartialEq)]
pub struct GridCoordinate {
    q:usize,
    r:usize,
}

impl GridCoordinate {
    pub fn new(q:usize, r:usize) -> GridCoordinate {
        GridCoordinate{q, r}
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.q, self.r)
    }
}

#[derive(Debug)]
pub struct Tile {
    coord: GridCoordinate,
    s: String,
}

impl Tile {
    pub fn new(q:usize, r:usize) -> Tile {
        Tile{coord: GridCoordinate::new(q, r), s: String::new()}
    }

    pub fn to_string(&self) -> String {
        format!("[{} {}]", self.s.as_str(), self.coord.to_string())
    }

    pub fn label_at(&self, q:i32, r:i32) -> &String {
        &self.s
    }
}

pub fn run() {
    println!("AoC 2024 Day 2");

    let filename = "input/day04.txt";
    // let filename = "./test_input/day04-test.txt";

    let input = std::fs::read_to_string(filename)
        .expect("Failed to read input file for Day 04");

    // Cast the input as a Vector<String> with leading and trailing
    // whitespace trimmed, or as best suites each puzzle
    let input_vec = Vec::from(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
    );

    // Debug:  Visually validate the puzzle input: Check for missing first and/or last row, etc!
    // dbg!(&input);
    // Debug:  Visually validate the puzzle input: Check for missing first and/or last row, etc!
    println!("Raw input: {:?}", input);
    println!("Input as Vec<String>:\n{}", input_vec.join("\n"));
    println!("End input inspection\n");


    // Track program runtime by elapsed time shown by a "clock on the wall"
    let stop_watch = Instant::now();

    //////////
    // Part 1
    //////////

    // Grid capacity is known
    let rows = input_vec.len();
    let cols = rows;

    // Create and initalize a grid for the word search
    let mut word_search: Vec<Vec<Tile>> = (0..rows)
        .map(|q| (0..cols).map(|c| Tile{ coord:GridCoordinate{q:q,r:c}, s:".".to_string()}).collect())
        .collect();

    // Iterate over the input vector of String and populate the word search grid
    let mut q = 0;
    for (row, line) in input_vec.iter().enumerate() {
        // println!("Index in keypad number {}:", row + 1); // Row number (1-based)

        // Iterate over each character in the string
        let mut r = 0;
        for ch in line.chars() {
            word_search[q][r].s = ch.to_string();
            r += 1;
        }
        q += 1;
    }

    // validate visually that it is correct
    for row in &word_search {
        println!("{}", row.iter().map(|tile| tile.to_string()).collect::<Vec<String>>().join(" "));
    }

    // Should be straightforward to find all the "XMAS" strings.
    let mut cnt = 0;
    for q in 0..rows {
        for r in 0..cols {
            let mut xmas_this_loop = 0;
            if  word_search[q][r].s.to_string() == "X".to_string() {
                // cnt += count_xmas(&word_search, q as i32, r as i32);
                // Right
                if r + 3 < cols {
                    if word_search[q][r+1].s.to_string() == "M".to_string()
                        && word_search[q][r+2].s.to_string() == "A".to_string()
                        && word_search[q][r+3].s.to_string() == "S".to_string() {
                        cnt += 1;
                        xmas_this_loop += 1;
                    }
                }
                // Left
                let r1 = r as i32; // type restriction requires cast to i32
                if r1-3 >= 0 {
                    if word_search[q][r - 1].s.to_string() == "M".to_string()
                        && word_search[q][r - 2].s.to_string() == "A".to_string()
                        && word_search[q][r - 3].s.to_string() == "S".to_string() {
                        cnt += 1;
                        xmas_this_loop += 1;
                    }
                }
                // Up
                let q1 = q as i32;
                if q1-3 >= 0 {
                    if word_search[q-1][r].s.to_string() == "M".to_string()
                        && word_search[q-2][r].s.to_string() == "A".to_string()
                        && word_search[q-3][r].s.to_string() == "S".to_string() {
                        cnt += 1;
                        xmas_this_loop += 1;
                    }
                }
                // Down
                if q+3 < cols {
                    if word_search[q+1][r].s.to_string() == "M".to_string()
                        && word_search[q+2][r].s.to_string() == "A".to_string()
                        && word_search[q+3][r].s.to_string() == "S".to_string() {
                        cnt += 1;
                        xmas_this_loop += 1;
                    }
                }
                // Right-Down diagonal
                if r + 3 < cols && q + 3 < rows {
                    if word_search[q+1][r+1].s.to_string() == "M".to_string()
                        && word_search[q+2][r+2].s.to_string() == "A".to_string()
                        && word_search[q+3][r+3].s.to_string() == "S".to_string() {
                        cnt += 1;
                        xmas_this_loop += 1;
                    }
                }
                // Left-Down diagonal
                let r1 = r1 as i32;
                if q+3 < cols && r1 - 3 >= 0 {
                    if word_search[q+1][r-1].s.to_string() == "M".to_string()
                        && word_search[q+2][r-2].s.to_string() == "A".to_string()
                        && word_search[q+3][r-3].s.to_string() == "S".to_string() {
                        cnt += 1;
                        xmas_this_loop += 1;
                    }
                }
                // Right-Up diagonal
                let q1 = q1 as i32;
                if r + 3 < cols && q1 - 3 >= 0 {
                    if word_search[q-1][r+1].s.to_string() == "M".to_string()
                        && word_search[q-2][r+2].s.to_string() == "A".to_string()
                        && word_search[q-3][r+3].s.to_string() == "S".to_string() {
                        cnt += 1;
                        xmas_this_loop += 1;
                    }
                }
                // Left-Up diagonal
                let q1 = q as i32;
                let r1 = r as i32;
                if q1-3 >= 0 && r1 - 3 >= 0 {
                    if word_search[q-1][r-1].s.to_string() == "M".to_string()
                        && word_search[q-2][r-2].s.to_string() == "A".to_string()
                        && word_search[q-3][r-3].s.to_string() == "S".to_string() {
                        cnt += 1;
                        xmas_this_loop += 1;
                    }
                }
                println!("Found at {:?}, N: {}, Total: {}", GridCoordinate{q:q, r:r}, xmas_this_loop, cnt);
            }
        }
    }


    let answer_p1 = cnt;
    println!("Part 1.  answer...  {answer_p1}");
    let lap1 = stop_watch.elapsed();
    println!("Elapsed time part 1: {:.2?}", lap1);
    println!();


    //////////
    // Part 2
    //////////

    let answer_p2 = 0;
    println!("Part 2. answer ... {answer_p2}");
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed()-lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}

fn count_xmas(ws: &Vec<Vec<Tile>>, q: i32, r: i32) -> i32 {

    // Right

    1
}

