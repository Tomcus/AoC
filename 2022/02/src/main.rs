use std::fs::File;
use std::io::{BufReader, BufRead};

enum RPS {
    Rock,
    Paper,
    Scissors
}

enum GameState {
    Victory = 6,
    Draw = 3,
    Loss = 0
}

fn normalize_me(me: char) -> GameState {
    match me {
        'X' => GameState::Loss,
        'Y' => GameState::Draw,
        'Z' => GameState::Victory,
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

    fn score_my_choice(&self, me: &RPS) -> u64 {
        match me {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }

    fn get_my_choice(&self, me: &GameState, enemy: &RPS) -> RPS {
        match enemy {
            RPS::Rock => match me {
                GameState::Draw => RPS::Rock,
                GameState::Loss => RPS::Scissors,
                GameState::Victory => RPS::Paper
            },
            RPS::Paper => match me {
                GameState::Loss => RPS::Rock,
                GameState::Draw => RPS::Paper,
                GameState::Victory => RPS::Scissors
            },
            RPS::Scissors => match me {
                GameState::Loss => RPS::Paper,
                GameState::Draw => RPS::Scissors,
                GameState::Victory => RPS::Rock
            }
        }
    }

    fn process_round(&mut self, me: char, enemy: char) {
        let game_end = normalize_me(me);
        let my_choice = self.get_my_choice(&game_end, &normalize_enemy(enemy));
        let my_score = self.score_my_choice(&my_choice);
        
        println!("{} {} => {} + {}", enemy, me, my_score, game_end as u64);
        self.score += my_score + game_end as u64;
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
