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
        let mut to_process: HashSet<Point> = Default::default();
        let mut new_to_process: HashSet<Point> = Default::default();
        to_process.insert(start.clone());

        while to_process.len() > 0 {
            for current in to_process.iter() {
                let current_height = *map.get(&current);
                if current_height == 9 {
                    score += 1;
                } else {
                    for dir in DIRECTIONS {
                        let new_point = current + &dir;
                        if map.is_on_map(&new_point) && *map.get(&new_point) == current_height + 1 {
                            new_to_process.insert(new_point);
                        }
                    }
                }
            }
            to_process.clear();
            std::mem::swap(&mut new_to_process, &mut to_process);
        }
    }

    Ok(score)
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
        assert_eq!(res, 36);
    }
}
