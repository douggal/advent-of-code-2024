use std::fs::{File};
use std::io::{Error, BufReader, BufRead};

pub fn read_contents_buffered(path: &str) -> Result<String, Error> {
    let mut file_txt = String::new();
    let readme = File::open(path)?;

    let buffer = BufReader::new(readme);

    for maybe_line in buffer.lines() {
        file_txt.push_str(maybe_line?.as_str());
        file_txt.push('\n');   // Add newline !!!
    }

    Ok(file_txt)
}
