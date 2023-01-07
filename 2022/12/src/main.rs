use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i64,
    y: i64
}

impl Position {
    fn in_bounds(&self, width: i64, height: i64) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

fn can_visit(a: char, b: char) -> bool {
    let mut aa = a;
    if aa == 'S' {
        aa = 'a';
    }
    let mut bb = b;
    if bb == 'E' {
        bb = 'z';
    }
    let can_visit = (aa as i32) - (bb as i32) >= -1;
    can_visit
}

const UP: Position = Position { x: -1, y: 0 };
const DOWN: Position = Position { x: 1, y: 0 };
const LEFT: Position = Position { x: 0, y: -1 };
const RIGHT: Position = Position { x: 0, y: 1 };

fn find_path(grid: &str, start: &Position, end: &Position, width: i64, height: i64) {
    let mut to_visit: HashSet<Position> = HashSet::new();
    to_visit.insert(start.clone());
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(start.clone());
    let mut len = 0;
    'outer :while to_visit.len() > 0 {
        let mut new_visit = HashSet::new();
        for pos in to_visit {
            if pos == *end {
                println!("Found!");
                break 'outer;
            } else {
                let mut new_pos = pos.clone() + UP;
                if new_pos.in_bounds(width, height) && !visited.contains(&new_pos) &&
                   can_visit(grid.chars().nth((pos.x + pos.y * width) as usize).unwrap(),
                             grid.chars().nth((new_pos.x + new_pos.y * width) as usize).unwrap()) {
                    new_visit.insert(new_pos);
                }
                new_pos = pos.clone() + DOWN;
                if new_pos.in_bounds(width, height) && !visited.contains(&new_pos) &&
                   can_visit(grid.chars().nth((pos.x + pos.y * width) as usize).unwrap(),
                             grid.chars().nth((new_pos.x + new_pos.y * width) as usize).unwrap()) {
                    new_visit.insert(new_pos);
                }
                new_pos = pos.clone() + LEFT;
                if new_pos.in_bounds(width, height) && !visited.contains(&new_pos) &&
                   can_visit(grid.chars().nth((pos.x + pos.y * width) as usize).unwrap(),
                             grid.chars().nth((new_pos.x + new_pos.y * width) as usize).unwrap()) {
                    new_visit.insert(new_pos);
                }
                new_pos = pos.clone() + RIGHT;
                if new_pos.in_bounds(width, height) && !visited.contains(&new_pos) &&
                   can_visit(grid.chars().nth((pos.x + pos.y * width) as usize).unwrap(),
                             grid.chars().nth((new_pos.x + new_pos.y * width) as usize).unwrap()) {
                    new_visit.insert(new_pos);
                }
            }
            visited.insert(pos);
        }
        to_visit = new_visit;
        len += 1;
    }
    println!("Len: {}", len);
}

fn main() {
    let input = File::open("input.txt").unwrap();
    
    let mut width = None;
    let mut height = 0;
    let mut grid = String::new();
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    
    for line_raw in BufReader::new(input).lines() {
            let line = line_raw.unwrap();
            if line.len() > 0 {
            if width == None {
                width = Some(line.len() as i64);
            }
            if line.contains('S') {
                start.x = line.find('S').unwrap() as i64;
                start.y = height;
            }
            if line.contains('E') {
                end.x = line.find('E').unwrap() as i64;
                end.y = height;
            }
            height += 1;
            grid += &line;
        }
    }
    println!("Loaded");
    find_path(&grid, &start, &end, width.unwrap(), height);
}

