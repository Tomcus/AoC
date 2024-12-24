use anyhow::*;
use std::collections::HashSet;
use std::io::{BufRead, Lines};
use std::ops::{Add, Sub};

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub for &Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct Map2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Map2D<char> {
    pub fn load<T>(lines: Lines<T>) -> Result<Self>
    where
        T: BufRead,
    {
        let mut width: Option<usize> = None;
        let mut height = 0usize;
        let mut data = vec![];
        for line_raw in lines {
            let line = line_raw?;
            let line = line.trim();
            if width.is_none() {
                width = Some(line.len());
            }
            for character in line.chars() {
                data.push(character);
            }
            height += 1;
        }

        Ok(Self {
            data,
            width: width.unwrap(),
            height,
        })
    }
}

impl<From> Map2D<From> {
    pub fn map<Into>(self, predicate: impl FnMut(&From) -> Result<Into>) -> Result<Map2D<Into>> {
        let res: Result<Vec<_>, _> = self.data.iter().map(predicate).collect();
        Ok(Map2D::<Into> {
            data: res?,
            width: self.width,
            height: self.height,
        })
    }
}

impl<T> Map2D<T> {
    pub fn for_each(&self, callback: &mut impl FnMut(Point, &T) -> Result<()>) -> Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point(x as isize, y as isize);
                let val = self.get(&point);
                callback(point, val)?;
            }
        }
        Ok(())
    }

    pub fn is_on_map(&self, point: &Point) -> bool {
        point.0 >= 0
            && point.0 < self.width as isize
            && point.1 >= 0
            && point.1 < self.height as isize
    }

    pub fn get(&self, point: &Point) -> &T {
        &self.data[(point.0 + point.1 * self.width as isize) as usize]
    }
}

const DIRECTIONS: [Point; 4] = [Point(1, 0), Point(-1, 0), Point(0, 1), Point(0, -1)];

fn solve<T>(lines: Lines<T>) -> Result<isize>
where
    T: BufRead,
{
    let map = Map2D::load(lines)?.map(|character| {
        ensure!(character.is_ascii_digit());
        Ok(*character as u8 - '0' as u8)
    })?;
    let mut score = 0;
    let mut start_points: HashSet<Point> = Default::default();
    let mut extractor = |point, val: &u8| {
        if *val == 0 {
            start_points.insert(point);
        }
        Ok(())
    };
    map.for_each(&mut extractor)?;

    for start in start_points {
        let mut to_process: Vec<Point> = Default::default();
        let mut new_to_process: Vec<Point> = Default::default();
        to_process.push(start.clone());

        for expected_height in 1..=9 {
            for current in to_process.iter() {
                for dir in DIRECTIONS {
                    let new_point = current + &dir;
                    if map.is_on_map(&new_point) && *map.get(&new_point) == expected_height {
                        new_to_process.push(new_point);
                    }
                }
            }
            to_process.clear();
            std::mem::swap(&mut new_to_process, &mut to_process);
        }
        score += to_process.len();
    }

    Ok(score as isize)
}

fn main() -> Result<()> {
    let res = solve(std::io::stdin().lines())?;
    println!("Result: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let input = std::io::Cursor::new(
            b"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 81);
    }
}
