use std::fs;
use std::time::Instant; // Import the stopwatch

fn main() {
    let input = fs::read_to_string("data/2024/09/input.txt")
        .expect("Should have been able to read the file");

    let start_time = Instant::now(); 

    let mut files: Vec<(i32, u32)> = Vec::new();

    for (i, c) in input.trim().chars().enumerate() {
        let size: u32 = c.to_digit(10).unwrap();
        if (i % 2) == 0 {
            files.push(((i / 2) as i32, size));
        } else {
            files.push((-1, size));
        }
    }

    // for (id, size) in &files{
    //     print!("{}, {}\n", id, size);
    // }

    if (files.len() % 2) == 0 {
        panic!("Oh no!");
    }
    
    let mut checksum: u64 = 0;
    let mut current_pos: i32 = 0;

    while !files.is_empty() {
        let (id, size) = files.remove(0);
        for _ in 0..size {
            if id != -1 {
                checksum += (current_pos * id) as u64;
            } else {
                let (id_last, size_last) = files.pop().unwrap();
                checksum += (current_pos * id_last) as u64;
                if size_last > 1 {
                    files.push((id_last, size_last - 1));
                } else {
                    files.pop();
                }
            }

            current_pos += 1;
        }
    }

    // Stop timing
    let duration = start_time.elapsed();

    println!("Part 1 Answer: {}", checksum);

    // Display the time
    println!("Total calculation time: {:?}", duration);

}