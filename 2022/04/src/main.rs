use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;

struct CleaningRange {
    begin: u64,
    end: u64
}

impl FromStr for CleaningRange {
    type Err = ParseIntError;

    fn from_str(range: &str) -> Result<Self, Self::Err> {
        match range.find('-') {
            Some(index) => {
                let begin = u64::from_str(&range[0..index])?;
                let end = u64::from_str(&range[index + 1..range.len()])?;
                Ok(CleaningRange { begin, end })
            },
            None => {
                panic!("Unable to find range delimiter '-'")
            }
        }
    }
}

impl CleaningRange {
    fn fully_contains(&self, other: &Self) -> bool {
        self.begin >= other.begin && self.end <= other.end
    }
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut cnt = 0;
    for line_raw in BufReader::new(input).lines() {
        let line = line_raw.unwrap();
        match line.find(',') {
            Some(index) => {
                let range_a = CleaningRange::from_str(&line[0..index]).unwrap();
                let range_b = CleaningRange::from_str(&line[index + 1..line.len()]).unwrap();
                if range_a.fully_contains(&range_b) || range_b.fully_contains(&range_a) {
                    cnt += 1;
                }
            },
            None => break
        }
    }
    println!("{}", cnt);
}
