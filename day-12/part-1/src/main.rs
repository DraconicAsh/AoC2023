use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = args().collect();

    let f = File::open(&args[1]).unwrap();
    let mut input = BufReader::new(f);
}

struct Record {
    row: Row,
    groups: Vec<u32>,
}

// TODO: Branching Recursive
impl Record {
    fn parse(line: &'_ str) -> Self {
        let mut row: Row = Vec::new();
        let mut groups: Vec<u32> = Vec::new();
        let mut line = line.split_whitespace();

        for c in line.next().unwrap().chars() {
            row.push(match c {
                '.' => Spring::Operational,
                '#' => Spring::Broken,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            })
        }

        for n in line.next().unwrap().split(',') {
            groups.push(n.parse().unwrap());
        }

        Self { row, groups }
    }
}

type Row = Vec<Spring>;

enum Spring {
    Operational,
    Broken,
    Unknown,
}
