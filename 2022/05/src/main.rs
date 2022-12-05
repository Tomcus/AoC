use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

struct CrateManipulator {
    stacks: Vec<String>
}

impl CrateManipulator {
    fn new() -> Self {
        Self { stacks: vec![] }
    }

    fn load_initial_state(&mut self, reader: &mut impl BufRead) {
        let mut line = String::new();
        loop {
            reader.read_line(&mut line).unwrap();
            if line.len() == 0 || !line.contains('[') {
                break;
            }
            let stack_count = line.len() / 4;
            if self.stacks.len() == 0 {
                self.stacks.resize(stack_count, String::new());   
            }
            for i in 0..stack_count {
                let data = &line[(i * 4)..((i + 1) * 4) - 1];
                if data.contains('[') {
                    self.stacks[i].push(data.chars().nth(1).unwrap());
                }
            }
            line.clear();
        }
        self.stacks = self.stacks.iter().map(|stack| {
            stack.chars().rev().collect()
        }).collect();
    }

    fn process_instructions(&mut self, reader: &mut impl BufRead) {
        let mut line = String::new();
        loop {
            match reader.read_line(&mut line) {
                Err(_) => break,
                Ok(_) => {}
            }
            if line == "\n" {
                continue;
            }
            if line.len() == 0 {
                break;
            }
            let nums: Vec<u64> = line.split(' ').map(|word| {
                if word.ends_with("\n") {
                    u64::from_str(&word[0..word.len() - 1])
                } else {
                    u64::from_str(word)
                }
            }).filter(|result| {
                result.is_ok()
            }).map(|result| {
                result.unwrap()
            }).collect();
            let count = nums[0] as usize;
            let from_index = nums[1] as usize - 1;
            let to_index = nums[2] as usize - 1;
            let mut source = self.stacks[from_index].clone();
            {
                let data_to_transfer = &source[source.len()-count..source.len()];
                self.stacks[to_index].push_str(&data_to_transfer.chars().rev().collect::<String>());
            }
            source.truncate(source.len() - count);
            self.stacks[from_index] = source;
            line.clear();
        }
        self.stacks.iter().for_each(|stack|{
            print!("{}", stack.chars().last().unwrap());
        });
        print!("\n");
    }
}

fn main() {
    let mut crate_manipulator = CrateManipulator::new();
    let input = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(input);

    crate_manipulator.load_initial_state(&mut reader);
    crate_manipulator.process_instructions(&mut reader);
}
