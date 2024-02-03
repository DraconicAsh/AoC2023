use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let mut input = BufReader::new(f);

    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();
    let instructions = parse_instructions(&buf);
    input.read_line(&mut buf).unwrap();

    let mut network: Network = HashMap::new();
    for line in input.lines() {
        parse_node(&mut network, &line.unwrap());
    }

    let mut node = "AAA";
    let mut count = 0;
    for i in instructions {
        if node == "ZZZ" {
            break;
        }
        count += 1;
        node = &network.get(node).unwrap()[i];
    }

    println!("Answer: {count}");
}

fn parse_node(network: &mut Network, line: &'_ str) {
    let mut line = line.split('=');
    let key = line.next().unwrap().trim().to_owned();

    let tup = line.next().unwrap().trim();
    let left = tup[1..=3].to_owned();
    let right = tup[6..=8].to_owned();

    network.insert(key, [left, right]);
}

fn parse_instructions(line: &'_ str) -> std::iter::Cycle<std::vec::IntoIter<usize>> {
    let mut res: Vec<usize> = Vec::new();
    for c in line.trim().chars() {
        res.push(match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        });
    }
    res.into_iter().cycle()
}

type Network = HashMap<String, [String; 2]>;
