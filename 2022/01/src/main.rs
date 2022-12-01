use std::fs::File;
use std::io::{BufReader, prelude::*};

struct CaloriesCounter {
    sum: u64,
    max_cal: u64
}

impl CaloriesCounter {
    fn new() -> Self {
        Self {
            sum: 0,
            max_cal: 0
        }
    }

    fn process_line(&mut self, line: &str) {
        match line.parse::<u64>() {
            Ok(calories) => {
                self.sum += calories;
            },
            Err(_e) => {
                if self.sum > self.max_cal {
                    self.max_cal = self.sum;
                }
                self.sum = 0;
            }
        }
    }
}

fn main() {
    let mut counter = CaloriesCounter::new();
    let input = File::open("./input.txt").unwrap();
    let reader = BufReader::new(input);
    for line_res in reader.lines() {
        match line_res {
            Ok(line) => counter.process_line(&line),
            Err(_e) => break
        }
    }
    println!("{}", counter.max_cal);
}
