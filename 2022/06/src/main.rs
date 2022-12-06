use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line = String::new();
    BufReader::new(input).read_line(&mut line).unwrap();
    for i in 14..line.len() {
        let substr = &line[i-14..i];
        let set: HashSet<char> = HashSet::from_iter(substr.chars());
        if set.len() == 14 {
            println!("{}", i);
            break;
        }
    }
}
