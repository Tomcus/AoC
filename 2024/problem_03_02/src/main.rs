use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Read;

fn solve(input: &str) -> Result<isize> {
    lazy_static! {
        static ref FNC_CALL: Regex = Regex::new(r"(mul|don't|do)\(([0-9,]*)\)").unwrap();
        static ref MUL_ARGS: Regex = Regex::new(r"(\d{1,3}),(\d{1,3})").unwrap();
    }
    let mut score = 0;
    let mut enabled = true;
    for capture in FNC_CALL.captures_iter(input) {
        let fn_call = capture.get(1).unwrap().as_str();
        match fn_call {
            "do" => enabled = true,
            "don't" => enabled = false,
            "mul" => {
                let capture = MUL_ARGS
                    .captures(capture.get(2).unwrap().as_str())
                    .ok_or_else(|| anyhow!("Unable to capture mul args!"))?;
                let lhs = isize::from_str_radix(capture.get(1).unwrap().as_str(), 10)?;
                let rhs = isize::from_str_radix(capture.get(2).unwrap().as_str(), 10)?;
                println!("{lhs} * {rhs} = {}", lhs * rhs);
                if enabled {
                    score += lhs * rhs;
                }
            }
            _ => bail!("Unknown function call {}", fn_call),
        };
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let res = solve(&input).unwrap();
        assert_eq!(res, 48);
    }
}
