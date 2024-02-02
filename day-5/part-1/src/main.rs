use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let mut input = BufReader::new(f);

    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_line = String::new();
    input.read_line(&mut seed_line).unwrap();
    let seed_line = seed_line.split(':').nth(1).unwrap().split_whitespace();
    for s in seed_line {
        seeds.push(s.parse::<u64>().unwrap());
    }

    let mut vals = seeds;
    loop {
        let section = read_section(&mut input);
        if section.is_empty() {
            break;
        }
        let map = gen_map(section);
        vals = map_values(&map, vals);
    }

    let mut locations = vals;
    locations.sort();
    println!("Answer: {}", locations[0]);
}

fn map_values(map: &Map, values: Vec<u64>) -> Vec<u64> {
    let mut res = values;
    let mut tmp: Vec<u64> = Vec::new();
    for v in res {
        tmp.push(map.map_val(v));
    }
    res = tmp;

    res
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

    match sec.as_str() {
        "" => "".to_string(),
        _ => sec.split(':').nth(1).unwrap().trim().to_string(),
    }
}

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
}

struct MapSegment {
    source_range: std::ops::Range<u64>,
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
