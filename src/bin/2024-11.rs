use std::fs;
use std::collections::HashMap;
use std::time::Instant;

fn blink(stone: u64, blinks: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(&result) = cache.get(&(stone, blinks)) {
        return result;
    }

    let result: u64;

    let s = stone.to_string();
    if stone == 0 {
        result = blink(1, blinks - 1, cache);
    } else if s.len() % 2 == 0 {
        let mid = s.len() / 2;
        let (left_s, right_s) = s.split_at(mid);
        
        let left_val = left_s.parse().unwrap();
        let right_val = right_s.parse().unwrap();

        let left_count = blink(left_val, blinks - 1, cache);
        let right_count = blink(right_val, blinks - 1, cache);
        result = left_count + right_count;
    } else {
        result = blink(stone * 2024, blinks - 1, cache);
    }

    cache.insert((stone, blinks), result);

    result
}

fn main() {
    let input = fs::read_to_string("data/2024/11/input.txt")
        .expect("Should have been able to read the file");

    let stones: Vec<u64> = input
        .trim()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();
    let mut result_p1 = 0;
    for stone in stones.iter() {
        result_p1 += blink(*stone, 25, &mut cache);
    }

    let mut result_p2 = 0;
    for stone in stones.iter() {
        result_p2 += blink(*stone, 75, &mut cache);
    }

    let start_time = Instant::now();

    println!("Part 1 Answer: {}", result_p1);
    println!("Part 2 Answer: {}", result_p2);
    println!("Cache Size: {}", cache.len());
    println!("Time: {:?}", start_time.elapsed());
}