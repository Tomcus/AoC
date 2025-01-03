use anyhow::*;
use common::{map::Map2D, point::Point};
use std::collections::HashSet;
use std::io::{BufRead, Lines};

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
