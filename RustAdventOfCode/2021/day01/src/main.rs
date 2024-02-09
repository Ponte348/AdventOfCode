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
    // Maximum value of i32
    let mut first = i32::MAX;

    // For every number, check if the previous number is less than the current number
    for number in numbers.iter() {
        if *number > first {
            increased += 1;
        }
        first = *number;
    }


    // Part 2
    let mut increased_sums = 0;
    // For every vector[i]+vector[i-1]+vector[i-2] > vector[i-1]+vector[i-2]+vector[i-3]
    // If it is, add 1 to the increased variable
    for i in 3..numbers.len() {
        if numbers[i] + numbers[i-1] + numbers[i-2] > numbers[i-1] + numbers[i-2] + numbers[i-3] {
            increased_sums += 1;
        }
    }

    // Print the result
    println!("Part 1: {}\nPart 2: {}\nTime taken: {:?} microseconds", increased, increased_sums, start.elapsed().as_micros());

}
