use std::fs::File;
use std::io::{prelude::*, BufReader};

struct CaloriesCounter {
    sum: u64,
    max_cal: [u64; 3],
}

impl CaloriesCounter {
    fn new() -> Self {
        Self {
            sum: 0,
            max_cal: [0, 0, 0],
        }
    }

    fn process_line(&mut self, line: &str) {
        match line.parse::<u64>() {
            Ok(calories) => {
                self.sum += calories;
            }
            Err(_e) => {
                self.process_sum();
            }
        }
    }

    fn process_sum(&mut self) {
        if self.sum > self.max_cal[0] {
            self.max_cal[2] = self.max_cal[1];
            self.max_cal[1] = self.max_cal[0];
            self.max_cal[0] = self.sum;
        } else if self.sum > self.max_cal[1] {
            self.max_cal[2] = self.max_cal[1];
            self.max_cal[1] = self.sum;
        } else if self.sum > self.max_cal[2] {
            self.max_cal[2] = self.sum;
        }
        println!(
            "{} => {}, {}, {}",
            self.sum, self.max_cal[0], self.max_cal[1], self.max_cal[2]
        );
        self.sum = 0;
    }
}

fn sum(numbers: &[u64]) -> u64 {
    let mut sum = 0 as u64;
    for number in numbers.iter() {
        sum += number;
    }
    sum
}

fn main() {
    let mut counter = CaloriesCounter::new();
    let input = File::open("./input.txt").unwrap();
    let reader = BufReader::new(input);
    for line_res in reader.lines() {
        match line_res {
            Ok(line) => counter.process_line(&line),
            Err(_e) => break,
        }
    }
    counter.process_sum();
    println!("{}", sum(&counter.max_cal));
}
