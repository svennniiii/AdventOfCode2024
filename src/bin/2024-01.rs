use std::collections::HashSet;
use std::fs;

fn main() {
    // Read directly from the file "input.txt"
    let content = fs::read_to_string("data/2024/01/input.txt")
        .expect("Failed to read input.txt - ensure the file exists in the same directory");

    // Standardize newlines to handle Windows (\r\n) or Unix (\n) formats
    let content = content.replace("\r\n", "\n");

    // The input is separated into two parts (rules and updates) by a blank line
    let sections: Vec<&str> = content.split("\n\n").collect();
    if sections.len() < 2 {
        eprintln!("Invalid input format. Expected rules and updates separated by a blank line.");
        return;
    }

    let rules_section = sections[0];
    let updates_section = sections[1];

    // Parse Rules into a HashSet for O(1) lookup
    // A rule (X, Y) means X must be printed before Y
    let mut rules = HashSet::new();
    for line in rules_section.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((x_str, y_str)) = line.split_once('|') {
            let x: u32 = x_str.trim().parse().expect("Invalid number in rules");
            let y: u32 = y_str.trim().parse().expect("Invalid number in rules");
            rules.insert((x, y));
        }
    }

    let mut sum_middle_pages = 0;

    // Process each update
    for line in updates_section.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let update: Vec<u32> = line
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid number in update"))
            .collect();

        if is_update_valid(&update, &rules) {
            // Find the middle page number
            // Integer division automatically handles the 0-indexed middle position
            // e.g., len 5 -> index 2 (0, 1, [2], 3, 4)
            let mid_index = update.len() / 2;
            sum_middle_pages += update[mid_index];
        }
    }

    println!("Sum of middle pages: {}", sum_middle_pages);
}

/// Checks if an update is correctly ordered according to the rules.
/// Returns false if any page appears *after* a page that it is supposed to precede.
fn is_update_valid(update: &[u32], rules: &HashSet<(u32, u32)>) -> bool {
    // Iterate through all pairs (i, j) where i comes before j in the update list
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            let page_early = update[i]; // The page currently appearing earlier
            let page_late = update[j];  // The page currently appearing later

            // Check if this specific order violates a rule.
            // A violation occurs if there is a rule saying 'page_late|page_early'
            // (meaning page_late MUST come before page_early), but here it comes after.
            if rules.contains(&(page_late, page_early)) {
                return false;
            }
        }
    }
    true
}