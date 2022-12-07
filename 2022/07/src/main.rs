use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

const TOTAL_DISC_SPACE: u64 = 70000000;
const SPACE_NEEDED: u64 = 30000000;

fn main() {
    let mut all_sizes: Vec<u64> = vec![];
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
                        all_sizes.push(dir_size);
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
        all_sizes.push(dir_size);
        if sizes.len() > 0 {
            let index = sizes.len() - 1;
            sizes[index] += dir_size;
        }
    }
    all_sizes.sort();
    let max_index = all_sizes.len() - 1;
    let used_space = all_sizes[max_index];
    let free_space = TOTAL_DISC_SPACE - used_space;
    let to_free = SPACE_NEEDED - free_space;
    let mut index = all_sizes.len() / 2;
    let mut jump_by = all_sizes.len() / 4;
    loop {
        if jump_by == 0 {
            break;
        }
        if all_sizes[index] >= to_free {
            index -= jump_by;
        } else {
            index += jump_by;
        }
        jump_by /= 2;
    }
    println!("To free: {} First Ok: {}", to_free, all_sizes[index]);
}
