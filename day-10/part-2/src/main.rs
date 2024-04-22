// Too long of a break
// Start fresh ignoring Part 1
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};
use std::{env, usize};

const NORTH: u8 = 0b0000_0010;
const SOUTH: u8 = 0b0000_0100;
const EAST: u8 = 0b0000_1000;
const WEST: u8 = 0b0001_0000;

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let mut input = BufReader::new(f);

    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let mut grid = Grid::from_line(&line.trim_end());

    for line in input.lines() {
        grid.add_line(&line.unwrap())
    }

    let (vertices, boundary_points) = find_vertices(&grid);
    println!("Vertices: {:?}", vertices);

    let area = shoelace(&vertices);
    println!("Area: {area}");

    let inner_points = picks(area, boundary_points);
    println!("Answer: {inner_points}");
}

fn picks(area: usize, points: usize) -> usize {
    area + 1 - (points / 2)
}

fn shoelace(vertices: &Vec<(usize, usize)>) -> usize {
    let mut a: isize = 0;
    let mut b: isize = 0;
    for i in 0..vertices.len() {
        let n = if (i + 1) < vertices.len() { i + 1 } else { 0 };
        a += vertices[i].0 as isize * vertices[n].1 as isize;
        b += vertices[i].1 as isize * vertices[n].0 as isize;
    }
    let abs = (a - b).abs() as usize;
    abs / 2
}

fn find_vertices(grid: &Grid) -> (Vec<(usize, usize)>, usize) {
    let start = grid.coords_from_idx(grid.start);
    let mut next_tile = (usize::MAX, usize::MAX);
    let mut previous_tile: u8 = 0;
    let mut vertices: Vec<(usize, usize)> = Vec::new();
    let mut points: usize = 1;
    vertices.push(start);

    if start.0 > 0 && grid.check_bounds((start.0 + 1, start.1)) {
        let west = (start.0 - 1, start.1);
        let east = (start.0 + 1, start.1);
        if grid[west] as u8 & EAST > 0 {
            next_tile = west;
            previous_tile = EAST;
        } else if grid[east] as u8 & WEST > 0 {
            next_tile = east;
            previous_tile = WEST;
        }
    }
    if start.1 > 0 && grid.check_bounds((start.0, start.1 + 1)) {
        let north = (start.0, start.1 - 1);
        let south = (start.0, start.1 + 1);
        if grid[north] as u8 & SOUTH > 0 {
            next_tile = north;
            previous_tile = SOUTH;
        } else if grid[south] as u8 & NORTH > 0 {
            next_tile = south;
            previous_tile = NORTH;
        }
    }

    while next_tile != start {
        points += 1;
        let t = grid[next_tile] as u8;
        match grid[next_tile] {
            Tile::NS | Tile::EW => (),
            Tile::Start | Tile::Ground => unreachable!(),
            _ => vertices.push(next_tile),
        }

        let next_dir = t ^ previous_tile;
        next_tile = match next_dir {
            NORTH => {
                previous_tile = SOUTH;
                (next_tile.0, next_tile.1 - 1)
            }
            SOUTH => {
                previous_tile = NORTH;
                (next_tile.0, next_tile.1 + 1)
            }
            WEST => {
                previous_tile = EAST;
                (next_tile.0 - 1, next_tile.1)
            }
            EAST => {
                previous_tile = WEST;
                (next_tile.0 + 1, next_tile.1)
            }
            _ => unreachable!(),
        };
    }

    (vertices, points)
}

struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    start: usize,
}

impl Grid {
    fn add_line(&mut self, line: &'_ str) {
        for (i, c) in line.char_indices() {
            let t = tile_from_char(c).unwrap();
            if let Tile::Start = t {
                self.start = self.idx_from_coords((i, self.height));
            }
            self.tiles.push(t);
        }
        self.height += 1;
    }

    fn from_line(line: &'_ str) -> Self {
        let width: usize = line.len();
        let mut tiles: Vec<Tile> = Vec::new();
        let mut start: usize = usize::MAX;

        for (i, c) in line.char_indices() {
            let t = tile_from_char(c).unwrap();
            if let Tile::Start = t {
                start = i;
            }
            tiles.push(t)
        }

        Self {
            tiles,
            width,
            height: 1,
            start,
        }
    }

    fn idx_from_coords(&self, coords: (usize, usize)) -> usize {
        (coords.1 * self.width) + coords.0
    }

    fn coords_from_idx(&self, idx: usize) -> (usize, usize) {
        let y = idx / self.width;
        let x = idx % self.width;
        (x, y)
    }

    fn check_bounds(&self, coords: (usize, usize)) -> bool {
        coords.0 < self.width && coords.1 < self.height
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = (index.1 * self.width) + index.0;
        &self.tiles[idx]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = (index.1 * self.width) + index.0;
        &mut self.tiles[idx]
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Tile {
    Start = 0,
    Ground = 1,
    NS = 0b0000_0110,
    EW = 0b0001_1000,
    NE = 0b0000_1010,
    NW = 0b0001_0010,
    SW = 0b0001_0100,
    SE = 0b0000_1100,
}

fn tile_from_char(c: char) -> Result<Tile, String> {
    match c {
        '|' => Ok(Tile::NS),
        '-' => Ok(Tile::EW),
        'L' => Ok(Tile::NE),
        'J' => Ok(Tile::NW),
        '7' => Ok(Tile::SW),
        'F' => Ok(Tile::SE),
        '.' => Ok(Tile::Ground),
        'S' => Ok(Tile::Start),
        _ => Err("Invalid Tile".to_string()),
    }
}
