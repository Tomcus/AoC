use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Read;

fn solve(input: &str) -> Result<isize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    }
    let mut score = 0;
    for capture in RE.captures_iter(input) {
        let lhs = isize::from_str_radix(capture.get(1).unwrap().as_str(), 10)?;
        let rhs = isize::from_str_radix(capture.get(2).unwrap().as_str(), 10)?;
        //println!("{lhs} * {rhs} = {}", lhs * rhs);
        score += lhs * rhs;
    }
    Ok(score)
}

fn main() -> Result<()> {
    let mut buffer = Vec::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut buffer)?;
    let input = String::from_utf8(buffer)?;
    let res = solve(&input)?;
    println!("Result: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let res = solve(&input).unwrap();
        assert_eq!(res, 161);
    }
}
