use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, usize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let mut input = BufReader::new(f);

    let mut nodes: Vec<Node> = Vec::new();
    let mut parts: Vec<Part> = Vec::new();

    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let line = line.trim_end();

    let width = line.len();
    let mut height = 1;
    parse_line(line, &mut nodes, &mut parts);
    for line in input.lines() {
        let line = line.unwrap();
        parse_line(&line, &mut nodes, &mut parts);
        height += 1;
    }

    for node in nodes.iter_mut() {
        if let Node::Placeholder(idx) = node {
            *node = Node::PartIndex(*idx);
        }
    }

    let mut grid = Grid::from_vec(nodes, width, height);
    let mut total = 0;
    for i in 0..grid.nodes.len() {
        if grid[i] == Node::Gear {
            let point = grid.coords_from_idx(i);
            total += gear_check(&mut grid, point, &mut parts);
        }
    }

    println!("Answer: {total}");
}

fn gear_check(grid: &mut Grid<Node>, point: Point, parts: &mut Vec<Part>) -> usize {
    let mut ratio = 0;
    let mut adj_parts: Vec<usize> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();
    let width = grid.width as isize;
    let height = grid.height as isize;
    for x in -1..=1 {
        let x = point.x + x;
        if x < 0 || x >= width {
            continue;
        }
        for y in -1..=1 {
            let y = point.y + y;
            if y < 0 || y >= height {
                continue;
            }
            if let Node::PartIndex(idx) = grid[Point { x, y }] {
                let val = parts[idx].count();
                if val != 0 {
                    adj_parts.push(val);
                    indices.push(idx);
                }
            }
        }
    }

    if adj_parts.len() == 2 {
        ratio = adj_parts[0] * adj_parts[1];
    }
    for i in indices {
        parts[i].counted = false;
    }

    ratio
}

fn parse_line(line: &'_ str, nodes: &mut Vec<Node>, parts: &mut Vec<Part>) {
    let mut num = String::new();
    let mut reading_num = false;
    for c in line.chars() {
        if reading_num {
            if c.is_digit(10) {
                num.push(c);
                continue;
            } else {
                let len = num.len();
                let n = num.parse::<usize>().unwrap();
                num.clear();
                let idx = parts.len();
                parts.push(Part::new(n));
                for _ in 0..len {
                    nodes.push(Node::Placeholder(idx));
                }
                reading_num = false;
            }
        }

        if c == '*' {
            nodes.push(Node::Gear);
        } else if c.is_digit(10) {
            num.push(c);
            reading_num = true;
        } else {
            nodes.push(Node::Blank);
        }
    }
    if reading_num {
        let len = num.len();
        let n = num.parse::<usize>().unwrap();
        let idx = parts.len();
        parts.push(Part::new(n));
        for _ in 0..len {
            nodes.push(Node::Placeholder(idx));
        }
    }
}

struct Grid<T> {
    nodes: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn from_vec(nodes: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            nodes,
            width,
            height,
        }
    }

    fn idx_from_coords(&self, coords: Point) -> usize {
        (coords.y as usize * self.width) + coords.x as usize
    }

    fn coords_from_idx(&self, idx: usize) -> Point {
        let y = (idx / self.width) as isize;
        let x = (idx % self.width) as isize;
        Point { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
enum Node {
    PartIndex(usize),
    Placeholder(usize),
    Gear,
    Blank,
}

#[derive(Debug, PartialEq)]
struct Part {
    num: usize,
    counted: bool,
}

impl Part {
    fn new(num: usize) -> Self {
        Self {
            num,
            counted: false,
        }
    }

    fn count(&mut self) -> usize {
        match self.counted {
            false => {
                self.counted = true;
                self.num
            }
            true => 0,
        }
    }
}

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let idx = self.idx_from_coords(index);
        &self.nodes[idx]
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let idx = self.idx_from_coords(index);
        &mut self.nodes[idx]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl std::fmt::Debug for Grid<Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for (i, node) in self.nodes.iter().enumerate() {
            if (i % self.width) == 0 {
                out.push('\n');
            }
            match node {
                Node::Blank => out.push('.'),
                Node::Gear => out.push('*'),
                Node::PartIndex(_) => out.push('#'),
                Node::Placeholder(_) => out.push('P'),
            }
        }

        write!(f, "Width: {}\nHeight: {}\n{}", self.width, self.height, out)
    }
}

impl std::fmt::Display for Grid<Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for (i, node) in self.nodes.iter().enumerate() {
            if (i % self.width) == 0 {
                out.push('\n');
            }
            match node {
                Node::Blank => out.push('.'),
                Node::Gear => out.push('*'),
                Node::PartIndex(_) => out.push('#'),
                Node::Placeholder(_) => out.push('P'),
            }
        }
        let out = out.trim_start();

        write!(f, "{out}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_indexing() {
        let v: Vec<u8> = (0..100).collect();
        let grid: Grid<u8> = Grid::from_vec(v, 10, 10);
        let nodes = &grid.nodes;

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
