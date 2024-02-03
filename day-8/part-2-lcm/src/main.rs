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

    let mut nodes = Vec::new();
    for key in network.keys() {
        if key.chars().nth_back(0).unwrap() == 'A' {
            nodes.push(key);
        }
    }

    let mut step_counts: Vec<u64> = Vec::new();
    for node_idx in 0..nodes.len() {
        let mut count = 0;
        for i in instructions.clone() {
            if nodes[node_idx].chars().nth_back(0).unwrap() == 'Z' {
                break;
            }
            count += 1;
            nodes[node_idx] = &network.get(nodes[node_idx]).unwrap()[i];
        }
        step_counts.push(count);
    }

    let mut ans = 1;
    for n in step_counts {
        ans = lcm(ans, n);
    }

    println!("Answer: {ans}");
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let (a, b) = if b > a { (b, a) } else { (a, b) };
    let (mut y, mut z) = (a, b);

    let mut remainder = std::u64::MAX;
    while remainder != 0 {
        let mut mult = 1;
        while remainder >= z {
            remainder = y % (z * mult);
            mult += 1;
        }
        (y, z) = (z, remainder);
    }
    y
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        let gcd_t = gcd(1071, 462);
        let gcd_r = gcd(462, 1071);

        assert_eq!(gcd_t, gcd_r);
        assert_eq!(gcd_t, 21);
    }

    #[test]
    fn lcm_test() {
        let lcm1 = lcm(24, 36);
        let lcm2 = lcm(11, 3);
        let lcm3 = lcm(8, 10);

        assert_eq!(lcm1, 72);
        assert_eq!(lcm2, 33);
        assert_eq!(lcm3, 40);
    }
}
