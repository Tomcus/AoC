use anyhow::*;
use common::{map::Map2D, point::Point};
use std::collections::HashSet;
use std::io::{BufRead, Lines};

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
