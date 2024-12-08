use anyhow::*;
use std::io::{BufRead, Lines};

const OPTIONS: [[(isize, isize); 3]; 8] = [
    [( 0,  1), ( 0,  2), ( 0,  3)],
    [( 0, -1), ( 0, -2), ( 0, -3)],
    [( 1,  0), ( 2,  0), ( 3,  0)],
    [(-1,  0), (-2,  0), (-3,  0)],
    [( 1,  1), ( 2,  2), ( 3,  3)],
    [(-1, -1), (-2, -2), (-3, -3)],
    [( 1, -1), ( 2, -2), ( 3, -3)],
    [(-1,  1), (-2,  2), (-3,  3)],
];

const CHARS: [char; 3] = ['M', 'A', 'S'];

fn solve<T>(lines: Lines<T>) -> Result<isize> where T: BufRead {
    let mut score = 0;
    let mut matrix: Vec<char> = vec![];
    let mut width: Option<usize> = None;
    let mut height = 0;

    for line_raw in lines {
        let line = line_raw?;
        if width.is_none() {
            width = Some(line.trim().len());
        }
        matrix.extend(line.trim().chars());
        height += 1;
    }

    let width = width.unwrap();
    for (index, character) in matrix.iter().enumerate() {
        if *character == 'X' {
            let origin_x = (index % width) as isize;
            let origin_y = (index / width) as isize;
            'outer: for directions in OPTIONS {
                let width = width as isize;
                for (index, (off_x, off_y)) in directions.iter().enumerate() {
                    let x = origin_x + off_x;
                    let y = origin_y + off_y;
                    if x < 0 || x >= width || y < 0 || y >= height {
                        continue 'outer;
                    }
                    if matrix[(y * width + x) as usize] != CHARS[index] {
                        continue 'outer;
                    }
                }
                score += 1;
            }
        }
    }
    
    Ok(score)
}

fn main() -> Result<()> {
    let res = solve(std::io::stdin().lines())?;
    println!("Result: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let input = std::io::Cursor::new(b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 18);
    }
}
