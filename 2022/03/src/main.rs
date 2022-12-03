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
    for line_raw in BufReader::new(input).lines() {
        let line = line_raw.unwrap();
        let comp1 = &line[0..(line.len()/2)];
        let comp2 = &line[(line.len()/2)..(line.len())];
        let data1 = to_hash_set(comp1);
        let data2 = to_hash_set(comp2);
        let intersection = data1.intersection(&data2);
        for letter in intersection {
            let priority = priority(*letter as u8);
            println!("{} -> {}", *letter, priority);
            sum += priority as u64;
        }
    }
    println!("{}", sum);
}
