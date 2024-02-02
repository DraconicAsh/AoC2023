use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);
    let mut buf = input.lines();

    let time = parse_line(&buf.next().unwrap().unwrap());
    let record = parse_line(&buf.next().unwrap().unwrap());

    let total = calc_err_margin((time, record));

    println!("Answer: {total}");
}

fn parse_line(line: &'_ str) -> u64 {
    let line = line.split(':').nth(1).unwrap();
    let num: String = line.chars().filter(|p| !p.is_whitespace()).collect();
    num.parse().unwrap()
}

fn calc_err_margin(race: (u64, u64)) -> u64 {
    let (time, record) = race;
    let poss_times: Vec<u64> = (1..time).collect();
    let vertex = time / 2;

    let mut res = 0;
    let mut idx = vertex as usize;
    while time_wins(poss_times[idx], time, record) {
        res += 1;
        idx += 1;
    }
    idx = vertex as usize - 1;
    while time_wins(poss_times[idx], time, record) {
        res += 1;
        idx -= 1;
    }

    res
}

fn time_wins(time_held: u64, total_time: u64, record: u64) -> bool {
    (total_time - time_held) * time_held > record
}
