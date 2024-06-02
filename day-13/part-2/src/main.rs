use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = args().collect();

    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);

    let mut total = 0;
    let mut rows: Vec<String> = Vec::new();
    let mut cols: Vec<String> = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            total += calc(&rows, &cols);
            rows.clear();
            cols.clear();
            continue;
        }

        if cols.is_empty() {
            for c in line.chars() {
                cols.push(c.into());
            }
        } else {
            for (i, c) in line.char_indices() {
                cols[i].push(c);
            }
        }
        rows.push(line);
    }
    total += calc(&rows, &cols);
    println!("Answer: {total}");
}

fn calc(rows: &Vec<String>, cols: &Vec<String>) -> usize {
    if let Some(n) = find_reflection(rows) {
        n * 100
    } else {
        find_reflection(cols).unwrap()
    }
}

fn find_reflection(rows: &Vec<String>) -> Option<usize> {
    let mut top_idx = 0;
    let mut bot_idx = 0;
    let mut reflecting = false;
    for i in 1..rows.len() {
        if reflecting {
            let bot_diff = i - bot_idx;
            if bot_diff > top_idx {
                return Some(bot_idx);
            }
            if rows[top_idx - bot_diff] == rows[i] {
                continue;
            } else {
                reflecting = false;
            }
        }

        if rows[i - 1] == rows[i] {
            top_idx = i - 1;
            bot_idx = i;
            reflecting = true;
        }
    }
    if reflecting {
        Some(bot_idx)
    } else {
        None
    }
}
