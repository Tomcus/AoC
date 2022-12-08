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
    let mut count = 0;
    let height = matrix.len() / width;
    for y in 1..height-1 {
        for x in 1..width-1 {
            let start_index = x + y * width;
            let tree_height = matrix[start_index];
            let mut visible = true;
            for i in 0..x {
                let index = i + y * width;
                if matrix[index] >= tree_height {
                    println!("{};{} - not visible from left {} >= {}", x, y, matrix[index], tree_height);
                    visible = false;
                    break;
                }
            }
            if visible {
                println!("{};{} - visible from left!", x, y);
                count += 1;
                continue;
            }
            visible = true;
            for i in x+1..width {
                let index = i + y * width;
                if matrix[index] >= tree_height {
                    println!("{};{} - not visible from right {} >= {}", x, y, matrix[index], tree_height);
                    visible = false;
                    break;
                }
            }
            if visible {
                println!("{};{} - visible from right!", x, y);
                count += 1;
                continue;
            }
            visible = true;
            for i in 0..y {
                let index = x + i * width;
                if matrix[index] >= tree_height {
                    println!("{};{} - not visible from top {} >= {}", x, y, matrix[index], tree_height);
                    visible = false;
                    break;
                }
            }
            if visible {
                println!("{};{} - visible from top!", x, y);
                count += 1;
                continue;
            }
            visible = true;
            for i in y+1..height {
                let index = x + i * width;
                if matrix[index] >= tree_height {
                    println!("{};{} - not visible from bottom {} >= {}", x, y, matrix[index], tree_height);
                    visible = false;
                    break;
                }
            }
            if visible {
                println!("{};{} - visible from bottom!", x, y);
                count += 1;
                continue;
            }
        }
    }
    count += 2*width;
    count += 2*(height - 2);
    println!("{}", count);
}
