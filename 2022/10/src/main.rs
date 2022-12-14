use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;

struct CPU {
    counter: i64,
    x: i64,
    signal: i64,
}

enum Instruction {
    Noop,
    Addx(i64)
}

impl CPU {
    fn new() -> Self {
        Self {
            counter: 1,
            x: 1,
            signal: 0,
        }
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
        if self.counter <= 220 && (self.counter - 20) % 40 == 0 {
            self.signal += self.x * self.counter;
            println!("[{},{}]: {} + {}", self.counter, self.x, self.signal - (self.x * self.counter), self.x * self.counter);
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => { self.increment_counter(); }
            Instruction::Addx(value) => {
                self.increment_counter();
                self.x += value;
                self.increment_counter();
                println!("[{},{}]", self.counter, self.x);
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
        println!("{}", line);
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
    println!("Signal: {}", cpu.signal);
}
