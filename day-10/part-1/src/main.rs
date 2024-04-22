use core::panic;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const NORTH: Direction = Direction(0, -1);
const SOUTH: Direction = Direction(0, 1);
const EAST: Direction = Direction(1, 0);
const WEST: Direction = Direction(-1, 0);

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let mut input = BufReader::new(f);

    let mut map = Map::new(&mut input);
    println!("{map}");
    map.map_dijkstra();
    println!("{map}");

    let mut ans = 0;
    for t in map.mapped_tiles.iter() {
        let val = map.dijkstra[*t];
        if val > ans {
            ans = val;
        }
    }

    println!("Answer: {ans}");
}

fn invert_direction(dir: Direction) -> Direction {
    Direction(dir.0 * -1, dir.1 * -1)
}

#[derive(Debug)]
struct Map {
    dijkstra: Grid<i16>,
    field: Grid<Tile>,
    start_coords: Point,
    mapped_tiles: Vec<Point>,
}

// Why did I come back to read this
impl Map {
    // Why is this named this way
    // Am I even using a graph? Let alone Dijkstra's algorithm
    fn map_dijkstra(&mut self) {
        let start_coords = self.start_coords;
        let mut starts: Vec<(Point, Direction)> = Vec::new();
        for d in [NORTH, SOUTH, EAST, WEST].iter() {
            let coords = start_coords + *d;
            if !self.field.coords_in_bounds(coords) {
                continue;
            }
            let dir = invert_direction(*d);
            if self.check_connection(coords, dir) {
                starts.push((coords, dir));
            }
        }

        self.dijkstra[start_coords] = 0;
        for path in starts.iter() {
            let (mut coords, mut prev_dir) = path;
            let mut count = 1;
            loop {
                if self.dijkstra[coords] == -1 || self.dijkstra[coords] > count {
                    self.dijkstra[coords] = count;
                    self.mark_tile(coords);
                }

                let Tile::Pipe(tile) = self.field[coords] else {
                    panic!()
                };
                let dir = tile.iter().find(|x| **x != prev_dir).unwrap();
                coords = coords + *dir;
                prev_dir = invert_direction(*dir);
                count += 1;

                if self.dijkstra[coords] == 0 {
                    break;
                }
            }
        }
    }

    fn mark_tile(&mut self, coords: Point) {
        if !self.mapped_tiles.contains(&coords) {
            self.mapped_tiles.push(coords);
        }
    }

    fn check_connection(&self, coords: Point, dir: Direction) -> bool {
        match self.field[coords] {
            Tile::Pipe(p) => p.contains(&dir),
            _ => false,
        }
    }

    fn new(input: &mut BufReader<File>) -> Self {
        let mut field: Vec<Tile> = Vec::new();
        let mut start_coords = Point { x: 0, y: 0 };
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        let width = buf.trim().len();

        let (mut v, i) = Self::parse_line(&buf);
        if let Some(idx) = i {
            start_coords = Point { x: idx, y: 0 };
        }
        field.append(&mut v);

        let mut height = 1;
        for line in input.lines() {
            let (mut v, i) = Self::parse_line(&line.unwrap());
            if let Some(idx) = i {
                start_coords = Point { x: idx, y: height };
            }
            field.append(&mut v);
            height += 1;
        }

        Self {
            dijkstra: Grid::from_vec(vec![-1; width * height], width, height),
            field: Grid::from_vec(field, width, height),
            start_coords,
            mapped_tiles: Vec::new(),
        }
    }

    fn parse_line(line: &'_ str) -> (Vec<Tile>, Option<usize>) {
        let mut res = Vec::new();
        let mut start_idx: Option<usize> = None;
        for c in line.trim().chars() {
            let idx = res.len();
            res.push(match c {
                '|' => Tile::Pipe([NORTH, SOUTH]),
                '-' => Tile::Pipe([EAST, WEST]),
                'L' => Tile::Pipe([NORTH, EAST]),
                'J' => Tile::Pipe([NORTH, WEST]),
                '7' => Tile::Pipe([SOUTH, WEST]),
                'F' => Tile::Pipe([SOUTH, EAST]),
                '.' => Tile::Ground,
                'S' => {
                    start_idx = Some(idx);
                    Tile::Start
                }
                _ => panic!(),
            });
        }
        (res, start_idx)
    }
}

#[derive(Debug)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Direction(isize, isize);
type Pipe = [Direction; 2];

#[derive(Debug)]
struct Grid<T> {
    tiles: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn from_vec(nodes: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            tiles: nodes,
            width,
            height,
        }
    }

    fn coords_in_bounds(&self, coords: Point) -> bool {
        coords.x < self.width && coords.y < self.height
    }

    fn idx_from_coords(&self, coords: Point) -> usize {
        (coords.y as usize * self.width) + coords.x as usize
    }

    fn coords_from_idx(&self, idx: usize) -> Point {
        let y = idx / self.width;
        let x = idx % self.width;
        Point { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl std::ops::Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let x = (self.x as isize + rhs.0) as usize;
        let y = (self.y as isize + rhs.1) as usize;
        Self { x, y }
    }
}

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let idx = self.idx_from_coords(index);
        &self.tiles[idx]
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let idx = self.idx_from_coords(index);
        &mut self.tiles[idx]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}

impl<T: ToString> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for (i, t) in self.tiles.iter().enumerate() {
            if i % self.width == 0 {
                out.push('\n');
            }
            out.push_str(&t.to_string());
        }
        write!(f, "{}", out)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for i in 0..self.field.tiles.len() {
            if i % self.field.width == 0 {
                out.push('\n');
            }

            let dij = self.dijkstra[i];
            if dij != -1 {
                out.push_str(&dij.to_string());
                continue;
            }

            out.push(match self.field[i] {
                Tile::Pipe(p) => match p {
                    [NORTH, SOUTH] => '|',
                    [EAST, WEST] => '-',
                    [NORTH, EAST] => 'L',
                    [NORTH, WEST] => 'J',
                    [SOUTH, WEST] => '7',
                    [SOUTH, EAST] => 'F',
                    _ => panic!(),
                },
                Tile::Ground => '.',
                Tile::Start => 'S',
            });
        }
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_indexing() {
        let v: Vec<u8> = (0..100).collect();
        let grid: Grid<u8> = Grid::from_vec(v, 10, 10);
        let nodes = &grid.tiles;

        assert_eq!(nodes[0], 0);
        assert_eq!(nodes[99], 99);
        assert_eq!(nodes[57], 57);

        let coords0 = grid.coords_from_idx(0);
        let coords1 = grid.coords_from_idx(99);
        let coords2 = grid.coords_from_idx(57);
        assert_eq!(coords0, Point { x: 0, y: 0 });
        assert_eq!(coords1, Point { x: 9, y: 9 });
        assert_eq!(coords2, Point { x: 7, y: 5 });

        assert_eq!(grid[coords0], 0);
        assert_eq!(grid[coords1], 99);
        assert_eq!(grid[coords2], 57);

        // let idx0 = grid.idx_from_coords(coords0);
        // let idx1 = grid.idx_from_coords(coords1);
        // let idx2 = grid.idx_from_coords(coords2);
        // assert_eq!(nodes[idx0], 0);
        // assert_eq!(nodes[idx1], 99);
        // assert_eq!(nodes[idx2], 57);
    }
}
