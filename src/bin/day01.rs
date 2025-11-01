use std::collections::HashMap;
use advent_of_code_2024::read_contents_buffered;
use chrono::Utc;

fn main() {

    println!("--- Advent of Code 2024 ---");
    println!("--- Day 1: Historian Hysteria ---\n");

    // Reading buffered file contents into a string line by line
    let filename = "./input/day01.txt";
    //let filename = "./test_input/day01-test.txt";

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

    let mut xs : Vec<i32> = Vec::new();
    let mut ys : Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        xs.push(iter.next().unwrap().parse::<i32>().unwrap());
        ys.push(iter.next().unwrap().parse::<i32>().unwrap());
        //dbg!(&xs, &ys);
    }

    // dbg!(&xs);
    // dbg!(&ys);

    // Part 1

    xs.sort();
    ys.sort();
    let zs = xs.iter()
        .zip(&ys)
        .collect::<Vec<_>>();
    // dbg!(&zs);

    // Note to self:  watch out for the silent integer overflow.
    let answer_p1 = zs.iter()
        .map(|&z| (z.0 - z.1).abs())
        .sum::<i32>();

    println!("What is the total distance between your lists?");
    println!("Day 01 Part 1 answer: {answer_p1}");  // 1941353


    // Part 2
    // for every number in the xs count how many times it appears in ys
    // then multiply the two numbers together and collect the sum the results of each calculation

    // Create a vector of results of counting how many times each value in xs appears in ys
    let rs = xs
        .iter()
        .map(|x| {
            ys.iter().filter(|&y| *y == *x ).count() as i32
        })
        .collect::<Vec<i32>>();
    // dbg!(&rs);

    // answer to Part 2 is sum of the product of each value in xs by its corresponding value in rs
    let answer_p2 = xs.iter().zip(rs).map(|xr| xr.0 * xr.1).sum::<i32>();

    println!("What is their similarity score?");
    println!("Day 01 Part 2 answer: {answer_p2}");  // 22539317

    // End
    let current_datetime = Utc::now();
    println!("End.  Current date and time (UTC): {}", current_datetime.format("%Y-%m-%d %H:%M:%S"));


    // *****
    // Errata
    // for study
    // noah r.'s Day 1 solution:  https://github.com/50SACINMYSOCIDGAF/AdventOfCode2024

    let mut left_numbers: Vec<i64> = Vec::new();
    let mut right_numbers: Vec<i64> = Vec::new();
    // Parse input into two vectors
    for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        dbg!(&numbers);

        if numbers.len() == 2 {
            left_numbers.push(numbers[0]);
            right_numbers.push(numbers[1]);
        }
    }

    // Part 1: Calculate total distance
    let mut left_sorted = left_numbers.clone();
    let mut right_sorted = right_numbers.clone();
    left_sorted.sort_unstable();
    right_sorted.sort_unstable();

    let total_distance: i64 = left_sorted.iter()
        .zip(right_sorted.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    // Part 2: Calculate similarity score
    let mut right_counts: HashMap<i64, i64> = HashMap::new();
    for num in &right_numbers {
        *right_counts.entry(*num).or_insert(0) += 1;
    }

    let similarity_score: i64 = left_numbers.iter()
        .map(|num| num * right_counts.get(num).unwrap_or(&0))
        .sum();

    let _asdf = 1;


}
