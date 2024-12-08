use anyhow::*;
use std::io::{BufRead, Lines};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

fn insert(map: &mut HashMap<isize, isize>, val: isize) {
    if map.contains_key(&val) {
        map.insert(val, map.get(&val).unwrap() + 1);
    } else {
        map.insert(val, 1);
    }
}

fn solve<T>(lines: Lines<T>) -> Result<isize> where T: BufRead {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    }
    let mut left = HashMap::new();
    let mut right = HashMap::new();

    for line_raw in lines {
        let line = line_raw?;
        let match_res = RE.captures(&line).ok_or_else(|| anyhow!("Unable to match regex"))?;
        let left_num = isize::from_str_radix(match_res.get(1).unwrap().as_str(), 10)?;
        insert(&mut left, left_num);
        let right_num = isize::from_str_radix(match_res.get(2).unwrap().as_str(), 10)?;
        insert(&mut right, right_num);
    }
    
    let mut score = 0;
    for (number, times_in_list) in left.iter() {
        let in_right = right.get(&number).or(Some(&0)).unwrap();
        score += number * times_in_list * in_right;
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
        let input = std::io::Cursor::new(b"3   4
4   3
2   5
1   3
3   9
3   3");
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 31);
    }
}
