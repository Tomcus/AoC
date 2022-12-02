use std::fs::File;
use std::io::{BufReader, BufRead};

enum RPS {
    Rock,
    Paper,
    Scissors
}

fn normalize_me(me: char) -> RPS {
    match me {
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => {
            panic!("Wrong me character: {}", me);
        }
    }
}

fn normalize_enemy(enemy: char) -> RPS {
    match enemy {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => {
            panic!("Wrong enemy character: {}", enemy);
        }
    }
}

struct ScoreCounter {
    score: u64,
}

impl ScoreCounter {
    fn new() -> Self {
        Self {
            score: 0
        }
    }

    fn score_my_choice(&self, me: RPS) -> u64 {
        match me {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }

    fn score_the_game(&self, me: RPS, enemy: RPS) -> u64 {
        match me {
            RPS::Rock => match enemy {
                RPS::Rock => 3,
                RPS::Paper => 0,
                RPS::Scissors => 6
            },
            RPS::Paper => match enemy {
                RPS::Rock => 6,
                RPS::Paper => 3,
                RPS::Scissors => 0
            },
            RPS::Scissors => match enemy {
                RPS::Rock => 0,
                RPS::Paper => 6,
                RPS::Scissors => 3
            }
        }
    }

    fn process_round(&mut self, me: char, enemy: char) {
        let my_score = self.score_my_choice(normalize_me(me));
        let game_score = self.score_the_game(normalize_me(me), normalize_enemy(enemy));
        println!("{} {} => {} + {}", enemy, me, my_score, game_score);
        self.score += my_score + game_score;
    }
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut counter = ScoreCounter::new();
    for line_raw in BufReader::new(input).lines() {
        let line = line_raw.unwrap();
        if line.len() == 3 {
            let enemy = line.as_bytes()[0] as char;
            let me = line.as_bytes()[2] as char;
            counter.process_round(me, enemy);
        }
    }
    println!("{}", counter.score);
}
