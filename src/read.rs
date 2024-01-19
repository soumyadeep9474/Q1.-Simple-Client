use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_read() {
    // Open the file for reading
    if let Ok(file) = File::open("cache_data.txt") {
        let reader = BufReader::new(file);

        // Read and print each line from the file
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    } else {
        eprintln!("Error: File not found or unable to read.");
    }
}
