use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);

    let mut total = 0;
    for line in input.lines() {
        let line = line.unwrap();
        total += Game::from_line(&line).calc_score();
    }

    println!("Answer: {total}");
}

struct Game {
    winning_nums: Vec<u32>,
    nums: Vec<u32>,
}

impl Game {
    fn calc_score(&self) -> u32 {
        let mut total = 1;
        for n in self.winning_nums.iter() {
            if self.nums.contains(n) {
                total *= 2;
            }
        }

        match total {
            1 => 0,
            _ => total / 2,
        }
    }

    fn from_line(line: &'_ str) -> Self {
        let line = line.split(':').nth(1).unwrap();
        let mut segments = line.split('|');
        let winning_nums = Game::parse_nums(segments.next().unwrap());
        let nums = Game::parse_nums(segments.next().unwrap());
        Self { winning_nums, nums }
    }

    fn parse_nums(nums: &'_ str) -> Vec<u32> {
        let mut res: Vec<u32> = Vec::new();
        for n in nums.split_whitespace() {
            let num = n.parse::<u32>().unwrap();
            res.push(num);
        }
        res
    }
}
