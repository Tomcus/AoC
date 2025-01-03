use anyhow::*;
use std::io::{BufRead, Lines};

enum Order {
    Unknown,
    Descreasing,
    Increasing,
}

fn solve<T>(lines: Lines<T>) -> Result<isize>
where
    T: BufRead,
{
    let mut score = 0;
    for line_raw in lines {
        let line = line_raw?;
        let numbers: Vec<isize> = line
            .split(' ')
            .map(|a| isize::from_str_radix(a, 10).unwrap())
            .collect();
        let mut order = Order::Unknown;
        let mut ok = true;
        for items in numbers.windows(2) {
            ensure!(items.len() == 2);
            let left = items[0];
            let right = items[1];
            let abs = (left - right).abs();

            if abs < 1 || abs > 3 {
                ok = false;
                break;
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
                        ok = false;
                        break;
                    }
                }
                Order::Increasing => {
                    if left > right {
                        ok = false;
                        break;
                    }
                }
            }
        }

        if ok {
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
        let input = std::io::Cursor::new(
            b"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 2);
    }
}
