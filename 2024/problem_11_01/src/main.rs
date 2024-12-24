use anyhow::*;
use std::io::Read;

const ITERATIONS: usize = 25;

fn numb_digits(num: usize) -> u32 {
    num.ilog10() + 1
}

fn solve(input: &str) -> Result<usize> {
    let stones: Result<Vec<_>, _> = input
        .trim()
        .split(' ')
        .map(|num| usize::from_str_radix(num, 10))
        .collect();

    let mut stones = stones?;
    let mut new_stones = vec![];
    for _ in 0..ITERATIONS {
        for stone in stones.iter() {
            match stone {
                0 => new_stones.push(1),
                x if numb_digits(*x) % 2 == 0 => {
                    let modulator = 10usize.pow(numb_digits(*x) / 2);
                    new_stones.push(x / modulator);
                    new_stones.push(x % modulator);
                }
                x => new_stones.push(x * 2024),
            }
        }
        std::mem::swap(&mut stones, &mut new_stones);
        new_stones.clear();
    }
    Ok(stones.len())
}

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let res = solve(&input)?;
    println!("Result: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let input = "125 17";
        let res = solve(&input).unwrap();
        assert_eq!(res, 55312);
    }
}
