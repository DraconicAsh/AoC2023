use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

fn main() {
    let args: Vec<String> = args().collect();

    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);

    let mut total = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let t = calc(&line);
        println!("{line}: {t}");
        total += t;
    }

    println!("Answer: {total}");
}

fn calc_with_cache(
    springs: &'_ str,
    groups: &'_ [u8],
    cache: &mut HashMap<(String, Vec<u8>), u64>,
) -> u64 {
    if let Some(hit) = cache.get(&(springs.into(), groups.into())) {
        println!("Hit!");
        return *hit;
    }

    if groups.is_empty() {
        if springs.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }
    if springs.len() < groups[0] as usize {
        return 0;
    }

    let mut res = 0;
    let this_spring = springs.chars().nth(0).unwrap();
    if this_spring == OPERATIONAL || this_spring == UNKNOWN {
        res += calc_with_cache(&springs[1..], groups, cache);
    }
    if this_spring == DAMAGED || this_spring == UNKNOWN {
        res += group_check(springs, groups, cache);
    }

    cache.insert((springs.into(), groups.into()), res);
    res
}

fn group_check(
    springs: &'_ str,
    groups: &'_ [u8],
    cache: &mut HashMap<(String, Vec<u8>), u64>,
) -> u64 {
    let group_size = groups[0] as usize;
    let this_group = &springs[0..group_size];

    if this_group.contains(OPERATIONAL) {
        return 0;
    }

    if group_size == springs.len() {
        if groups.len() == 1 {
            return 1;
        } else {
            return 0;
        }
    }

    let next_spring = springs.chars().nth(group_size).unwrap();
    if next_spring == OPERATIONAL || next_spring == UNKNOWN {
        return calc_with_cache(&springs[(group_size + 1)..], &groups[1..], cache);
    }

    0
}

fn calc(line: &'_ str) -> u64 {
    let mut line = line.split_whitespace();
    let springs = line.next().unwrap();
    let mut groups: Vec<u8> = Vec::new();
    for n in line.next().unwrap().split(',') {
        groups.push(n.parse().unwrap());
    }
    let mut cache: HashMap<(String, Vec<u8>), u64> = HashMap::new();
    calc_with_cache(&springs, &groups, &mut cache)
}
