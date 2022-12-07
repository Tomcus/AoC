use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

const FILE_SIZE_THRESHOLD: u64 = 100000;

fn main() {
    let mut sum_of_sizes: u64 = 0;
    let mut sizes: Vec<u64> = vec![];
    let input = File::open("input.txt").unwrap();
    let fb = BufReader::new(input);
    for line_raw in fb.lines() {
        let line = line_raw.unwrap();
        let splits: Vec<&str> = line.split(' ').collect();
        if splits[0] == "$" {
            println!("CMD: {}", line);
            if splits.len() == 3 && splits[1] == "cd" {
                match splits[2] {
                    ".." => {
                        let dir_size = sizes.pop().unwrap();
                        if dir_size <= FILE_SIZE_THRESHOLD {
                            sum_of_sizes += dir_size;
                        }
                        let index = sizes.len() - 1;
                        sizes[index] += dir_size;
                        println!("{:?}", sizes);
                    },
                    _ => {
                        sizes.push(0);
                        println!("{:?}", sizes);
                    }
                }
            }
        } else {
            println!("OUT: {}", line);
            if splits[0] != "dir" {
                let index = sizes.len() - 1;
                sizes[index] += u64::from_str(splits[0]).unwrap();
                println!("{:?}", sizes);
            }
        }
    }
    while sizes.len() > 0 {
        let dir_size = sizes.pop().unwrap();
        if dir_size <= FILE_SIZE_THRESHOLD {
            sum_of_sizes += dir_size;
            if sizes.len() > 0 {
                let index = sizes.len() - 1;
                sizes[index] += dir_size;
            }
        }
    }
    println!("Sum: {}", sum_of_sizes);
}
