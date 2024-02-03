use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);

    let mut total = 0;
    for line in input.lines() {
        let history = parse_line(&line.unwrap());
        total += calc_next(history);
    }

    println!("Answer: {total}");
}

fn parse_line(line: &'_ str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn calc_next(history: Vec<i64>) -> i64 {
    if history.iter().all(|x| *x == 0) {
        return 0;
    }

    let mut diffs: Vec<i64> = Vec::new();
    let mut last_num = 0;

    let mut iter = history.iter().peekable();
    while let Some(num) = iter.next() {
        let next = match iter.peek() {
            Some(n) => n,
            None => {
                last_num = *num;
                break;
            }
        };

        diffs.push(*next - num);
    }

    last_num + calc_next(diffs)
}
