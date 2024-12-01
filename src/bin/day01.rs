use advent_of_code_2024::read_contents_buffered;

fn main() {

    println!("Advent of Code 2024");
    println!("--- Day 1: Historian Hysteria ---");

    // Reading buffered file contents into a string line by line
    let filename = "./input/day01.txt";
    //let filename = "./test_input/day01-test.txt";

    println!("Reading input file, filename = {}", filename);
    let input = match read_contents_buffered(filename) {
        Ok(file_contents) => {
            println!("Read input file contents successfully!\n\n {}", file_contents);
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

    xs.sort();
    ys.sort();
    let zs = xs.iter().zip(ys).collect::<Vec<_>>();
    // dbg!(&zs);
    let ds = zs.iter().map(|&z| (z.0 - z.1).abs()).collect::<Vec<i32>>();
    // dbg!(&ds);
    let sum = zs.iter().map(|&z| (z.0 - z.1).abs()).sum::<i32>();

    println!("Day 01 answer {sum}");

}
