use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::*;
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

fn match_line<'a>(line: &'a str, re: &'a Regex) -> Result<regex::Captures<'a>> {
    re.captures(line)
        .ok_or_else(|| anyhow!("Unable to match regex on line"))
}

fn parse_input<T>(lines: &mut Lines<T>) -> Result<Option<(Point, Point, Point)>>
where
    T: BufRead,
{
    lazy_static! {
        static ref BTNA_RE: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        static ref BTNB_RE: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        static ref PRIZE_LOC: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    }
    let button_a = if let Some(line) = lines.next() {
        let line = line?;
        let cap = match_line(&line, &BTNA_RE)?;
        let x = isize::from_str_radix(
            cap.get(1)
                .ok_or_else(|| anyhow!("Unable to extract x"))?
                .into(),
            10,
        )?;
        let y = isize::from_str_radix(
            cap.get(2)
                .ok_or_else(|| anyhow!("Unable to extract y"))?
                .into(),
            10,
        )?;
        Point(x, y)
    } else {
        return Ok(None);
    };
    let button_b = {
        let line = lines
            .next()
            .ok_or_else(|| anyhow!("Unable to extract button B string"))??;
        let cap = match_line(&line, &BTNB_RE)?;
        let x = isize::from_str_radix(
            cap.get(1)
                .ok_or_else(|| anyhow!("Unable to extract x"))?
                .into(),
            10,
        )?;
        let y = isize::from_str_radix(
            cap.get(2)
                .ok_or_else(|| anyhow!("Unable to extract y"))?
                .into(),
            10,
        )?;
        Point(x, y)
    };
    let dest = {
        let line = lines
            .next()
            .ok_or_else(|| anyhow!("Unable to extract button location string"))??;
        let cap = match_line(&line, &PRIZE_LOC)?;
        let x = isize::from_str_radix(
            cap.get(1)
                .ok_or_else(|| anyhow!("Unable to extract x"))?
                .into(),
            10,
        )?;
        let y = isize::from_str_radix(
            cap.get(2)
                .ok_or_else(|| anyhow!("Unable to extract y"))?
                .into(),
            10,
        )?;
        Point(x, y)
    };
    Ok(Some((button_a, button_b, dest)))
}

#[derive(Eq, Debug)]
struct QueueItem {
    pub remaining: Point,
    pub pressed: Point,
}

impl QueueItem {
    pub fn is_valid(&self) -> bool {
        self.remaining.0 >= 0
            && self.remaining.1 >= 0
            && self.pressed.0 <= 100
            && self.pressed.1 <= 100
    }

    pub fn token_cost(&self) -> u16 {
        self.pressed.0 as u16 * 3 + self.pressed.1 as u16
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.token_cost().eq(&other.token_cost())
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.token_cost().partial_cmp(&other.token_cost())
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.token_cost().cmp(&other.token_cost())
    }
}

fn solve<T>(mut lines: Lines<T>) -> Result<isize>
where
    T: BufRead,
{
    let mut score = 0;

    loop {
        if let Some((button_a, button_b, dest)) = parse_input(&mut lines)? {
            let mut queue = std::collections::BinaryHeap::new();
            let mut visited = std::collections::HashSet::new();
            visited.insert(Point(0, 0));

            queue.push(Reverse(QueueItem {
                remaining: dest,
                pressed: Point(0, 0),
            }));

            while let Some(Reverse(queue_item)) = queue.pop() {
                if !queue_item.is_valid() {
                    continue;
                }
                if queue_item.remaining == Point(0, 0) {
                    score += queue_item.token_cost() as isize;
                    println!("Solution: {}", queue_item.token_cost());
                    break;
                }

                let a_pressed_comb = &queue_item.pressed + &Point(1, 0);
                if !visited.contains(&a_pressed_comb) {
                    queue.push(Reverse(QueueItem {
                        remaining: &queue_item.remaining - &button_a,
                        pressed: a_pressed_comb.clone(),
                    }));
                    visited.insert(a_pressed_comb);
                }

                let b_pressed_comb = &queue_item.pressed + &Point(0, 1);
                if !visited.contains(&b_pressed_comb) {
                    queue.push(Reverse(QueueItem {
                        remaining: &queue_item.remaining - &button_b,
                        pressed: b_pressed_comb.clone(),
                    }));
                    visited.insert(b_pressed_comb);
                }
            }

            if let Some(line) = lines.next() {
                let line = line?;
                ensure!(line.trim() == "");
            } else {
                break;
            }
        } else {
            break;
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
        let input = b"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let input = std::io::BufReader::new(std::io::Cursor::new(input));
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 480);
    }
}
