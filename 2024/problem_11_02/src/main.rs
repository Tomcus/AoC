use anyhow::*;
use std::io::Read;
use std::collections::HashMap;

type Cache = HashMap<usize, HashMap<usize, usize>>;

fn numb_digits(num: usize) -> u32 {
    num.ilog10() + 1
}

fn solve_rec(stone: usize, iteration: usize, cache: &mut Cache) -> usize {
    if iteration == 0 {
        return 1;
    } else {
        if let Some(iter_cache) = cache.get(&stone) {
            if let Some(result) = iter_cache.get(&iteration) {
                return *result;
            }
        }
        let res = match stone {
            0 => solve_rec(1, iteration - 1, cache),
            x if numb_digits(x) % 2 == 0 => {
                let modulator = 10usize.pow(numb_digits(x) / 2);
                solve_rec(stone / modulator, iteration - 1, cache) + solve_rec(stone % modulator, iteration - 1, cache)
            }
            x => solve_rec(x * 2024, iteration - 1, cache),
        };
        if let Some(iter_cache) = cache.get_mut(&stone) {
            iter_cache.insert(iteration, res);
        } else {
            let mut iter_cache: HashMap<usize, usize> = Default::default();
            iter_cache.insert(iteration, res);
            cache.insert(stone, iter_cache);
        }
        return res;
    }
}

fn solve(input: &str, iterations: usize) -> Result<usize> {
    let stones: Result<Vec<_>, _> = input
        .trim()
        .split(' ')
        .map(|num| usize::from_str_radix(num, 10))
        .collect();

    let initial_stones = stones?;
    let mut score = 0;
    let mut cache: Cache = Default::default();
    for stone in initial_stones {
        println!("Starting stone: {stone}");
        score += solve_rec(stone, iterations, &mut cache);
    }
    Ok(score)
}

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let res = solve(&input, 75)?;
    println!("Result: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let input = "125 17";
        let res = solve(&input, 25).unwrap();
        assert_eq!(res, 55312);
    }
}
