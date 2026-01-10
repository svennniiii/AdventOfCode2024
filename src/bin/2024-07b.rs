use std::fs;
use std::time::Instant; // Import the stopwatch

fn equation_possible(test_value: u64, current_total: u64, remaining_values: &[u64], with_concat: bool) -> bool {
    // BASE CASE: If no numbers are left, check if our total matches the target
    if remaining_values.is_empty() {
        return current_total == test_value;
    }

    // PRUNING: If our total is already too big, stop looking 
    // (This works because all operators increase the number)
    if current_total > test_value {
        return false;
    }

    let next_val = remaining_values[0];
    let next_remaining = &remaining_values[1..];

    // Try Addition
    if equation_possible(test_value, current_total + next_val, next_remaining, with_concat) {
        return true;
    }

    // Try Multiplication
    if equation_possible(test_value, current_total * next_val, next_remaining, with_concat) {
        return true;
    }

    // Try Concatenation (only if enabled)
    if with_concat {
        // Mathematical concatenation:
        // To join 12 and 345: 12 * 1000 + 345
        let mut multiplier = 10;
        while next_val >= multiplier {
            multiplier *= 10;
        }
        let concatenated = current_total * multiplier + next_val;
        
        if equation_possible(test_value, concatenated, next_remaining, with_concat) {
            return true;
        }
    }

    false
}

fn main() {
    // Using unwrap() is fine for Advent of Code scripts!
    let input = fs::read_to_string("data/2024/07/input.txt").expect("Failed to read file");
    
    // Start timing AFTER the file is read, so we only measure the logic
    let start_time = Instant::now();    

    let mut part1_total = 0;
    let mut part2_total = 0;

    for line in input.lines() {
        if line.is_empty() { continue; }

        let (target_str, nums_str) = line.split_once(": ").unwrap();
        let target: u64 = target_str.parse().unwrap();
        let numbers: Vec<u64> = nums_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // We start the recursion with the first number as our starting total
        // and the "rest" of the numbers as the remaining values.
        if equation_possible(target, numbers[0], &numbers[1..], false) {
            part1_total += target;
            part2_total += target;
        } else if equation_possible(target, numbers[0], &numbers[1..], true) {
            part2_total += target;
        }
    }

    // Stop timing
    let duration = start_time.elapsed();

    println!("Part 1: {}", part1_total);
    println!("Part 2: {}", part2_total);

    // Display the time
    println!("Total calculation time: {:?}", duration);
}