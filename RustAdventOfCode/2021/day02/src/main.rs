//use std::time::Instant;
use std::fs;

fn part1(numbers: Vec<(String, i32)>) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;

    for number in numbers.iter() {
        if number.0 == "down" {
            depth += number.1;
        } else if number.0 == "up" {
            depth -= number.1;
        } else if number.0 == "forward" {
            horizontal += number.1;
        }
    }
    return depth*horizontal;
}

fn part2(numbers: Vec<(String, i32)>) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for number in numbers.iter() {
        if number.0 == "down" {
            aim += number.1;
        } else if number.0 == "up" {
            aim -= number.1;
        } else if number.0 == "forward" {
            horizontal += number.1;
        }
    }
    return depth*horizontal;
}

fn main(){
    //let start = Instant::now();
    
    // Read the values in input file line by line
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    // Split the contents by new line
    let lines: Vec<&str> = contents.split("\n").collect();
    println!("{:?}", lines);
    // Lines are "String Integer" pairs
    let mut numbers: Vec<(String, i32)> = Vec::new();
    for line in lines.iter() {
        let pair: Vec<&str> = line.split(" ").collect();
        numbers.push((pair[0].to_string(), pair[1].parse::<i32>().unwrap()));
    }
    println!("{:?}", numbers);

    // Part 1
    let result1 = part1(numbers);

    // Part 2



    // Print the result
    //println!("Part 1: {}\nPart 2: {}\nTime taken: {:?} microseconds", result1, increased_sums, start.elapsed().as_micros());
}