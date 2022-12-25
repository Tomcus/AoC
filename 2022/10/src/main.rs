use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;

struct CPU {
    counter: i64,
    x: i64,
}

enum Instruction {
    Noop,
    Addx(i64)
}

impl CPU {
    fn new() -> Self {
        Self {
            counter: 0,
            x: 1,
        }
    }

    fn increment_counter(&mut self) {
        let counter_pixel_pos = self.counter % 40;
        if self.x - 1 <= counter_pixel_pos && self.x + 1 >= counter_pixel_pos {
            print!("#");
        } else {
            print!(".");
        }
        self.counter += 1;
        if self.counter > 0 && self.counter % 40 == 0 {
            print!("\n");
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => { self.increment_counter(); }
            Instruction::Addx(value) => {
                self.increment_counter();
                self.increment_counter();
                self.x += value;
            }
        }
    }
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut cpu = CPU::new();
    for line_raw in BufReader::new(input).lines() {
        let line = line_raw.unwrap();
        let splits: Vec<&str> = line.split(' ').collect();
        match splits[0] {
            "noop" => {
                cpu.process_instruction(Instruction::Noop);
            },
            "addx" => {
                let increment = i64::from_str(splits[1]).unwrap();
                cpu.process_instruction(Instruction::Addx(increment));
            }
            _ => { panic!("Unknown instruction: {}", splits[0]); }
        }
    }
}
