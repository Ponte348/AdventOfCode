use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
    
    // Read the values in input file line by line
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    // Split the contents by new line
    let lines: Vec<&str> = contents.split("\n").collect();
    println!("{:?}", lines); 

    println!("Part 1: {}\nPart 2: {}\nTime taken: {:?} microseconds", 0, 0, start.elapsed().as_micros());
}
