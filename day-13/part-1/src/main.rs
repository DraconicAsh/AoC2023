use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = args().collect();

    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);

    let mut rows: Vec<String> = Vec::new();
    let mut refl_idx_top = 0;
    let mut refl_idx_bot = 0;
    let mut cols: Vec<String> = Vec::new();
    let mut reflecting = false;
    let mut total = 0;
    for line in input.lines() {
        let line = line.unwrap();
        // Check if we've reached the end of a pattern
        if line.is_empty() {
            total += if reflecting {
                // If we found a vertical reflection add the # of top rows * 100
                refl_idx_bot * 100
            } else {
                // Else find the horizontal reflection and add the # of left columns
                find_horiz_refl(&cols).unwrap()
            };

            // Reset vars before continuing to next line
            rows.clear();
            refl_idx_top = 0;
            refl_idx_bot = 0;
            cols.clear();
            reflecting = false;
            continue;
        }

        // If we're at the start of a pattern initialize the column Strings
        if cols.is_empty() {
            for c in line.chars() {
                cols.push(c.into());
            }
        } else {
            for (i, c) in line.char_indices() {
                cols[i].push(c);
            }
        }
        let idx = rows.len();
        rows.push(line);

        // If this is this pattern's first row continue to next line
        if rows.len() == 1 {
            continue;
        }

        // If we're currently checking a reflection
        if reflecting {
            // How many rows past the reflection point are we
            let bot_diff = idx - refl_idx_bot;
            // If the bottom is longer than the top continue
            if bot_diff > refl_idx_top {
                continue;
            }
            // Check if current row is reflected on top
            if rows[refl_idx_top - bot_diff] == rows[idx] {
                continue;
            } else {
                reflecting = false;
            }
        }

        // If current row reflects previous begin checking rows for reflection
        if rows[idx - 1] == rows[idx] {
            refl_idx_top = idx - 1;
            refl_idx_bot = idx;
            reflecting = true;
        }
    }
    // Wrap up final pattern
    total += if reflecting {
        refl_idx_bot * 100
    } else {
        find_horiz_refl(&cols).unwrap()
    };

    println!("Answer: {total}");
}

fn find_horiz_refl(cols: &Vec<String>) -> Option<usize> {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut reflecting = false;
    for i in 1..cols.len() {
        if reflecting {
            let right_diff = i - right_idx;
            // If right is longer than left return now
            if right_diff > left_idx {
                return Some(right_idx);
            }
            // Check if current column is reflected
            if cols[left_idx - right_diff] == cols[i] {
                continue;
            } else {
                reflecting = false;
            }
        }

        // If current column is reflected begin checking successive columns
        if cols[i - 1] == cols[i] {
            left_idx = i - 1;
            right_idx = i;
            reflecting = true;
        }
    }
    // Return # of left columns only if reflection followed through to the end
    if reflecting {
        Some(right_idx)
    } else {
        None
    }
}
