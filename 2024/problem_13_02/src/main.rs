use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::*;
use std::io::{BufRead, Lines};
use std::ops::{Add, Sub, Mul};

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

impl Mul<Point> for isize {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point(self * rhs.0, self * rhs.1)
    }
}

impl Mul<&Point> for isize {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point(self * rhs.0, self * rhs.1)
    }
}

fn match_line<'a>(line: &'a str, re: &'a Regex) -> Result<regex::Captures<'a>> {
    re.captures(line)
        .ok_or_else(|| anyhow!("Unable to match regex on line: {line}"))
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

const OFFSET: isize = 10000000000000;

fn solve<T>(mut lines: Lines<T>) -> Result<isize>
where
    T: BufRead,
{
    let mut score = 0;

    loop {
        if let Some((button_a, button_b, dest)) = parse_input(&mut lines)? {
            // A*Xa + B*Xb = X + 10**13
            // A*Ya + B*Yb = Y + 10**13
            // ===============
            // A = (X + 10**13 - B*Xb) / Xa
            // ===============
            // ((X + 10**13 - B*Xb) / Xa) * Ya + B*Yb = Y + 10**13
            // ===============
            // ((YaX + Ya*10**13 - BXbYa) / Xa) - Y - 10**13 = -BYb
            // YaX + Ya*10**13 - BXbYa - YXa - Xa*10**13 = -BYbXa
            // YaX - YXa + Ya*10**13 - Xa*10**13 = BXbYa - BYbXa
            // YaX - YXa + Ya*10**13 - Xa*10**13 = B(XbYa - YbXa)
            // (YaX - YXa + Ya*10**13 - Xa*10**13) / (XbYa - YbXa) = B
            let clicked_b = (button_a.1 * dest.0 - dest.1 * button_a.0 + OFFSET * (button_a.1 - button_a.0))
                / (button_b.0 * button_a.1 - button_b.1 * button_a.0);
            let clicked_a = (dest.0 + OFFSET - clicked_b * button_b.0) / button_a.0;
            if clicked_a * button_a + clicked_b * button_b == Point(dest.0 + OFFSET, dest.1 + OFFSET) {
                score += 3 * clicked_a + clicked_b;
            }

            if let Some(line) = lines.next() {
                ensure!(line?.trim().is_empty());
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
