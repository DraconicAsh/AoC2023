use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

const EXP_COST: u64 = 1_000_000;

fn main() {
    let args: Vec<String> = args().collect();

    let f = File::open(&args[1]).unwrap();
    let mut input = BufReader::new(f);

    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let mut builder = UniverseBuilder::new(&line.trim_end());
    for line in input.lines() {
        builder.add_row(&line.unwrap());
    }
    let universe = builder.build();
    println!("{universe:?}");

    let total = universe.total_distance();
    println!("Answer: {total}");
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<(u64, u64)>,
    expanded_rows: Vec<u64>,
    expanded_columns: Vec<u64>,
    width: u64,
    height: u64,
}

impl Universe {
    fn total_distance(&self) -> u64 {
        let mut total = 0;
        let (tx, rx): (Sender<(usize, DijkstraMap)>, Receiver<(usize, DijkstraMap)>) =
            mpsc::channel();
        let mut threads = Vec::new();
        let len = self.galaxies.len();
        for i in 0..len {
            let sender = tx.clone();
            let idx = i;
            let source = self.galaxies[i];
            let width = self.width;
            let height = self.height;
            let expanded_rows = self.expanded_rows.to_vec();
            let expanded_columns = self.expanded_columns.to_vec();
            let child = thread::spawn(move || {
                let map: DijkstraMap = DijkstraMapBuilder::new(
                    source,
                    width,
                    height,
                    &expanded_rows,
                    &expanded_columns,
                )
                .build();
                sender.send((idx, map)).unwrap();
            });
            threads.push(child);
            println!("Thread {i} Spawned");
        }
        println!("{len}");
        for _ in 0..len {
            let (i, map) = rx.recv().unwrap();
            for p in i..len {
                total += map.get_distance(self.galaxies[p]);
            }
        }

        total
    }
}

#[derive(Debug)]
struct DijkstraMap {
    grid: Vec<u64>,
    width: u64,
    height: u64,
}

impl DijkstraMap {
    fn get_distance(&self, coords: (u64, u64)) -> u64 {
        let idx = self.idx_from_coords(coords);
        self.grid[idx] as u64
    }

    fn idx_from_coords(&self, coords: (u64, u64)) -> usize {
        let res = (coords.1 * self.width) + coords.0;
        res as usize
    }
}

struct DijkstraMapBuilder {
    grid: Vec<u64>,
    cost_grid: Vec<u64>,
    source: (u64, u64),
    width: u64,
    height: u64,
}

impl DijkstraMapBuilder {
    fn build(&mut self) -> DijkstraMap {
        let mut remaining_idx: Vec<usize> = (0..self.grid.len()).collect();
        while !remaining_idx.is_empty() {
            remaining_idx.sort_unstable_by(|a, b| {
                let a: u64 = self.grid[*a];
                let b: u64 = self.grid[*b];
                b.cmp(&a)
            });
            let current = remaining_idx.pop().unwrap();
            let neighbors: Vec<usize> = self
                .neighbors(self.coords_from_idx(current))
                .into_iter()
                .filter(|p| remaining_idx.contains(p))
                .collect();
            for n in neighbors {
                let cost = self.grid[current] + self.cost_grid[n];
                if cost < self.grid[n] {
                    self.grid[n] = cost;
                }
            }
        }

        DijkstraMap {
            grid: self.grid.to_vec(),
            width: self.width,
            height: self.height,
        }
    }

    fn new(
        source: (u64, u64),
        width: u64,
        height: u64,
        expanded_rows: &Vec<u64>,
        expanded_columns: &Vec<u64>,
    ) -> Self {
        let size = width as usize * height as usize;
        let cost_grid: Vec<u64> = vec![0; size];
        let mut res = Self {
            grid: vec![u64::MAX; size],
            cost_grid,
            source,
            width,
            height,
        };

        for row in expanded_rows.iter() {
            for i in 0..res.height {
                let idx = res.idx_from_coords((i, *row));
                res.cost_grid[idx] += EXP_COST;
            }
        }
        for col in expanded_columns.iter() {
            for i in 0..res.width {
                let idx = res.idx_from_coords((*col, i));
                res.cost_grid[idx] += EXP_COST;
            }
        }
        for t in res.cost_grid.iter_mut() {
            if *t == 0 {
                *t = 1;
            }
        }
        let idx = res.idx_from_coords(res.source);
        res.grid[idx] = 0;
        res.cost_grid[idx] = 0;
        res
    }

    fn neighbors(&self, coords: (u64, u64)) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        if coords.0 > 0 {
            let n = self.idx_from_coords((coords.0 - 1, coords.1));
            res.push(n);
        }
        if coords.0 < self.width - 1 {
            let n = self.idx_from_coords((coords.0 + 1, coords.1));
            res.push(n);
        }
        if coords.1 > 0 {
            let n = self.idx_from_coords((coords.0, coords.1 - 1));
            res.push(n);
        }
        if coords.1 < self.height - 1 {
            let n = self.idx_from_coords((coords.0, coords.1 + 1));
            res.push(n);
        }
        res
    }

    fn idx_from_coords(&self, coords: (u64, u64)) -> usize {
        let res = (coords.1 * self.width) + coords.0;
        res as usize
    }

    fn coords_from_idx(&self, idx: usize) -> (u64, u64) {
        let idx = idx as u64;
        let y = idx / self.width;
        let x = idx % self.width;
        (x, y)
    }
}

impl std::fmt::Display for DijkstraMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let mut steps = 0;
        for i in self.grid.iter() {
            if steps == self.width {
                out.push_str("\n");
                steps = 0;
            }
            out.push_str(&format!(" {i} ")[..]);
            steps += 1;
        }
        write!(
            f,
            "\n{}\nWidth: {}\nHeight: {}\n",
            out, self.width, self.height
        )
    }
}

#[derive(Debug)]
struct UniverseBuilder {
    galaxies: Vec<(u64, u64)>,
    width: u64,
    height: u64,
    empty_rows: Vec<u64>,
    empty_columns: Vec<u64>,
}

impl UniverseBuilder {
    fn new(line: &'_ str) -> Self {
        let mut galaxies: Vec<(u64, u64)> = Vec::new();
        let mut empty_columns: Vec<u64> = Vec::new();
        let mut width = 0;
        for (i, c) in line.char_indices() {
            match c {
                '#' => galaxies.push((i as u64, 0)),
                '.' => empty_columns.push(i as u64),
                _ => unreachable!(),
            }
            width += 1;
        }
        let empty_rows: Vec<u64> = if galaxies.is_empty() {
            vec![0]
        } else {
            Vec::new()
        };

        Self {
            galaxies,
            width,
            height: 1,
            empty_rows,
            empty_columns,
        }
    }

    fn add_row(&mut self, line: &'_ str) {
        let mut empty: bool = true;
        for (i, c) in line.char_indices() {
            let i = i as u64;
            match c {
                '#' => {
                    self.galaxies.push((i, self.height));
                    empty = false;
                    if let Ok(idx) = self.empty_columns.binary_search(&i) {
                        self.empty_columns.remove(idx);
                    }
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
        if empty {
            self.empty_rows.push(self.height);
        }
        self.height += 1;
    }

    fn build(&mut self) -> Universe {
        Universe {
            galaxies: self.galaxies.to_vec(),
            width: self.width,
            height: self.height,
            expanded_rows: self.empty_rows.to_vec(),
            expanded_columns: self.empty_columns.to_vec(),
        }
    }
}
