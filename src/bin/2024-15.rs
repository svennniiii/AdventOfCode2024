use std::fs;
use std::time::Instant;

#[derive(Clone, Copy)]
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

fn main() {
    let input = fs::read_to_string("data/2024/15/example.txt")
        .expect("Should have been able to read the file");
    let start_time = Instant::now();

    let (warehouse_str, movements_str) = input.split_once("\n\n").unwrap();

    let mut robot = Position{row: 0 ,column: 0 };
    let mut warehouse: Vec<Vec<char>> = Vec::new();

    for (row_index, line) in warehouse_str.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (column_index, char) in line.trim().chars().enumerate() {
            match char {
                '@' => {
                    robot = Position { row: row_index, column: column_index };
                    row.push('.');
                }
                _ => {row.push(char);}
            }
        }

        warehouse.push(row);
    }

    for direction in movements_str.chars() {
        if direction.is_whitespace() {
            continue;
        }

        let new_pos = robot.move_step(direction);
        let check_pos = new_pos;
        let mut ignore_movement: bool = false;
        while warehouse[check_pos.row][check_pos.column] != '.' {
            if warehouse[check_pos.row][check_pos.column] == '#' {
                ignore_movement = true;
                break;
            }

            check_pos.move_step(direction);            
        }

        if ignore_movement == true {
            continue;
        }

        if warehouse[new_pos.row][new_pos.column] != 'O' {
            warehouse[new_pos.row][new_pos.column] = '.';
            warehouse[check_pos.row][check_pos.column] = 'O';
        }
    }

    for row in warehouse {
        for c in row{
            print!("{}", c);
        }
        println!();
    }   

    println!("Part 1 Answer: {}", 0);
    println!("Part 2 Answer: {}", 0);
    println!("Time: {:?}", start_time.elapsed());
}