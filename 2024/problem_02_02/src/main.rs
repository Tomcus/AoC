use anyhow::*;
use std::io::{BufRead, Lines};

enum Order {
    Unknown,
    Descreasing,
    Increasing,
}

fn validate(numbers: &[isize]) -> (usize, bool) {
    let mut order = Order::Unknown;
    for (left_index, items) in numbers.windows(2).enumerate() {
        let left = items[0];
        let right = items[1];
        let abs = (left - right).abs();

        if abs < 1 || abs > 3 {
            return (left_index as usize, false);
        }

        match order {
            Order::Unknown => {
                if left > right {
                    order = Order::Descreasing;
                } else {
                    order = Order::Increasing;
                }
            }
            Order::Descreasing => {
                if left < right {
                    return (left_index as usize, false);
                }
            }
            Order::Increasing => {
                if left > right {
                    return (left_index as usize, false);
                }
            }
        }
    }
    return (0, true);
}

fn solve<T>(lines: Lines<T>) -> Result<isize>
where
    T: BufRead,
{
    let mut score = 0;
    for line_raw in lines {
        let line = line_raw?;
        let mut numbers: Vec<isize> = line
            .split(' ')
            .map(|a| isize::from_str_radix(a, 10).unwrap())
            .collect();
        if let (to_remove, false) = validate(&numbers) {
            let original_numbers = numbers.clone();
            numbers.remove(to_remove);
            if let (_, true) = validate(&numbers) {
                score += 1;
                println!(
                    "{:?}: Safe by removing the level {}.",
                    original_numbers, to_remove
                );
            } else {
                let mut numbers = original_numbers.clone();
                numbers.remove(to_remove + 1);
                if let (_, true) = validate(&numbers) {
                    score += 1;
                    println!(
                        "{:?}: Safe by removing the level {}.",
                        original_numbers,
                        original_numbers.len() - 1
                    );
                } else {
                    let mut numbers = original_numbers.clone();
                    numbers.remove(0);
                    if let (_, true) = validate(&numbers) {
                        score += 1;
                        println!(
                            "{:?}: Safe by removing the level {}.",
                            original_numbers,
                            original_numbers.len() - 1
                        );
                    } else {
                        println!(
                            "{:?}: Unsafe regardless of which level is removed.",
                            original_numbers
                        )
                    }
                }
            }
        } else {
            score += 1;
            println!("{:?}: Safe without removing any level.", numbers);
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
            b"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 4);
    }

    #[test]
    fn extra_tests() {
        {
            let input = std::io::Cursor::new(b"7 6 5 4 3 8");
            let res = solve(input.lines()).unwrap();
            assert_eq!(res, 1);
        }
        {
            let input = std::io::Cursor::new(b"8 7 6 4 5 4 3");
            let res = solve(input.lines()).unwrap();
            assert_eq!(res, 1);
        }
        {
            let input = std::io::Cursor::new(b"4 6 5 3 2");
            let res = solve(input.lines()).unwrap();
            assert_eq!(res, 1);
        }
    }
}
