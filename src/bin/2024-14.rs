use std::fs;
use std::time::Instant;

struct Position {
    x: i32,
    y: i32
}

struct Robot {
    pos: Position,
    vel: Position
}

impl Robot {
    fn move_step(&mut self, height: i32, width: i32) {
        self.pos.x = (self.pos.x + self.vel.x).rem_euclid(width);
        self.pos.y = (self.pos.y + self.vel.y).rem_euclid(height);
    }
}

fn main() {
    let input = fs::read_to_string("data/2024/14/input.txt")
        .expect("Should have been able to read the file");
    
    let start_time = Instant::now();

    let width = 101;
    let height = 103;

    let mut robots: Vec<Robot> = Vec::new();

    for line in input.trim().lines() {
        let (pos_str, vel_str) = line.trim().split_once(" ").unwrap();
        
        let (x_str, y_str) = pos_str[2..].split_once(',').unwrap();
        let pos = Position {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        };

        let (vx_str, vy_str) = vel_str[2..].split_once(',').unwrap();
        let vel = Position {
            x: vx_str.parse().unwrap(),
            y: vy_str.parse().unwrap(),
        };

        robots.push(Robot { pos, vel });
    }

    // Step 100 times
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.move_step(height, width);
        }
    }

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    let mid_x = width / 2;
    let mid_y = height / 2;

    for robot in robots.iter() {
        let x = robot.pos.x;
        let y = robot.pos.y;

        // Robots exactly on the middle lines are ignored per rules
        if x < mid_x && y < mid_y {
            q1 += 1;
        } else if x > mid_x && y < mid_y {
            q2 += 1;
        } else if x < mid_x && y > mid_y {
            q3 += 1;
        } else if x > mid_x && y > mid_y {
            q4 += 1;
        }
    }

    let safety_factor = q1 * q2 * q3 * q4;

    println!("Part 1 Answer: {}", safety_factor);
    println!("Time: {:?}", start_time.elapsed());
}