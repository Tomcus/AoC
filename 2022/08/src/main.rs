use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut matrix: Vec<u8> = vec![];
    let mut width = 0;
    for raw_line in BufReader::new(input).lines() {
        let line = raw_line.unwrap();
        matrix.extend(line.chars().filter(|c| {
            c.is_numeric()
        }).map(|c| {
            u8::from_str(&c.to_string()).unwrap()
        }));
        if width == 0 {
            width = matrix.len();
        }
    }
    let mut max = 0;
    let height = matrix.len() / width;
    for y in 1..height-1 {
        for x in 1..width-1 {
            let start_index = x + y * width;
            let tree_height = matrix[start_index];
            let mut curr_count = 0;
            let mut total_count = 1;
            for i in (0..x).rev() {
                let index = i + y * width;
                curr_count += 1;
                if matrix[index] >= tree_height {
                    break;
                }
            }
            total_count *= curr_count;
            curr_count = 0;
            for i in x+1..width {
                let index = i + y * width;
                curr_count += 1;
                if matrix[index] >= tree_height {
                    break;
                }
            }
            total_count *= curr_count;
            curr_count = 0;
            for i in (0..y).rev() {
                let index = x + i * width;
                curr_count += 1;
                if matrix[index] >= tree_height {
                    break;
                }
            }
            total_count *= curr_count;
            curr_count = 0;
            for i in y+1..height {
                let index = x + i * width;
                curr_count += 1;
                if matrix[index] >= tree_height {
                    break;
                }
            }
            total_count *= curr_count;
            if total_count > max {
                max = total_count;
            }
        }
    }
    println!("{}", max);
}
