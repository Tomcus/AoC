use anyhow::*;
use common::point::Point;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};

fn is_on_map(point: &Point, width: isize, height: isize) -> bool {
    point.0 >= 0 && point.0 < width && point.1 >= 0 && point.1 < height
}

fn is_antena(input: char) -> bool {
    input.is_ascii_alphanumeric()
}

fn solve<T>(lines: Lines<T>) -> Result<isize>
where
    T: BufRead,
{
    let mut width: Option<isize> = None;
    let mut locations: HashMap<char, HashSet<Point>> = Default::default();
    let mut y = 0 as isize;

    for line_raw in lines {
        let line = line_raw?;
        let line = line.trim();
        if width.is_none() {
            width = Some(line.len() as isize);
        }

        for (x, character) in line.chars().enumerate() {
            if is_antena(character) {
                match locations.get_mut(&character) {
                    Some(points) => {
                        points.insert(Point(x as isize, y));
                    }
                    None => {
                        let mut points: HashSet<Point> = Default::default();
                        points.insert(Point(x as isize, y));
                        locations.insert(character, points);
                    }
                }
            }
        }
        y += 1;
    }
    let height = y;
    let width = width.unwrap();

    let mut points: HashSet<Point> = Default::default();
    for (_, to_check) in locations.iter() {
        let to_check: Vec<_> = to_check.iter().collect();
        for i in 0..(to_check.len() - 1) {
            for j in (i + 1)..to_check.len() {
                let point_a = to_check[i];
                let point_b = to_check[j];
                let diff = point_b - point_a;

                let new_point_a = point_a - &diff;
                if is_on_map(&new_point_a, width, height) {
                    points.insert(new_point_a);
                }

                let new_point_b = point_b + &diff;
                if is_on_map(&new_point_b, width, height) {
                    points.insert(new_point_b);
                }
            }
        }
    }

    Ok(points.len() as isize)
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
            b"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 14);
    }
}
