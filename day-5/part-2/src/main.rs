use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let mut input = BufReader::new(f);

    let mut seeds: Vec<ValRange> = Vec::new();
    let mut seed_line = String::new();
    input.read_line(&mut seed_line).unwrap();
    let mut seed_line = seed_line.split(':').nth(1).unwrap().split_whitespace();
    while let Some(s) = seed_line.next() {
        let start = s.parse::<u64>().unwrap();
        let end = seed_line.next().unwrap().parse::<u64>().unwrap() + start;
        seeds.push(ValRange(start, end));
    }

    let mut maps: Vec<Map> = Vec::new();
    loop {
        let sec = read_section(&mut input);
        if sec.is_empty() {
            break;
        }

        maps.push(gen_map(sec));
    }

    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds {
        println!("Run through {seed:?}");
        locations.push(seeds_to_location(&maps, seed));
    }

    locations.sort();
    println!("Answer: {}", locations[0]);
}

fn seeds_to_location(maps: &Vec<Map>, seed: ValRange) -> u64 {
    let mut maps = maps.iter();
    let mut res = maps.next().unwrap().map_range(seed);

    for map in maps {
        res = map.map_values(res);
    }

    res.sort();
    res[0]
}

fn gen_map(raw_text: String) -> Map {
    let mut res = Map::new();
    for line in raw_text.split("\n") {
        res.add_segment(line);
    }
    res
}

fn read_section<T: std::io::Read>(buf: &mut BufReader<T>) -> String {
    let mut sec = String::new();
    loop {
        if buf.read_line(&mut sec).unwrap() == 0 {
            break;
        }
        if sec.contains("\n\n") {
            break;
        }
    }

    if sec.is_empty() {
        sec.to_string()
    } else {
        sec.split(':').nth(1).unwrap().trim().to_string()
    }
}

#[derive(Debug)]
struct ValRange(u64, u64);

struct Map {
    segments: Vec<MapSegment>,
}

impl Map {
    fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    fn add_segment(&mut self, line: &'_ str) {
        println!("Add segment\n{line}");
        self.segments.push(MapSegment::from_line(line));
    }

    fn map_val(&self, val: u64) -> u64 {
        for seg in self.segments.iter() {
            if let Some(v) = seg.map_val(val) {
                return v;
            }
        }
        val
    }

    fn map_values(&self, values: Vec<u64>) -> Vec<u64> {
        let mut res = values;
        let mut tmp = Vec::new();
        for val in res {
            tmp.push(self.map_val(val));
        }
        res = tmp;

        res
    }

    fn map_range(&self, range: ValRange) -> Vec<u64> {
        let mut res = Vec::new();
        for val in range.0..range.1 {
            res.push(self.map_val(val));
        }

        res
    }
}

struct MapSegment {
    source_range: Range<u64>,
    source_start: u64,
    dest_start: u64,
}

impl MapSegment {
    fn from_line(line: &'_ str) -> Self {
        let mut vals = line.split_whitespace();
        let dest_start = vals.next().unwrap().parse::<u64>().unwrap();
        let source_start = vals.next().unwrap().parse::<u64>().unwrap();
        let range = vals.next().unwrap().parse::<u64>().unwrap();

        let source_range = source_start..(source_start + range);

        Self {
            source_range,
            source_start,
            dest_start,
        }
    }

    fn map_val(&self, val: u64) -> Option<u64> {
        if self.source_range.contains(&val) {
            let idx = val - self.source_start;
            Some(self.dest_start + idx as u64)
        } else {
            None
        }
    }
}
