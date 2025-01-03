use anyhow::*;
use std::io::{BufRead, Lines};

struct Rule {
    before: usize,
    after: usize,
}

impl Rule {
    pub fn new(raw_rule: &str) -> Result<Self> {
        let splits: Vec<&str> = raw_rule.trim().split('|').collect();
        ensure!(splits.len() == 2);
        let before = usize::from_str_radix(splits[0], 10)?;
        let after = usize::from_str_radix(splits[1], 10)?;

        Ok(Self { before, after })
    }

    pub fn validate(&self, pages: &[usize]) -> bool {
        let before_index = pages.iter().position(|a| *a == self.before);
        if before_index.is_none() {
            return true;
        }
        let after_index = pages.iter().position(|a| *a == self.after);
        if after_index.is_none() {
            return true;
        }
        before_index.unwrap() < after_index.unwrap()
    }
}

enum State {
    ReadingRules,
    ProcessingPages,
}

fn solve<T>(lines: Lines<T>) -> Result<usize>
where
    T: BufRead,
{
    let mut score = 0;
    let mut rules = vec![];
    let mut state = State::ReadingRules;
    'outer: for line_raw in lines {
        let line = line_raw?;
        let line = line.trim();
        if line.len() == 0 {
            state = State::ProcessingPages;
            continue;
        }
        match state {
            State::ReadingRules => {
                rules.push(Rule::new(line)?);
            }
            State::ProcessingPages => {
                let updated_pages: Vec<usize> = line
                    .split(',')
                    .map(|a| usize::from_str_radix(a.trim(), 10).unwrap())
                    .collect();
                for rule in &rules {
                    if !rule.validate(&updated_pages) {
                        continue 'outer;
                    }
                }
                score += updated_pages[updated_pages.len() / 2];
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
        let input = std::io::Cursor::new(
            b"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 143);
    }
}
