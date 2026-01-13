use std::fs;
use std::collections::{HashSet,HashMap};
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
    // A helper to filter valid moves
    fn valid_neighbors(&self, grid: &[Vec<u8>], height: isize, width: isize, current_h: u8) -> impl Iterator<Item = Coord> {
        self.neighbors().into_iter().filter(move |n| {
            n.r >= 0 && n.r < height && 
            n.c >= 0 && n.c < width && 
            grid[n.r as usize][n.c as usize] == current_h + 1
        })
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

    let mut score_sum: usize = 0;

    for start in trailheads.iter() {
        let mut current_positions: HashSet<Coord> = HashSet::from([*start]);

        for target_height in 0..=8 {
            let mut next_positions: HashSet<Coord> = HashSet::new();

            for pos in current_positions {
                for next in pos.valid_neighbors(&grid, height, width, target_height) {
                    next_positions.insert(next);
                }
            }
            // Move forward
            current_positions = next_positions;
        }
        score_sum += current_positions.len();
    }

    let mut rating_sum = 0;    

    for start in trailheads.iter() {
        let mut current_positions: HashMap<Coord, u32> = HashMap::from([(*start, 1)]);
 
        for target_height in 1..=9 {
            let mut next_positions: HashMap<Coord, u32>  = HashMap::new();

            for (pos, count) in current_positions {
                for next in pos.valid_neighbors(&grid, height, width, target_height) {                    
                    *next_positions.entry(next).or_insert(0) += count;                    
                }
            }
            // Move forward
            current_positions = next_positions;
        }

        rating_sum += current_positions.values().sum::<u32>() as usize;
    }

    println!("Part 1 Answer: {}", score_sum);
    println!("Part 2 Answer: {}", rating_sum);
    println!("Time: {:?}", start_time.elapsed());
}