# Advent of Code 2024
My solutions to the Advent of Code programming contest, December 2024.

Advent of Code website:  [Advent of Code](https://adventofcode.com)

Solutions are in Rust lang v1.83+ unless otherwise noted.

To run a solution use cargo and supply the day's script name:
```shell
cargo run --package advent-of-code-2024 --bin day01
```

1. --- Day 1: Historian Hysteria ---
2. --- Day 2: Red-Nosed Reports --- 
3. --- Day 3: Mull It Over --- 
4. --- Day 4: Ceres Search ---
5. --- Day 5: Print Queue --- 
6. --- Day 6: Guard Gallivant ---
7. Day  7:  
8. Day  8:  
9. Day  9:  
10. Day 10: 
11. Day 11: 
12. Day 12: 
13. Day 13: 
14. Day 14: 
15. Day 15: 

### Days which were straightforward to solve
1, 2 (p1)

### My stats at the end of the official contest
TBD

### Notes

#### Day 1
- [Project structure suggestions from Reddit](https://www.reddit.com/r/adventofcode/comments/zikosa/how_to_organize_rust_code_for_advent_of_code/)
- I should have paid more attention in Rust class.  Rust looked easier from the back of the room than it is to me when coding. 🙂
- I was studying a top AoC competitor's Rust code for Day 1 ( ["noah r."](https://github.com/50SACINMYSOCIDGAF/AdventOfCode2024) ) and found he used a "sort_unstable()" method call.  I'd never heard term unstable sort (that I recall) and had to look it up ([Stackoverflow](https://stackoverflow.com/questions/15125552/what-is-the-meaning-of-stable-and-unstable-for-various-sorting-algorithms)).  Sure enough the [Rust docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable) say it's faster.

```text
Day 01 Part 1.  What is the total distance between your lists? answer: 1941353
Day 01 Part 2.   What is their similarity score?  answer: 22539317
End.  Current date and time (UTC): 2024-12-01 18:02:49
```

#### Day 2
- Part 1 straightforward
- Part 2 DNF

```text
Day 02 Part 1.  How many reports are safe?  572
End.  Current date and time (UTC): 2024-12-04 01:34:03
```

#### Day 3
- Decided to build a Finite State Machine instead of a regex to parse the input.
- It was a win for Part 1, but took 3.5 hours to get it running.
```text
Day 03 Part 1.  What do you get if you add up all of the results of the multiplications? 178886550
Day 03 Part 2.  ... DNF on same day
End.  Current date and time (UTC): 2024-12-04 04:41:42
```

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
- 