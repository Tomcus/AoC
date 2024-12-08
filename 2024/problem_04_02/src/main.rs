use anyhow::*;
use std::io::{BufRead, Lines};

const TO_CHECK: [(isize, isize); 2] = [
    (1, 1), (-1, 1) 
];

const CHARS: [char; 2] = ['M', 'S'];

fn invert_char(character: char) -> Result<char> {
    Ok(match character {
        'M' => 'S',
        'S' => 'M',
        _ => bail!("Invalid input {character}"),
    })
}

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
    'outer: for (index, character) in matrix.iter().enumerate() {
        if *character == 'A' {
            let origin_x = (index % width) as isize;
            let origin_y = (index / width) as isize;
            for (off_x, off_y) in TO_CHECK {
                let width = width as isize;
                let x = origin_x + off_x;
                let y = origin_y + off_y;
                if x < 0 || x >= width || y < 0 || y >= height {
                    continue 'outer;
                }
                let character = matrix[(y * width + x) as usize]; 
                if !CHARS.contains(&character) {
                    continue 'outer;
                }
                let oposite_x = origin_x - off_x;
                let oposite_y = origin_y - off_y;
                if oposite_x < 0 || oposite_x >= width || oposite_y < 0 || oposite_y >= height {
                    continue 'outer;
                }
                let oposite_character = invert_char(character)?;
                if matrix[(oposite_y * width + oposite_x) as usize] != oposite_character {
                    continue 'outer;
                }
            }
            score += 1;
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
        assert_eq!(res, 9);
    }
}
