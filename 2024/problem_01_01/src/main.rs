use anyhow::*;
use std::io::{BufRead, Lines};
use regex::Regex;
use lazy_static::lazy_static;

fn solve<T>(lines: Lines<T>) -> Result<isize> where T: BufRead {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    }
    let mut left = vec![];
    let mut right = vec![];

    for line_raw in lines {
        let line = line_raw?;
        let match_res = RE.captures(&line).ok_or_else(|| anyhow!("Unable to match regex"))?;
        left.push(isize::from_str_radix(match_res.get(1).unwrap().as_str(), 10)?);
        right.push(isize::from_str_radix(match_res.get(2).unwrap().as_str(), 10)?);
    }
    left.sort();
    right.sort();
    Ok(left.iter().zip(right).map(|(left, right)| (left - right).abs()).fold(0, |acc, x| acc + x))
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
        let input = std::io::Cursor::new(b"3   4
4   3
2   5
1   3
3   9
3   3");
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 11);
    }
}
