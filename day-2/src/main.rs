use core::panic;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const RED_CAP: u8 = 12;
const GREEN_CAP: u8 = 13;
const BLUE_CAP: u8 = 14;

const RED: usize = 0;
const GREEN: usize = 1;
const BLUE: usize = 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);

    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        let mut line = line.unwrap();
        if game_possible(&mut line[..]) {
            total += i + 1;
        }
    }

    println!("Answer: {total}");
}

fn game_possible(game: &mut str) -> bool {
    let prefix_end = game.find(':').unwrap() + 2;
    let game = parse_game(&mut game[prefix_end..]);
    for round in game {
        if round.0 > RED_CAP || round.1 > GREEN_CAP || round.2 > BLUE_CAP {
            return false;
        }
    }
    true
}

fn parse_game(game: &mut str) -> Game {
    let mut res: Game = Vec::new();
    for round in game.split(';') {
        let mut r = Round(0, 0, 0);
        for color in round.split(',') {
            let pair: Vec<&str> = color.split_whitespace().collect();
            let count = pair[0].parse::<u8>().unwrap();
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

struct Round(u8, u8, u8);
