# Advent of Code 2024
My solutions to the Advent of Code programming contest, December 2024.

Advent of Code website:  [Advent of Code](https://adventofcode.com)

Solutions are in Rust lang v1.83+ (2021 Edition) unless otherwise noted.

> ⚠️ Puzzle descriptions and input files are not included due to copyright restrictions. Please visit the official Advent of Code website to view the original puzzles.

[Rust language cheat sheet](https://cheats.rs/)

## Usage
```bash
cargo run -- <day>
```
Example:
```bash
cargo run -- 1
```
This will run the solution for Day 1 using the input file `inputs/day01.txt`.

## Setup
```bash
cargo build
```
## Folder structure
The layout below shows the recommended / existing organization and how to find inputs,
shared utilities, and each day's solution.

```
/
├─ Cargo.toml                 # workspace or crate manifest
├─ README.md                  # this file
├─ input/                     # your puzzle inputs (one file per day)
│  ├─ day01.txt
│  ├─ day02.txt
│  └─ ...
├─ src/                       # shared library code and helpers (optional)
│  ├─ lib.rs                  # reusable functions used by multiple days
│  └─ utils/                  # small helper modules
│     └─ mod.rs
├─ src/bin/                   # per-day binaries (common layout for AoC)
│  ├─ day01.rs                # Day 1 solution (binary)
│  ├─ day02.rs                # Day 2 solution
│  └─ ...
├─ days/                      # alternative layout: per-day crates (workspace)
│  ├─ day01/
│  │  └─ src/
│  │     └─ main.rs
│  └─ ...
└─ target/                    # build output (ignored by git)
```

To run a solution use cargo and supply the day's script name:
```shell
cargo run --package advent-of-code-2024 --days day01
```

### Days which were straightforward to solve
1, 2 (p1)

### My stats at the end of the official contest
As of end of day 25 December 2024:
- 7 stars,
- and total of 6 with at least one part complete,
- and bailed out early and way behind my previous years.


### Notes

#### Day 1
- I should have paid more attention in Rust class.  Rust looked easier from the back of the room than it is to me when coding. 🙂
- I was studying a top AoC competitor's Rust code for Day 1 ( ["noah r."](https://github.com/50SACINMYSOCIDGAF/AdventOfCode2024) ) and found he used a "sort_unstable()" method call.  I'd never heard term unstable sort (that I recall) and had to look it up ([Stackoverflow](https://stackoverflow.com/questions/15125552/what-is-the-meaning-of-stable-and-unstable-for-various-sorting-algorithms)).  Sure enough the [Rust docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable) say it's faster.

#### Day 3
- Decided to build a Finite State Machine instead of a regex to parse the input.
- It was a win for Part 1, but took 3.5 hours to get it running.

#### Day 4
- Nifty way to replace chars in a string this using match expression. Link: [stackoverflow.com](https://stackoverflow.com/questions/34606043/how-do-i-replace-specific-characters-idiomatically-in-rust)
```rust
   let s:String = input.chars()
   .map(|x| match x {
   '\n' => ' ',
   _ => x
   }).collect();
```
- How to find overlapping matches.  Look ahead:  https://stackoverflow.com/questions/11430863/how-to-find-overlapping-matches-with-a-regexp
- DNF - TODO
- November 2025, re-do w/new code warm-up for 2025

#### Day 5
- DNF

#### Day 6
- Informal way to compute elapsed time link: [Stackoveflow](https://stackoverflow.com/questions/13322479/how-to-benchmark-programs-in-rust)
- Used straightforward implementation representing grid as nested vector.
- Got help laying out grid's implementation with origin top-left and referencing the data 
as the familiar (row, column) or (x,y)  style Cartestion coords 
from [Stackoverflow](https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust)

#### Day 7
- Implemented as a tree structure.
- I'll mark this one to come back to and study this one
in January.  I should be able to dd a better job with it.  
I carried value of each node forward, and added to a list of results when I reach leaf/deepest point in each path.
- Part 2 TODO

#### Day 8
- How to idiomatically &quot;upsert&quot; to a mut Rust HashMap  [Stackoverflow](https://stackoverflow.com/questions/33243784/append-to-vector-as-value-of-hashmap)
- Use a HashSet to avoid duplicates

Acknowledgements
----------------
- * Advent of Code (https://adventofcode.com)
- * Rust language and community resources
- * [Amit Patel’s Thoughts on Grids](https://www.redblobgames.com/)

