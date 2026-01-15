use std::fs;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
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

fn calculate_cost(start: &Coord, checked_coords: &mut HashSet<Coord>, farm: &Vec<Vec<char>>, height: usize, width: usize) -> u32 {
    let mut to_check: Vec<Coord> = Vec::new();
    to_check.push(*start);

    let mut fence = 0;
    let mut area = 0;
    let plant = farm[start.r as usize][start.c as usize];

    while !to_check.is_empty() {
        let coord = to_check.pop().unwrap();
        if checked_coords.contains(&coord){
            continue;
        }
        //if farm[coord.r as usize][coord.c as usize] != plant {
        //    continue;
        //}

        checked_coords.insert(coord);
        
        area += 1;
        fence += 4;

        for neighbor in coord.neighbors() {
            let Coord{c: nc, r:nr} = neighbor;
            if nr < 0 || nr >= height as isize {
                continue;
            } else if nc < 0 || nc >= width as isize {
                continue;
            } else if farm[nr as usize][nc as usize] != plant {
                continue;
            }

            fence -= 1;

            if !checked_coords.contains(&neighbor) {
                to_check.push(neighbor);
            }
        }

    }
    println!("{} {} {}", plant, area, fence);
    area * fence
}

fn main() {
    let input = fs::read_to_string("data/2024/12/input.txt")
        .expect("Should have been able to read the file");
    
    let farm: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
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

            cost += calculate_cost(&current_coord, &mut checked_coords, &farm, height, width);
        }
    }

    let start_time = Instant::now();

    println!("Part 1 Answer: {}", cost);
    println!("Part 2 Answer: {}", 0);
    println!("Time: {:?}", start_time.elapsed());
}