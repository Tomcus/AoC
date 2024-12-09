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

        Ok(Self{
            before,
            after,
        })
    }

    #[inline]
    pub fn before_index(&self, pages: &[usize]) -> Option<usize> {
        pages.iter().position(|a| *a == self.before)
    }

    #[inline]
    pub fn after_index(&self, pages: &[usize]) -> Option<usize> {
        pages.iter().position(|a| *a == self.after)
    }
    
    #[inline]
    pub fn is_valid(&self, pages: &[usize]) -> bool {
        self.before_index(pages).is_some() && self.after_index(pages).is_some()
    }

    #[inline]
    pub fn validate(&self, pages: &[usize]) -> bool {
        self.before_index(pages).unwrap() < self.after_index(pages).unwrap()
    }
}

enum State {
    ReadingRules,
    ProcessingPages
}

fn solve<T>(lines: Lines<T>) -> Result<usize> where T: BufRead {
    let mut score = 0;
    let mut rules = vec![];
    let mut state = State::ReadingRules;
    for line_raw in lines {
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
                let mut updated_pages: Vec<usize> = line.split(',').map(|a| usize::from_str_radix(a.trim(), 10).unwrap()).collect();
                let selected_rules: Vec<&Rule> = rules.iter().filter(|a| a.is_valid(&updated_pages)).collect();
                if selected_rules.iter().any(|a| !a.validate(&updated_pages)) {
                    loop {
                        let mut swaped = false;
                        
                        for rule in &selected_rules {
                            if !rule.validate(&updated_pages) {
                                let before_index = rule.before_index(&updated_pages).unwrap();
                                let after_index = rule.after_index(&updated_pages).unwrap();
                                updated_pages.swap(before_index, after_index);
                                swaped = true;
                            }
                        }
                        
                        if !swaped {
                            break;
                        }
                    }
                    score += updated_pages[updated_pages.len()/2];
                }
            },
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
        let input = std::io::Cursor::new(b"47|53
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
97,13,75,29,47");
        let res = solve(input.lines()).unwrap();
        assert_eq!(res, 123);
    }
}
