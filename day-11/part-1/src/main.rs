use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    galaxies: Vec<(u32, u32)>,
    width: u32,
    height: u32,
}

impl Universe {
    fn total_distance(&self) -> u32 {
        let mut total = 0;
        let len = self.galaxies.len();
        for i in 0..len {
            let mut map: DijkstraMap = DijkstraMap::new(self.galaxies[i], self.width, self.height);
            map.build();
            for p in (i + 1)..len {
                total += map.get_distance(self.galaxies[p]);
            }
        }
        total
    }
}

#[derive(Debug)]
struct DijkstraMap {
    grid: Vec<u32>,
    source: (u32, u32),
    width: u32,
    height: u32,
}

impl DijkstraMap {
    fn get_distance(&self, coords: (u32, u32)) -> u32 {
        let idx = self.idx_from_coords(coords);
        self.grid[idx]
    }

    fn build(&mut self) {
        let idx = self.idx_from_coords(self.source);
        self.grid[idx] = 0;
        let mut frontier: Vec<usize> = vec![idx];
        let mut imm_frontier: Vec<usize> = Vec::new();
        while !frontier.is_empty() {
            imm_frontier.append(&mut frontier);
            while !imm_frontier.is_empty() {
                let current = imm_frontier.pop().unwrap();
                let neighbors = self.unchecked_neighbors(self.coords_from_idx(current));
                for n in neighbors {
                    self.grid[n] = self.grid[current] + 1;
                    frontier.push(n);
                }
            }
        }
    }

    fn new(source: (u32, u32), width: u32, height: u32) -> Self {
        Self {
            grid: vec![u32::MAX; width as usize * height as usize],
            source,
            width,
            height,
        }
    }

    fn unchecked_neighbors(&self, coords: (u32, u32)) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        if coords.0 > 0 {
            let n = self.idx_from_coords((coords.0 - 1, coords.1));
            if self.grid[n] == u32::MAX {
                res.push(n);
            }
        }
        if coords.0 < self.width - 1 {
            let n = self.idx_from_coords((coords.0 + 1, coords.1));
            if self.grid[n] == u32::MAX {
                res.push(n);
            }
        }
        if coords.1 > 0 {
            let n = self.idx_from_coords((coords.0, coords.1 - 1));
            if self.grid[n] == u32::MAX {
                res.push(n);
            }
        }
        if coords.1 < self.height - 1 {
            let n = self.idx_from_coords((coords.0, coords.1 + 1));
            if self.grid[n] == u32::MAX {
                res.push(n);
            }
        }
        res
    }

    fn idx_from_coords(&self, coords: (u32, u32)) -> usize {
        let res = (coords.1 * self.width) + coords.0;
        res as usize
    }

    fn coords_from_idx(&self, idx: usize) -> (u32, u32) {
        let idx = idx as u32;
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
    galaxies: Vec<(u32, u32)>,
    width: u32,
    height: u32,
    empty_rows: Vec<u32>,
    empty_columns: Vec<u32>,
}

impl UniverseBuilder {
    fn new(line: &'_ str) -> Self {
        let mut galaxies: Vec<(u32, u32)> = Vec::new();
        let mut empty_columns: Vec<u32> = Vec::new();
        let mut width = 0;
        for (i, c) in line.char_indices() {
            match c {
                '#' => galaxies.push((i as u32, 0)),
                '.' => empty_columns.push(i as u32),
                _ => unreachable!(),
            }
            width += 1;
        }
        let empty_rows: Vec<u32> = if galaxies.is_empty() {
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
            let i = i as u32;
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
        self.empty_rows.reverse();
        self.empty_columns.reverse();
        for row in self.empty_rows.iter() {
            for g in self.galaxies.iter_mut() {
                if g.1 > *row {
                    *g = (g.0, g.1 + 1);
                }
            }
            self.height += 1;
        }
        for column in self.empty_columns.iter() {
            for g in self.galaxies.iter_mut() {
                if g.0 > *column {
                    *g = (g.0 + 1, g.1);
                }
            }
            self.width += 1;
        }
        Universe {
            galaxies: self.galaxies.to_vec(),
            width: self.width,
            height: self.height,
        }
    }
}
