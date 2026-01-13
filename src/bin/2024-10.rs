use std::fs;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Coord {
    r: isize,
    c: isize,
}

impl Coord {
    fn neighbors(&self) -> [Coord; 4] {
        [
            Coord { r: self.r - 1, c: self.c }, // Up
            Coord { r: self.r, c: self.c + 1 }, // Right
            Coord { r: self.r + 1, c: self.c }, // Down
            Coord { r: self.r, c: self.c - 1 }, // Left
        ]
    }
}

fn main() {
    let input = fs::read_to_string("data/2024/10/input.txt")
        .expect("Should have been able to read the file");

    let start_time = Instant::now();

    // Parse grid
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    // Find Trailheads
    let mut trailheads = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            if val == 0 {
                trailheads.push(Coord { r: r as isize, c: c as isize });
            }
        }
    }

    let mut score_sum = 0;

    for start in trailheads {
        let mut current_positions: HashSet<Coord> = HashSet::new();
        current_positions.insert(start);

        for target_height in 1..=9 {
            let mut next_positions: HashSet<Coord> = HashSet::new();

            for pos in current_positions {
                for next in pos.neighbors() {
                    // Check bounds
                    if next.r < 0 || next.r >= height || next.c < 0 || next.c >= width {
                        continue;
                    }
                    // Check height requirement
                    if grid[next.r as usize][next.c as usize] == target_height {
                        next_positions.insert(next);
                    }
                }
            }
            // Move forward
            current_positions = next_positions;
        }
        score_sum += current_positions.len();
    }

    println!("Part 1 Answer: {}", score_sum);
    println!("Time: {:?}", start_time.elapsed());
}