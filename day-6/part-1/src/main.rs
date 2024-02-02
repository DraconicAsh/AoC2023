use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);
    let mut buf = input.lines();

    let times = parse_line(&buf.next().unwrap().unwrap());
    let records = parse_line(&buf.next().unwrap().unwrap());
    let races = zip(times, records);

    let mut total = 1;
    for race in races {
        total *= calc_err_margin(race);
    }

    println!("Answer: {total}");
}

fn parse_line(line: &'_ str) -> Vec<u32> {
    let mut res = Vec::new();
    let line = line.split(':').nth(1).unwrap();
    for n in line.split_whitespace() {
        res.push(n.parse::<u32>().unwrap());
    }
    res
}

fn calc_err_margin(race: (u32, u32)) -> u32 {
    let (time, record) = race;
    let poss_times: Vec<u32> = (1..time).collect();
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

fn time_wins(time_held: u32, total_time: u32, record: u32) -> bool {
    (total_time - time_held) * time_held > record
}
