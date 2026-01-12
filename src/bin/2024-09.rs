use std::fs;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum Segment {
    File { id: usize, len: usize },
    Free { len: usize },
}

fn main() {
    // Note: Ensure the path is correct for your environment
    let input = fs::read_to_string("data/2024/09/input.txt")
        .unwrap_or_else(|_| "2333133121414131402".to_string()); // Fallback to example if file missing

    let start_time = Instant::now();
    let trim_input = input.trim();

    // 1. Parse Input into Segments
    // We clone this later for Part 2 because Part 1 logic consumes its own data structure
    let mut initial_segments: Vec<Segment> = Vec::with_capacity(trim_input.len());
    let mut file_id = 0;

    for (i, b) in trim_input.bytes().enumerate() {
        let len = (b - b'0') as usize;
        if i % 2 == 0 {
            initial_segments.push(Segment::File { id: file_id, len });
            file_id += 1;
        } else {
            initial_segments.push(Segment::Free { len });
        }
    }
    
    // The highest file ID is actually file_id - 1 because we incremented at the end
    let max_file_id = file_id - 1;

    // --- PART 1 LOGIC (Preserved) ---
    // Expanding to blocks
    let mut total_blocks = 0;
    for seg in &initial_segments {
        match seg {
            Segment::File { len, .. } | Segment::Free { len } => total_blocks += len,
        }
    }

    let mut disk: Vec<Option<usize>> = Vec::with_capacity(total_blocks);
    for segment in &initial_segments {
        match segment {
            Segment::File { id, len } => {
                for _ in 0..*len { disk.push(Some(*id)); }
            }
            Segment::Free { len } => {
                for _ in 0..*len { disk.push(None); }
            }
        }
    }

    let mut left = 0;
    let mut right = disk.len().saturating_sub(1);
    while left < right {
        while left < right && disk[left].is_some() { left += 1; }
        while left < right && disk[right].is_none() { right -= 1; }
        if left < right {
            disk.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    let mut checksum_p1: u64 = 0;
    for (i, block) in disk.iter().enumerate() {
        if let Some(id) = block {
            checksum_p1 += (i as u64) * (*id as u64);
        }
    }

    // --- PART 2 LOGIC ---
    
    // Work on a fresh copy of segments
    let mut segments = initial_segments.clone();

    // Iterate DECREASING file ID
    for curr_id in (0..=max_file_id).rev() {
        
        // 1. Find the current location (index) of the file with curr_id
        let mut old_idx = 0;
        let mut file_len = 0;

        for (i, seg) in segments.iter().enumerate() {
            if let Segment::File { id, len } = seg {
                if *id == curr_id {
                    old_idx = i;
                    file_len = *len;
                    break;
                }
            }
        }

        // 2. Find the leftmost suitable free space
        // Important: We only look strictly to the left (i < old_idx)
        let mut target_idx = None;
        for i in 0..old_idx {
            if let Segment::Free { len } = segments[i] {
                if len >= file_len {
                    target_idx = Some(i);
                    break;
                }
            }
        }

        // 3. Move the file if a spot was found
        if let Some(free_idx) = target_idx {
            // Retrieve the full length of the free space we found
            let free_len = match segments[free_idx] {
                Segment::Free { len } => len,
                _ => unreachable!()
            };

            let remainder = free_len - file_len;

            // A. Replace the OLD file position with Free space
            // This preserves the disk layout size
            segments[old_idx] = Segment::Free { len: file_len };

            // B. Place the file in the NEW position
            segments[free_idx] = Segment::File { id: curr_id, len: file_len };

            // C. If there was extra free space, insert it AFTER the new file
            // Note: Since free_idx < old_idx, inserting here changes indices
            // to the right, but we are done with old_idx for this iteration anyway.
            if remainder > 0 {
                segments.insert(free_idx + 1, Segment::Free { len: remainder });
            }
        }
    }

    // --- Calculate Checksum Part 2 ---
    let mut checksum_p2: u64 = 0;
    let mut global_pos: u64 = 0;

    for segment in &segments {
        match segment {
            Segment::File { id, len } => {
                for _ in 0..*len {
                    checksum_p2 += global_pos * (*id as u64);
                    global_pos += 1;
                }
            }
            Segment::Free { len } => {
                // Free space advances position but adds nothing to checksum
                global_pos += *len as u64;
            }
        }
    }

    let duration = start_time.elapsed();

    println!("Part 1 Answer: {}", checksum_p1);
    println!("Part 2 Answer: {}", checksum_p2);
    println!("Total calculation time: {:?}", duration);
}