use std::fs;
use std::time::Instant;

fn calculate_minimum_cost(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64, part_1: bool) -> Option<i64> {
    let denom = ax * by - ay * bx;

    // If determinant is 0, lines are parallel. 
    if denom == 0 {
        return None; 
    }

    // Numerator for A: Px*By - Py*Bx
    let nom_a = px * by - py * bx;
    // Numerator for B: Ax*Py - Ay*Px
    let nom_b = ax * py - ay * px;

    if nom_a % denom != 0 || nom_b % denom != 0 {
        return None;
    }

    let a = nom_a / denom;
    let b = nom_b / denom;

    if a < 0 || b < 0 {
        return None;
    }

    // Part 1 limitation: buttons can't be pressed more than 100 times
    if part_1 && (a > 100 || b > 100) {
        return None;
    }

    Some(3 * a + b)
}

fn main() {
    let input = fs::read_to_string("data/2024/13/input.txt")
        .expect("Should have been able to read the file");
    
    let start_time = Instant::now();

    let machines_str: Vec<&str> = input.trim().split("\n\n").collect();

    let mut lowest_cost_p1 = 0;
    let mut lowest_cost_p2 = 0;

    for machine in machines_str {
         let nums: Vec<i64> = machine
            .split(|c: char| !c.is_numeric()) 
            .filter(|s| !s.is_empty())        
            .map(|s| s.parse::<i64>().unwrap()) 
            .collect();                       

        let ax = nums[0];
        let ay = nums[1];
        let bx = nums[2];
        let by = nums[3];
        let px = nums[4];
        let py = nums[5];

        if let Some(cost) = calculate_minimum_cost(ax, ay, bx, by, px, py, true) {
            lowest_cost_p1 += cost;
        }

        if let Some(cost) = calculate_minimum_cost(ax, ay, bx, by, px + 10_000_000_000_000, py + 10_000_000_000_000, false) {
            lowest_cost_p2 += cost;
        }
    }

    println!("Part 1 Answer: {}", lowest_cost_p1);
    println!("Part 2 Answer: {}", lowest_cost_p2);
    println!("Time: {:?}", start_time.elapsed());
}