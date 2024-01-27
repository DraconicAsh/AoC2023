use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input").unwrap();
    let mut input = BufReader::with_capacity(22528, f);
    let mut total = 0_u32;

    let mut line = String::new();
    loop {
        line.clear();
        let len = input.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }
        total += parse_line(&mut line);
    }

    println!("Answer: {total}");
}

fn parse_line(line: &mut String) -> u32 {
    let mut res = String::new();
    let mut last_num: char = ' ';
    for c in line.chars() {
        if c.is_numeric() {
            if res.is_empty() {
                res.push(c);
            }
            last_num = c;
        }
    }
    res.push(last_num);
    res.parse().unwrap()
}
