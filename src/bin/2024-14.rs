use std::fs;
use std::time::Instant;

use std::thread;
use std::time::Duration;

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

fn print_robots(height: usize, width: usize, robots: &Vec<Robot>) {
    let mut map: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for robot in robots {
        let Position{x, y} = robot.pos;
        map[y as usize][x as usize] = true;
    }

    for row in map.iter() {
        for c in row {
            if *c == true {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
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

    let mut min_safety_factor = 224969976;
    
    for i in 0..100000 {
         for robot in robots.iter_mut() {
            robot.move_step(height, width);
        }

        q1 = 0;
        q2 = 0;
        q3 = 0;
        q4 = 0;

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

        if safety_factor < min_safety_factor {
            println!("{}:", i+100+1);
            print_robots(height as usize, width as usize, &robots);
            thread::sleep(Duration::from_millis(1000));  // Sleep for 500 milliseconds
            min_safety_factor = safety_factor;
        }
    }

    println!("Part 1 Answer: {}", safety_factor);
    println!("Part 2 Answer: {}", safety_factor);
    println!("Time: {:?}", start_time.elapsed());
}