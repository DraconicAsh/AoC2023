use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = args().collect();

    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);

    let mut total = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let mut r = Record::parse(&line);
        let t = r.variants();
        println!("\n{}: {:?} | {}", line, r.row, t);
        total += t;
    }

    println!("Answer: {total}");
}

struct Record {
    row: Row,
    groups: Vec<u8>,
    memo: HashMap<Row, u32>,
}

impl Record {
    fn parse(line: &'_ str) -> Self {
        let mut row: Row = Vec::new();
        let mut groups: Vec<u8> = Vec::new();
        let mut line = line.split_whitespace();

        for c in line.next().unwrap().chars() {
            row.push(match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            })
        }

        for n in line.next().unwrap().split(',') {
            groups.push(n.parse().unwrap());
        }

        Self {
            row,
            groups,
            memo: HashMap::new(),
        }
    }

    fn variants(&mut self) -> u32 {
        let row = self.row.to_owned();
        self.gen(&row)
    }

    fn gen(&mut self, row: &Row) -> u32 {
        if let Some(total) = self.memo.get(row) {
            return *total;
        }

        let mut res = 0;
        let mut new_row: Row = Vec::new();
        let mut branch_idx = 0;
        let mut branch: bool = false;
        for (i, s) in row.iter().enumerate() {
            if *s == Spring::Unknown && !branch {
                branch = true;
                branch_idx = i;
            }
            new_row.push(*s);
        }

        if !branch {
            return self.check(row);
        }

        new_row[branch_idx] = Spring::Operational;
        res += self.gen(&new_row);
        new_row[branch_idx] = Spring::Damaged;
        res += self.gen(&new_row);

        res
    }

    fn check(&self, row: &Row) -> u32 {
        let mut groups: Vec<u8> = Vec::new();
        let mut total: u8 = 0;
        for s in row.iter() {
            match s {
                Spring::Damaged => total += 1,
                Spring::Operational => {
                    if total != 0 {
                        groups.push(total);
                        total = 0;
                    }
                }
                Spring::Unknown => unreachable!(),
            }
        }
        if total != 0 {
            groups.push(total);
        }

        if groups == self.groups {
            return 1;
        }
        0
    }
}

type Row = Vec<Spring>;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}
