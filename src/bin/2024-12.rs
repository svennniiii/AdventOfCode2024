use std::fs;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::atomic::AtomicIsize;
use std::time::Instant;

#[derive(Hash, Eq, PartialEq)]
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

fn calculate_cost(start: &Coord, checked_coords: &mut HashSet<Coord>) -> u32 {
    // Implement flood fill for each plot
    // keep track of fence

    return 0;
}

fn main() {
    let input = fs::read_to_string("data/2024/12/example.txt")
        .expect("Should have been able to read the file");
    
    let farm: Vec<&str> = input.trim().lines().collect();
    let mut checked_coords: HashSet<Coord> = HashSet::new();

    let height = farm.len();
    let width = farm[0].len();

    let mut cost = 0;

    for row in 0..height {
        for column in 0..width {
            let current_coord = Coord{r: row as isize, c: column as isize};
            if checked_coords.contains(&current_coord) {
                continue;
            }

            cost += calculate_cost(&current_coord, &mut checked_coords);
        }
    }

    let start_time = Instant::now();

    println!("Part 1 Answer: {}", cost);
    println!("Part 2 Answer: {}", 0);
    println!("Time: {:?}", start_time.elapsed());
}