# Advent of Code 2024
My solutions to the Advent of Code programming contest, December 2024.

Advent of Code website:  [Advent of Code](https://adventofcode.com)

Solutions are in Rust lang v1.83+ (2021 Edition) unless otherwise noted.

> ⚠️ Puzzle descriptions and input files are not included due to copyright restrictions. Please visit the official Advent of Code website to view the original puzzles.

[Rust language cheat sheet](https://cheats.rs/)

To run a solution use cargo and supply the day's script name:
```shell
cargo run --package advent-of-code-2024 --bin day01
```

1. [Day 1...](https://adventofcode.com/2024/day/1)
2. [Day 1...](https://adventofcode.com/2024/day/2)
3. [Day 1...](https://adventofcode.com/2024/day/3)
4. [Day 1...](https://adventofcode.com/2024/day/4)
5. [Day 1...](https://adventofcode.com/2024/day/5)
6. [Day 1...](https://adventofcode.com/2024/day/6)
7. [Day 1...](https://adventofcode.com/2024/day/7)
8. [Day 1...](https://adventofcode.com/2024/day/8)
9. ...

### Days which were straightforward to solve
1, 2 (p1)

### My stats at the end of the official contest
As of end of day 25 December 2024:
- 7 stars,
- and total of 6 with at least one part complete,
- and bailed out early and way behind my previous years.


### Notes

#### Day 1
- [Project structure suggestions from Reddit](https://www.reddit.com/r/adventofcode/comments/zikosa/how_to_organize_rust_code_for_advent_of_code/)
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
