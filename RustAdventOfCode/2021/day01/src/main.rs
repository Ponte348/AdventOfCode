use std::time::Instant;
use std::fs;

fn main(){
    let start = Instant::now();
    
    // Read the values in input file line by line
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    // Split the contents by new line
    let lines: Vec<&str> = contents.split("\n").collect();
    // Convert the lines to integers
    let numbers: Vec<i32> = lines.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    // Print Vector
    // println!("{:?}", numbers);


    // Part 1
    let mut increased = 0;
    // Infinite number
    let mut first = i32::MAX;

    // Loop through the numbers
    // For every number, check if the previous number is less than the current number
    for number in numbers.iter() {
        if *number > first {
            increased += 1;
        }
        first = *number;
    }

    // Print the result
    println!("Part 1: {}\nTime: {:?} microseconds", increased, start.elapsed().as_micros());
}
