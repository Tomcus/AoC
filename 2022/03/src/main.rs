use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn to_hash_set(data: &str) -> HashSet<char> {
    let mut res = HashSet::new();
    for c in data.chars() {
        res.insert(c);
    }
    res
}

fn priority(c: u8) -> u8 {
    if c >= ('a' as u8) && c <= ('z' as u8) {
        return c - ('a' as u8) + 1;
    } else if c >= ('A' as u8) && c <= ('Z' as u8) {
        return c - ('A' as u8) + 27;
    }
    0
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut sum = 0 as u64;
    let mut lines = Vec::new();
    for line_raw in BufReader::new(input).lines() {
        let line = line_raw.unwrap();
        lines.push(line);
        if lines.len() == 3 {
            let line_set1 = to_hash_set(&lines[0]);
            let line_set2 = to_hash_set(&lines[1]);
            let line_set3 = to_hash_set(&lines[2]);
            for c in line_set1.iter().filter(|c| line_set2.contains(c)).filter(|c| line_set3.contains(c)) {
                sum += priority(*c as u8) as u64;
                print!("{}", c);
            }
            print!("\n");
            lines.clear();
        }
    }
    println!("{}", sum);
}
