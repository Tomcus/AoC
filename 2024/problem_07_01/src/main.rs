use anyhow::*;
use std::io::{BufRead, Lines};

fn solve_rec(target: usize, current: usize, remaining: &[usize]) -> usize {
    if remaining.is_empty() {
        return (target == current) as usize;
    }
    return solve_rec(target, current + remaining[0], &remaining[1..])
        + solve_rec(target, current * remaining[0], &remaining[1..]);
}

fn solve<T>(lines: Lines<T>) -> Result<usize>
where
    T: BufRead,
{
    let mut score = 0;
    for line_raw in lines {
        let line = line_raw?;
        let line = line.trim();

        let splits: Vec<&str> = line.split(':').collect();
        ensure!(splits.len() == 2);
        let target = usize::from_str_radix(splits[0], 10)?;
        let pieces: Result<Vec<usize>, _> = splits[1]
            .trim()
            .split(' ')
            .map(|raw| usize::from_str_radix(raw, 10))
            .collect();
        let pieces = pieces?;

        if pieces.len() == 1 && pieces[0] == target {
            score += target;
            continue;
        }

        if solve_rec(target, pieces[0], &pieces[1..]) > 0 {
            score += target;
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
        let input = std::io::Cursor::new(
            b"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 3749);
    }
}
