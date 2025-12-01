#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use chrono::Utc;
/// Advent of Code 2024
/// December 2024
/// https://adventofcode.com/2024

use clap::Parser;
mod days;

#[derive(Parser)]
struct Args {
    #[arg(help = "Day number (1-25)")]
    day: u8,
}

fn main() {
    println!("Advent of Code 2024");
    // Start
    let start_datetime = Utc::now();
    println!(
        "Start date and time (UTC): {}",
        start_datetime.format("%Y-%m-%d %H:%M:%S")
    );

    // Read day number from command line.
    let args = Args::parse();

    match args.day {
        1 => days::day01::run(),
        2 => days::day02::run(),
        3 => days::day03::run(),
        //4 => days::day04::run(),
        4 => days::day04_v2::run(),
        // 5 => days::day05::run(),
        6 => days::day06::run(),
        7 => days::day07::run(),
        _ => eprintln!("Day must be between 1 and 25."),
    }

    // End
    println!();
    println!(
        "End date and time (UTC): {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S")
    );
}
