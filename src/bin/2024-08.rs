use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant; // Import the stopwatch

fn main() {
    let input = fs::read_to_string("data/2024/08/input.txt")
        .expect("Should have been able to read the file");

    let start_time = Instant::now();   

    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    // Parse (using row/col naming to avoid confusion)
    for (r, line) in lines.iter().enumerate() {
        for (c, freq) in line.chars().enumerate() {
            if freq != '.' {
                antennas
                    .entry(freq)
                    .or_default() // shorter than or_insert_with
                    .push((r as i32, c as i32));
            }
        }
    }

    let mut p1_antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut p2_antinodes: HashSet<(i32, i32)> = HashSet::new();

    // Helper to check bounds
    let in_bounds = |r, c| r >= 0 && r < height && c >= 0 && c < width;

    for coords in antennas.values() {
        // Iterate all permutations (A, B) and (B, A)
        for i in 0..coords.len() {
            for j in 0..coords.len() {
                if i == j { continue; }

                let (r1, c1) = coords[i];
                let (r2, c2) = coords[j];

                // Calculate distance vector
                let dr = r2 - r1;
                let dc = c2 - c1;

                // Part 1: Exactly one step away from r2
                let p1_r = r2 + dr;
                let p1_c = c2 + dc;
                if in_bounds(p1_r, p1_c) {
                    p1_antinodes.insert((p1_r, p1_c));
                }

                // Part 2: Keep stepping starting from the antenna itself
                let mut curr_r = r2;
                let mut curr_c = c2;
                
                // Add the antenna itself (distance 0) and all multiples
                while in_bounds(curr_r, curr_c) {
                    p2_antinodes.insert((curr_r, curr_c));
                    curr_r += dr;
                    curr_c += dc;
                }
            }
        }
    }

    // Stop timing
    let duration = start_time.elapsed();

    println!("Part 1 Answer: {}", p1_antinodes.len());
    println!("Part 2 Answer: {}", p2_antinodes.len());

    // Display the time
    println!("Total calculation time: {:?}", duration);
}