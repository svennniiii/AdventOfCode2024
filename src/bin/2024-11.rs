use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn blink(stone: u64, blinks: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    // Base case: If no blinks left, this stone counts as 1
    if blinks == 0 {
        return 1;
    }

    // Check cache
    if let Some(&count) = cache.get(&(stone, blinks)) {
        return count;
    }

    let result = if stone == 0 {
        blink(1, blinks - 1, cache)
    } else {
        // Calculate number of digits using log10
        // (stone.ilog10() returns 0 for digits 1-9, 1 for 10-99, etc.)
        let digits = stone.ilog10() + 1;
        
        if digits % 2 == 0 {
            let divisor = 10_u64.pow(digits / 2);
            let left = stone / divisor;
            let right = stone % divisor;
            
            blink(left, blinks - 1, cache) + blink(right, blinks - 1, cache)
        } else {
            blink(stone * 2024, blinks - 1, cache)
        }
    };

    // Store in cache
    cache.insert((stone, blinks), result);
    result
}

fn main() {
    let input = fs::read_to_string("data/2024/11/input.txt")
        .expect("Should have been able to read the file");
    
    let start_time = Instant::now();

    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|n| n.parse().expect("Invalid number"))
        .collect();

    let mut cache = HashMap::new();

    // Part 1
    let result_p1: u64 = stones.iter()
        .map(|&stone| blink(stone, 25, &mut cache))
        .sum();

    // Part 2
    // Keep the same cache!
    let result_p2: u64 = stones.iter()
        .map(|&stone| blink(stone, 75, &mut cache))
        .sum();

    println!("Part 1 Answer: {}", result_p1);
    println!("Part 2 Answer: {}", result_p2);
    println!("Cache Size: {}", cache.len());
    println!("Time: {:?}", start_time.elapsed());
}