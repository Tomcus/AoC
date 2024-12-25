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

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

const DIRECTIONS: [Point; 4] = [Point(1, 0), Point(-1, 0), Point(0, 1), Point(0, -1)];

fn solve<T>(lines: Lines<T>) -> Result<isize>
where
    T: BufRead,
{
    let map = Map2D::load(lines)?;
    let mut score = 0;
    let mut visited: HashSet<Point> = Default::default();

    for y in 0..map.get_height() {
        for x in 0..map.get_width() {
            let start_point = Point(x as isize, y as isize);
            if !visited.contains(&start_point) {
                let mut area = 0;
                let mut to_visit: HashSet<Point> = Default::default();
                let mut future_to_visit: HashSet<Point> = Default::default();
                to_visit.insert(start_point.clone());
                let mut perimeter = 0;
                let test_char = map.get(&start_point);
                loop {
                    for point in &to_visit {
                        visited.insert(point.clone());
                        area += 1;
                        for dir in DIRECTIONS {
                            let test_point = point.clone() + dir;
                            if map.is_on_map(&test_point) && map.get(&test_point) == test_char {
                                if !visited.contains(&test_point) {
                                    future_to_visit.insert(test_point);
                                }
                            } else {
                                perimeter += 1;
                            }
                        }
                    }
                    if future_to_visit.len() == 0 {
                        score += area * perimeter;
                        break;
                    }
                    std::mem::swap(&mut to_visit, &mut future_to_visit);
                    future_to_visit.clear();
                }
            }
        }
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
    fn test_example1() {
        let input = std::io::Cursor::new(
            b"AAAA
BBCD
BBCC
EEEC",
        );
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 140);
    }

    #[test]
    fn test_example2() {
        let input = std::io::Cursor::new(
            b"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        );
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 772);
    }

    #[test]
    fn test_example3() {
        let input = std::io::Cursor::new(
            b"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        );
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 1930);
    }
}
