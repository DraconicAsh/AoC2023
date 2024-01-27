use core::panic;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

//const RED: usize = 0;
//const GREEN: usize = 1;
//const BLUE: usize = 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);

    let mut total = 0;
    for line in input.lines() {
        let mut line = line.unwrap();
        println!("{line}");
        total += power_of_set(&mut line[..]);
    }

    println!("Answer: {total}");
}

fn power_of_set(game: &mut str) -> usize {
    let prefix_end = game.find(':').unwrap() + 2;
    let game = parse_game(&mut game[prefix_end..]);
    let mut rounds = game.into_iter();
    let mut min_set: Round = rounds.next().unwrap();
    println!("{:?}", min_set);
    for set in rounds {
        if set.0 > min_set.0 {
            min_set.0 = set.0
        }
        if set.1 > min_set.1 {
            min_set.1 = set.1
        }
        if set.2 > min_set.2 {
            min_set.2 = set.2
        }
        println!("{:?}", min_set);
    }
    min_set.0 * min_set.1 * min_set.2
}

fn parse_game(game: &mut str) -> Game {
    let mut res: Game = Vec::new();
    for round in game.split(';') {
        let mut r = Round(0, 0, 0);
        for color in round.split(',') {
            let pair: Vec<&str> = color.split_whitespace().collect();
            let count = pair[0].parse::<usize>().unwrap();
            match pair[1] {
                "red" => r.0 += count,
                "green" => r.1 += count,
                "blue" => r.2 += count,
                _ => panic!("Bad Color"),
            }
        }
        res.push(r);
    }
    res
}

type Game = Vec<Round>;

#[derive(Debug)]
struct Round(usize, usize, usize);
