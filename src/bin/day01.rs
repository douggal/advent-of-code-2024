use advent_of_code_2024::read_contents_buffered;

fn main() {

    println!("Advent of Code 2024");
    println!("Day 01");

    // Reading buffered file contents into a string line by line
    let filename = "./input/day01.txt";
    //let filename = "./test_input/day01_test.txt";

    println!("Reading input file, filename = {}", filename);
    let input = match read_contents_buffered(filename) {
        Ok(file_contents) => {
            println!("Read input file contents successfully!\n\n {}", file_contents);
            file_contents
        },
        Err(err) => {
            print!("Error reading input file contents: {:?}\nn", err);
            panic!();
        }
    };

    let line = input.trim();
    let seq = line.split(", ").collect::<Vec<&str>>();

    dbg!(seq);

}
