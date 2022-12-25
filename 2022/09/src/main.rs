use std::cmp::Eq;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::str::FromStr;
use std::fmt;

const ROPE_LEN: usize = 10;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Point {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }

    fn is_close(&self, other: &Self) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        return dx*dx + dy*dy <= 2;
    }

    fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => { self.x += 1; },
            Direction::Down => { self.x -= 1; },
            Direction::Left => { self.y -= 1; },
            Direction::Right => { self.y += 1; }
        }
    }

    fn move_to(&mut self, point: &Point) {
        if !self.is_close(point) {
            let dx = point.x - self.x;
            let dy = point.y - self.y;
            self.x += dx.signum();
            self.y += dy.signum();
        }
    }
}

fn main() {
    let mut rope: Vec<Point> = std::iter::repeat_with(|| Point::new()).take(ROPE_LEN).collect();
    let mut visited: HashSet<Point> = HashSet::new();
    let input = File::open("input.txt").unwrap();
    for line_raw in BufReader::new(input).lines() {
        let line = line_raw.unwrap();
        let splits: Vec<&str> = line.split(' ').collect();
        let count = u32::from_str(splits[1]).unwrap();
        for _ in 0..count {
            match splits[0] {
                "U" => { rope[0].move_in_direction(Direction::Up); },
                "D" => { rope[0].move_in_direction(Direction::Down); },
                "L" => { rope[0].move_in_direction(Direction::Left); },
                "R" => { rope[0].move_in_direction(Direction::Right); },
                _ => { panic!("wrong instruction"); }
            }
            for i in 1..ROPE_LEN {
                let prev_point = rope[i - 1].clone();
                rope[i].move_to(&prev_point);
            }
            visited.insert(rope[9].clone());
        }
    }
    println!("{}", visited.len());
}
