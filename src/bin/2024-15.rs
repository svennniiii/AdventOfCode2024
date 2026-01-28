use core::panic;
use std::fs;
use std::hash::Hash;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column: usize
}

impl Position {
    fn move_step(&self, direction: char) -> Position {
        match direction {
            'v' => Position { row: self.row + 1, column: self.column     },
            '<' => Position { row: self.row    , column: self.column - 1 },
            '^' => Position { row: self.row - 1, column: self.column     },
            '>' => Position { row: self.row    , column: self.column + 1 },
            _ => panic!("Invalid direction.")
        }
    }
}

fn calculate_coordinates(warehouse: & Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for (ri, row) in warehouse.iter().enumerate() {
        for (ci, c) in row.iter().enumerate() {
            if *c == 'O' || *c == '[' {
                sum += 100 * ri + ci;
            }
        }
    }
    return sum;
}

fn main() {
    let input = fs::read_to_string("data/2024/15/input.txt")
        .expect("Should have been able to read the file");
    let start_time = Instant::now();

    let (warehouse_str, movements_str) = input.trim().split_once("\n\n").unwrap();

    let mut robot = Position{row: 0 ,column: 0 };
    let mut warehouse: Vec<Vec<char>> = Vec::new();

    // Part 1
    for (row_index, line) in warehouse_str.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (column_index, c) in line.trim().chars().enumerate() {
            match c {
                '@' => {
                    robot = Position { row: row_index, column: column_index };
                    row.push('.');
                }
                _ => {row.push(c);}
            }
        }

        warehouse.push(row);
    }

    for direction in movements_str.chars() {
        if direction.is_whitespace() {
            continue;
        }

        let new_pos = robot.move_step(direction);
        let mut check_pos = new_pos;
        let mut skip_movement: bool = false;
        while warehouse[check_pos.row][check_pos.column] != '.' {
            if warehouse[check_pos.row][check_pos.column] == '#' {
                skip_movement = true;
                break;
            }

            check_pos = check_pos.move_step(direction);            
        }

        if skip_movement == true {
            continue;
        }

        if warehouse[new_pos.row][new_pos.column] == 'O' {
            warehouse[new_pos.row][new_pos.column] = '.';
            warehouse[check_pos.row][check_pos.column] = 'O';
        }
        robot = new_pos;
    }

    let coordinates_p1 = calculate_coordinates(&warehouse); 

    // Part 2
    warehouse.clear();
    for (row_index, line) in warehouse_str.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.trim().chars() {
            match ch {
                '@' => {
                    robot = Position { row: row_index, column: row.len() };
                    row.push('.');
                    row.push('.');
                }
                '#' => { row.push('#'); row.push('#'); }
                'O' => { row.push('['); row.push(']'); }
                '.' => { row.push('.'); row.push('.'); }
                _ => panic!("Unknown character: {}", ch)
            }
        }
        warehouse.push(row);
    }

    for direction in movements_str.chars() {
        if direction.is_whitespace() { continue; }

        let next_pos = robot.move_step(direction);
        let char_at_next = warehouse[next_pos.row][next_pos.column];

        // Case 1: Hit a wall immediately
        if char_at_next == '#' {
            continue;
        }

        // Case 2: Empty space
        if char_at_next == '.' {
            robot = next_pos;
            continue;
        }

        // Case 3: Hit a box ('[' or ']')
        let mut to_check: Vec<Position> = Vec::new();
        let mut boxes_to_move: Vec<Position> = Vec::new();
        
        to_check.push(next_pos);
        
        if direction == '^' || direction == 'v' {
             if char_at_next == '[' {
                 to_check.push(Position{ row: next_pos.row, column: next_pos.column + 1});
             } else if char_at_next == ']' {
                 to_check.push(Position{ row: next_pos.row, column: next_pos.column - 1});
             }
        }

        let mut blocked = false;

        // Process the queue
        let mut i = 0;
        while i < to_check.len() {
            let pos = to_check[i];
            i += 1;

            let tile = warehouse[pos.row][pos.column];

            if tile == '#' {
                blocked = true;
                break;
            }
            if tile == '.' {
                continue;
            }

            if !boxes_to_move.contains(&pos) {
                boxes_to_move.push(pos);
            }

            let target_pos = pos.move_step(direction);

            if !to_check.contains(&target_pos) {
                to_check.push(target_pos);
            }

            if direction == '^' || direction == 'v' {
                 let target_tile = warehouse[target_pos.row][target_pos.column];
                 
                 if target_tile == '[' {
                     let right_half = Position{ row: target_pos.row, column: target_pos.column + 1};
                     if !to_check.contains(&right_half) { to_check.push(right_half); }
                 } else if target_tile == ']' {
                     let left_half = Position{ row: target_pos.row, column: target_pos.column - 1};
                     if !to_check.contains(&left_half) { to_check.push(left_half); }
                 }
            }
        }

        if blocked {
            continue;
        }

        let mut old_values: Vec<char> = Vec::new();
        for pos in &boxes_to_move {
            old_values.push(warehouse[pos.row][pos.column]);
        }
        
        for pos in &boxes_to_move {
            warehouse[pos.row][pos.column] = '.';
        }

        for (idx, pos) in boxes_to_move.iter().enumerate() {
            let new_pos = pos.move_step(direction);
            warehouse[new_pos.row][new_pos.column] = old_values[idx];
        }

        robot = next_pos;
    }

    let coordinates_p2 = calculate_coordinates(&warehouse);  

    println!("Part 1 Answer: {}", coordinates_p1);
    println!("Part 2 Answer: {}" ,coordinates_p2);
    println!("Time: {:?}", start_time.elapsed());
}